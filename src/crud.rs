use slint::{Model, ModelExt, SharedString, StandardListViewItem};
use std::cell::RefCell;
use std::rc::Rc;

slint::slint!(import { MainWindow } from "ui/crud.slint";);

use crate::db::{open_db};
use crate::sqlmdl::SqliteModel;
use crate::pets::*;

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
		main_window.set_name(pet.name.into());
	    }
        }});

    main_window.on_createClicked({
        let main_window_weak = main_window.as_weak();
        let report_error =report_error.clone();
        let model = model.clone();
        move || {
            let main_window = main_window_weak.unwrap();
            let mut entry = Pet::new();
	    entry.name = main_window.get_name().to_string();
            model.add(entry).unwrap_or_else(|err| report_error(format!("{err:}")));
        }});

    main_window.on_updateClicked({
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        move || {
            let main_window = main_window_weak.unwrap();
	    let row = filtered_model.unfiltered_row(main_window.get_current_item() as usize);
	    match model.row_data(row) {
		Some(mut entry) => {
		    entry.name = main_window.get_name().to_string();
		    model.set_row_data(row, entry);
		}
		None => { println!("TODO signal entry not found!") }
	    };
        }});

    main_window.on_deleteClicked({
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        move || {
            let main_window = main_window_weak.unwrap();
            let index = filtered_model.unfiltered_row(main_window.get_current_item() as usize);
            model.del_row(index);
        }});

    main_window.on_prefixEdited({
        let main_window_weak = main_window.as_weak();
        let filtered_model = filtered_model.clone();
        move || {
            let main_window = main_window_weak.unwrap();
            *prefix.borrow_mut() = main_window.get_prefix();
            filtered_model.reset();
        }});

    /* Finally, once everything is set up. */
    main_window.run().unwrap();
}
