use serde_rusqlite::*;
use rusqlite::{Connection, Result};

type PetId = String;

pub // why?
struct SqlIdTable<'a> {
    pub initially: &'a [&'a str],
    pub create: &'a str,
    pub read: &'a str,
    pub all: &'a str,
    pub update: &'a str,
    pub delete: &'a str,
    pub delkey: &'a [&'a str],
}

impl <'a>SqlIdTable<'a> {
    pub fn new(
        initially: &'a [&'a str],
        create: &'a str,
        read: &'a str,
        all: &'a str,
        update: &'a str,
        delete: &'a str,
        delkey: &'a [&'a str],
    ) -> Self {
        Self{
            initially,
            create,
            read,
            all,
            update,
            delete,
            delkey,
        }
    }
}
use slint::{Model, ModelNotify, ModelTracker};
use std::cell::RefCell;
use std::rc::Rc;

/// A [`Model`] backed by a  sqlite connection `Vec<T>`
//#[derive(Default)]
pub struct SqliteModel<'a, T> {
    array: RefCell<Vec<T>>, // cache the current backng store
    connection: Rc<RefCell<Connection>>,
    sql: SqlIdTable<'a>,
    notify: ModelNotify,
}

impl <'a, T: 'static + for<'de> serde::Deserialize<'de> + serde::Serialize > SqliteModel<'a, T> {

    pub fn init(&self) /* ->Result<(), rusqlite::Error> */ {
        let conn = self.connection.borrow();
        for step in self.sql.initially {
            match conn.execute(step, () ) {
	        Ok(_) => {} // Ok(()),
	        Err(err) => {println!("Err {err:?}"); panic!("No Shit Sherlock!");}
	    }
        }
        self.reset();
        /* Ok(()) */
    }

    pub fn new(connection: Rc<RefCell<Connection>>, sql: SqlIdTable<'static>) -> Self {
        let array: Vec<T> = Vec::new();
        let result = Self {
            array: RefCell::new(array),
            connection: connection,
            sql: sql,
            notify: Default::default()
        };
        result.init();
        result
    }

    fn add_obj(&self, obj: T) -> Result<(), rusqlite::Error> {
        let conn = self.connection.borrow();
	let mut stmt = match conn.prepare(self.sql.create) {
	    Ok(stmt) => stmt,
	    Err(err) => {println!("Err {err:?}"); return Err(err)}
	};
        match to_params_named(&obj) {
	    Ok(params) => {
		// let columns = columns_from_statement(&stmt);
		match stmt.execute(
		    params.to_slice().as_slice()) {
                    Ok(_) => Ok(()),
                    Err(x) => {println!("Err {x:?}");Err(x)}
		}
	    }
	    Err(err) => {println!("Err {err:?}"); Ok(()) /* FIXME This is lying */}
	}
    }
    fn update_obj(&self, obj: T) -> Result<(), rusqlite::Error> {
        let conn = self.connection.borrow();
	let mut stmt = match conn.prepare(self.sql.update) {
	    Ok(stmt) => stmt,
	    Err(err) => {println!("Err {err:?}"); return Err(err)}
	};
        match to_params_named(&obj) {
	    Ok(params) => {
		// let columns = columns_from_statement(&stmt);
		match stmt.execute(
		    params.to_slice().as_slice()) {
                    Ok(_) => Ok(()),
                    Err(x) => {println!("Err {x:?}");Err(x)}
		}
	    }
	    Err(err) => {println!("Err {err:?}"); Ok(()) /* FIXME This is lying */}
	}
    }
    fn del_key(&self, name: PetId) -> Result<()> {
        match self.connection.borrow().execute(self.sql.delete, [&name]) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
    fn get(&self, id: String) -> Option<T> {
        let conn = self.connection.borrow();
	let mut stmt = match conn.prepare(self.sql.read) {
	    Ok(stmt) => stmt,
	    Err(_) => return None
	};
	let mut rows = stmt.query_and_then([id], from_row::<T>).unwrap(); // prove: hopefully never happens
	match rows.next() {
	    Some(row) => Some(row.expect("unexpected ERROR")),
	    None => None
	}
    }
    fn all(&self) -> Result<Vec<T>> {
        let conn = self.connection.borrow();
        let stmt = conn.prepare(self.sql.all);
        match stmt {
            Ok(mut stmt) => {
                let rows = from_rows(stmt.query([]).unwrap()); // prove: hopefully never happens
                let mut all: Vec<T> = vec![];
                for p in rows {
                    all.push(p.unwrap()) // OOM
                }
                Ok(all)
            }
            Err(x) => Err(x),
        }
    }

    pub fn reset(&self) { // try to keep this NOT public for performance
        match self.all() {
            Ok(rows) => {
                *self.array.borrow_mut() = rows.into();
                self.notify.reset();
            }
	    Err(err) => {println!("Err in reset {err:?}");}
        }
    }

    /// Add a row to the model
    pub fn add(&self, value: T) {
        match self.add_obj(value) {
            Ok(_) => { self.reset(); }
	    Err(err) => {println!("Err in add {err:?}");}
        }
    }

    /// Remove the row from the model
    fn delete_value(&self, value: &T) -> Result<usize, serde_rusqlite::Error> {
/*
        match self.connection.borrow().execute(self.sql.delete, [&name]) {
            Ok(_) => { self.reset(); }
	    Err(err) => {println!("Err in del {err:?}");}
        }
*/
        let conn = self.connection.borrow();
	let mut stmt = conn.prepare(self.sql.delete)?;
        // let columns = stmt.column_names();
        match to_params_named_with_fields(&value, self.sql.delkey) {
	    Ok(params) => {
		// let columns = columns_from_statement(&stmt);
                match stmt.execute(params.to_slice().as_slice()) {
                    Ok(n) => { Ok(n) }
                    Err(x) => {println!("Err {x:?}"); Err(serde_rusqlite::Error::Rusqlite(x))}
		}
	    }
	    Err(err) => {println!("Err {err:?}"); Err(err)}
	}
    }
    /// Remove the row from the model
    pub fn del_value(&self, value: &T) {
        self.delete_value(value);
        self.reset();
    }

    /// Remove the row from the model
    pub fn del_row(&self, index: usize) {
        let array = self.array.borrow();
	if let Some(value) = array.get(index) {
            match self.delete_value(value) {
                Ok(n) => {
                    drop(array);
                    // if n > 0 { self.notify.row_removed(index, 1); }
                    self.reset();
                }
	        Err(err) => {eprintln!("Err {err:?}");}
            }
        }
    }
/*
    /// Replace inner Vec with new data
    pub fn set_vec(&self, new: impl Into<Vec<T>>) {
        *self.array.borrow_mut() = new.into();
        self.notify.reset();
    }

    /// Extend the model with the content of the iterator
    ///
    /// Similar to [`Vec::extend`]
    pub fn extend<I: IntoIterator<Item = T>>(&self, iter: I) {
        let mut array = self.array.borrow_mut();
        let old_idx = array.len();
        array.extend(iter);
        let count = array.len() - old_idx;
        drop(array);
        self.notify.row_added(old_idx, count);
    }
*/

}

/*
impl <'a, T>From<Connection> for SqliteModel<'a, T> {
    fn from(connection: Connection, sql: SqlIdTable<'a>) -> Self {
        let array: Vec<T> = Vec::new();
        SqliteModel {
            array: RefCell::new(array),
            connection: RefCell::new(connection),
            sql: sql,
            notify: Default::default()
        }
    }
}
 */

impl<T: Clone + 'static + for<'de> serde::Deserialize<'de> + serde::Serialize > Model for SqliteModel<'static, T> {
    type Data = T;

    fn row_count(&self) -> usize {
        self.array.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.array.borrow().get(row).cloned()
    }

    fn set_row_data(&self, row: usize, data: Self::Data) {
        match self.update_obj(data.clone()) {
            Ok(_) => {
                *self.array.borrow_mut().get_mut(row).unwrap()=data.into();
                self.notify.row_changed(row)
            }
	    Err(err) => {println!("Err in set_row_data {err:?}");}
        }
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}
