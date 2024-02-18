use slint::{Model, ModelExt, SharedString, StandardListViewItem};
use std::cell::RefCell;
use std::rc::Rc;

slint::slint!(import { MainWindow } from "ui/crud.slint";);

use crate::db::{open_db};
use slintext::sqlmdl::SqliteModel;
use slintext::sqltmdl::SqliteStandardTableModel;
use crate::pets::*;

impl From<Pet> for PetUi {
    fn from(value: Pet) -> Self {
        PetUi {
            id: value.id.into(),
            name: value.name.into(),
        }
    }
}

pub fn main() {
    let main_window = MainWindow::new().unwrap();

    let prefix = Rc::new(RefCell::new(SharedString::from("")));

    let shop = open_db().expect("TODO: move to main HANDLEERROR could not create model file");
    let shop = Rc::new(RefCell::new(shop));

     let report_error = {
        let main_window_weak = main_window.as_weak();
        move |msg| {
            let main_window = main_window_weak.unwrap();
            main_window.set_last_error(SharedString::from(msg))
        }
    };

    let model = Rc::new(SqliteModel::<Pet,_>::new(shop.clone(), PET_SHOP_TABLE, report_error.clone()));
    let model_mapped = Rc::new(
        model
            .clone()
            .map(|n| StandardListViewItem::from(slint::format!("{}", n.name)))
    );

    let filtered_model = Rc::new(
        model_mapped.clone()
            .filter({
                let prefix = prefix.clone();
                move |e| e.text.starts_with(prefix.borrow().as_str())}),
    );

    main_window.set_names_list(filtered_model.clone().into());

    /* Callbacks */

    main_window.on_currentItemChanged({
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        move |idx| {
            let main_window = main_window_weak.unwrap();
	    let row = filtered_model.unfiltered_row(idx as usize);
	    if let Some(pet) = model.row_data(row) {
		main_window.invoke_set_current_pet(PetUi::from(pet));
	    }
        }});

    main_window.on_createClicked({
        let report_error =report_error.clone();
        let model = model.clone();
        move |new| {
            let mut entry = Pet::new();
	    entry.name = new.name.into();
            match model.add(entry) { Ok(_) => true, Err(err) => { report_error(format!("Create Failed\n{err:}")); false } }
        }});

    main_window.on_updateClicked({
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        move |update| {
            let main_window = main_window_weak.unwrap();
	    let row = filtered_model.unfiltered_row(main_window.get_current_item() as usize);
	    match model.row_data(row) {
		Some(mut entry) => {
		    entry.name = update.name.into();
		    model.set_row_data(row, entry);
		}
		None => { println!("TODO signal entry not found!") }
	    };
        }});

    main_window.on_deleteClicked({
        let model = model.clone();
        let report_error = report_error.clone();
        move |removed| {
            match model.del_value(Pet{id: removed.id.into(), ..Default::default()}) {
                Ok(_) => true,
                Err(err) => { report_error(format!("{}", err)); false }
            }
        }});

    main_window.on_prefixEdited({
        let filtered_model = filtered_model.clone();
        move |str| {
            prefix.replace(str.into());
            filtered_model.reset();
        }});

    let test_model: Rc<RefCell<Option<Rc<SqliteStandardTableModel<_>>>>> = Rc::new(RefCell::new(None));

    main_window.on_test_query({
        let main_window_weak = main_window.as_weak();
        let report_error = report_error.clone();
        let shop = shop.clone();
        let test_model = test_model.clone();
        move |query, update| {
            match slintext::sqltmdl::sqlite_standard_table_model_from(shop.clone(), query.into(), update.into(), report_error.clone()) {
                Ok((headings, rows, model)) => {
                    test_model.replace(Some(model));
                    let main_window = main_window_weak.unwrap();
                    main_window.invoke_test_result(headings, rows);
                }
                Err(err) => report_error(format!("In Query handling:\n{err:}"))
            }
        }});

    main_window.on_test_execute({
        let report_error = report_error.clone();
        let model = test_model.clone();
        move |command: SharedString, line| {
            match model.borrow().as_ref() {
                None => {report_error("No Query defined.".to_string()); false}
                Some(model) => {
                    match model.execute(command.into(), line) {
                        Ok(_) => true,
                        Err(err) => {report_error(format!("In Query handling:\n{err:}")); false}
                    }
                }
            }
        }});

    /* Finally, once everything is set up. */
    main_window.run().unwrap();
}
