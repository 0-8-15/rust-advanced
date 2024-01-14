use slint::{Model, ModelExt, SharedString, StandardListViewItem, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

slint::slint!(import { MainWindow } from "ui/crud.slint";);

use crate::db::{open_db};
use crate::pets::*;

pub fn main() {
    let main_window = MainWindow::new().unwrap();

    let prefix = Rc::new(RefCell::new(SharedString::from("")));
    let prefix_for_wrapper = prefix.clone();

    let shop = open_db().expect("TODO: move to main HADLEERROR could not create model file");
    let _ = shop.init_db(); // TODO: HADLEERROR
    let all = shop.all_pets().expect("TODO: HADLEERROR model initialization failed");

    let model = Rc::new(VecModel::from(all));

    let filtered_model = Rc::new(
        model
            .clone()
            .map(|n| StandardListViewItem::from(slint::format!("{}", n.name)))
            .filter(move |e| e.text.starts_with(prefix_for_wrapper.borrow().as_str())),
    );

    main_window.set_names_list(filtered_model.clone().into());

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        main_window.on_createClicked(move || {
            let main_window = main_window_weak.unwrap();
            let mut new_entry = Pet::new();
	    new_entry.name = format!("{} {}", main_window.get_name(), main_window.get_surname());
	    let shop = open_db().expect("TODO HADLEERROR model file missing");
	    match shop.add_pet(new_entry.clone()) {
		Ok(_) => model.push(new_entry),
		Err(x) => {println!("TODO HADLEERROR")}
	    };
        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        main_window.on_updateClicked(move || {
            let main_window = main_window_weak.unwrap();

	    let shop = open_db().expect("TODO: HADLEERROR: failed to open model file");
            let updated_entry = shop.get_pet(main_window.get_petid().to_string());
	    match updated_entry {
		Some(mut updated_entry) => {
		    updated_entry.name = format!("{}{}", main_window.get_surname().to_string(), main_window.get_surname());
		    match shop.add_pet(updated_entry.clone()) {
			Ok(_) => {
			    let row = filtered_model.unfiltered_row(main_window.get_current_item() as usize);
			    model.set_row_data(row, updated_entry);
			},
			Err(_) => { println!("TODO HANDLEERROR: failed to update entry {:}", updated_entry.id) }
		    }
		}
		None => { println!("TODO signal entry not found!") }
	    };

        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        main_window.on_deleteClicked(move || {
            let main_window = main_window_weak.unwrap();

            let index = filtered_model.unfiltered_row(main_window.get_current_item() as usize);
            model.remove(index);
        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let filtered_model = filtered_model.clone();
        main_window.on_prefixEdited(move || {
            let main_window = main_window_weak.unwrap();
            *prefix.borrow_mut() = main_window.get_prefix();
            filtered_model.reset();
        });
    }

    main_window.run().unwrap();
}
