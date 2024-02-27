use rusqlite::{params_from_iter, Connection, Result};

struct SqlIdTable {
    pub all: String,
    pub update: String,
}

impl SqlIdTable {
    pub fn new(all: String, update: String) -> Self {
        Self {
            all: all.to_string(),
            update: update.to_string(),
        }
    }
}
use slint::{Model, ModelNotify, ModelTracker};
use std::cell::RefCell;
use std::rc::Rc;

/// A [`Model`] backed by a  sqlite connection `Vec<T>`
//#[derive(Default)]
pub struct SqliteStandardTableModel<R>
where
    R: 'static + Fn(String),
{
    array: RefCell<Vec<ModelRc<StandardListViewItem>>>, // cache the current backing store
    connection: Rc<RefCell<Connection>>,
    sql: Box<SqlIdTable>,
    notify: ModelNotify,
    report_error: R,
}

impl<R> SqliteStandardTableModel<R>
where
    R: 'static + Fn(String),
{
    fn new(
        connection: Rc<RefCell<Connection>>,
        array: Vec<ModelRc<StandardListViewItem>>,
        sql: SqlIdTable,
        report_error: R,
    ) -> Self {
        Self {
            array: RefCell::new(array),
            connection: connection,
            sql: Box::new(sql),
            notify: Default::default(),
            report_error,
        }
    }

    /// execute SQL statement
    pub fn execute(&self, sql: String, obj: ModelRc<StandardListViewItem>) -> Result<usize> {
        let conn = self.connection.borrow();
        let mut stmt = conn.prepare(sql.as_str())?;
        let mut cols = vec![]; // FIXME: avoid allocation
        for i in 0..obj.row_count() {
            cols.push(obj.row_data(i).unwrap().text.to_string())
        }
        let result = stmt.execute(params_from_iter(cols));
        self.reset();
        result
    }
    fn update_obj(&self, obj: ModelRc<StandardListViewItem>) -> Result<usize> {
        let conn = self.connection.borrow();
        let mut stmt = match conn.prepare(self.sql.update.as_str()) {
            Ok(stmt) => stmt,
            Err(err) => return Err(err),
        };
        let mut cols = vec![]; // FIXME: avoid allocation
        for i in 0..obj.row_count() {
            cols.push(obj.row_data(i).unwrap().text.to_string())
        }
        stmt.execute(params_from_iter(cols))
    }
    fn column_titles(&self) -> Result<ModelRc<TableColumn>> {
        let conn = self.connection.borrow();
        let stmt = conn.prepare(self.sql.all.as_str())?;
        let headings = stmt
            .column_names()
            .into_iter()
            .map(|n| {
                let mut x: slint::TableColumn = Default::default();
                x.title = slint::format!("{}", n).into();
                x
            })
            .collect::<Vec<_>>();
        Ok(ModelRc::new(VecModel::from(headings)))
    }
    fn all(&self) -> Result<Vec<ModelRc<StandardListViewItem>>> {
        let conn = self.connection.borrow();
        let mut stmt = conn.prepare(self.sql.all.as_str())?;

        let colco = stmt.column_count();
        let rows = stmt.query_map([], |row| {
            let mut v: Vec<StandardListViewItem> = Vec::new();
            for j in 0..colco {
                let c = match row.get_ref_unwrap(j) {
                    ValueRef::Null => slint::format!(""),
                    ValueRef::Integer(n) => slint::format!("{n:}"),
                    ValueRef::Real(n) => slint::format!("{n:}"),
                    ValueRef::Text(s) => slint::format!("{}", ValueRef::Text(s).as_str()?),
                    ValueRef::Blob(_) => slint::format!("BLOB"),
                };
                v.push(StandardListViewItem::from(c))
            }
            Ok(ModelRc::new(VecModel::from(v)))
        })?;

        let mut data = Vec::new();
        for row in rows {
            data.push(row?);
        }

        Ok(data)
    }

    pub fn reset(&self) {
        // try to keep this NOT public
        match self.all() {
            Ok(rows) => {
                self.array.replace(rows);
                self.notify.reset();
            }
            Err(err) => {
                println!("Err in reset {err:?}");
            }
        }
    }
}

impl<R> Model for SqliteStandardTableModel<R>
where
    R: 'static + Fn(String),
{
    type Data = ModelRc<StandardListViewItem>;

    fn row_count(&self) -> usize {
        self.array.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.array.borrow().get(row).cloned()
    }

    fn set_row_data(&self, row: usize, data: Self::Data) {
        match self.update_obj(data.clone()) {
            Ok(_) => {
                *self.array.borrow_mut().get_mut(row).unwrap() = data.into();
                // NOTE: This is not always correct: self.notify.row_changed(row);
                self.reset()
            }
            Err(err) => {
                (self.report_error)(format!("{err:?}"));
                self.reset();
            }
        }
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

/* **************************************************************************** */
use rusqlite::types::ValueRef;
use slint::{ModelRc, StandardListViewItem, TableColumn, VecModel};

pub fn standard_table_model_from(
    conn: &Connection,
    query: &str,
) -> Result<(ModelRc<TableColumn>, ModelRc<ModelRc<StandardListViewItem>>)> {
    let mut stmt = conn.prepare(query)?;
    let headings = stmt
        .column_names()
        .into_iter()
        .map(|n| {
            let mut x: slint::TableColumn = Default::default();
            x.title = slint::format!("{}", n).into();
            x
        })
        .collect::<Vec<_>>();

    let colco = stmt.column_count();
    let rows = stmt.query_map([], |row| {
        let mut v: Vec<StandardListViewItem> = Vec::new();
        for j in 0..colco {
            let c = match row.get_ref_unwrap(j) {
                ValueRef::Null => slint::format!(""),
                ValueRef::Integer(n) => slint::format!("{n:}"),
                ValueRef::Real(n) => slint::format!("{n:}"),
                ValueRef::Text(s) => slint::format!("{}", ValueRef::Text(s).as_str()?),
                ValueRef::Blob(_) => slint::format!("BLOB"),
            };
            v.push(StandardListViewItem::from(c))
        }
        Ok(ModelRc::new(VecModel::from(v)))
    })?;

    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }

    Ok((
        ModelRc::new(VecModel::from(headings)),
        ModelRc::new(VecModel::from(data)),
    ))
}

pub fn sqlite_standard_table_model_from<R: std::ops::Fn(std::string::String) + 'static>(
    conn: Rc<RefCell<Connection>>,
    query: String,
    update: String, // FIXME: add Parameter for parameters!!!
    report_error: R,
) -> Result<(
    ModelRc<TableColumn>,
    ModelRc<ModelRc<StandardListViewItem>>,
    Rc<SqliteStandardTableModel<R>>,
)> {
    let conn_b = conn.borrow();
    let mut stmt = conn_b.prepare(query.as_str())?;
    let headings = stmt
        .column_names()
        .into_iter()
        .map(|n| {
            let mut x: slint::TableColumn = Default::default();
            x.title = slint::format!("{}", n).into();
            x
        })
        .collect::<Vec<_>>();

    let colco = stmt.column_count();
    let rows = stmt.query_map([], |row| {
        let mut v: Vec<StandardListViewItem> = Vec::new();
        for j in 0..colco {
            let c = match row.get_ref_unwrap(j) {
                ValueRef::Null => slint::format!(""),
                ValueRef::Integer(n) => slint::format!("{n:}"),
                ValueRef::Real(n) => slint::format!("{n:}"),
                ValueRef::Text(s) => slint::format!("{}", ValueRef::Text(s).as_str()?),
                ValueRef::Blob(_) => slint::format!("BLOB"),
            };
            v.push(StandardListViewItem::from(c))
        }
        Ok(ModelRc::new(VecModel::from(v)))
    })?;

    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }

    drop(stmt);
    drop(conn_b);

    let commands = SqlIdTable::new(query, update);
    let model = SqliteStandardTableModel::new(conn.clone(), data, commands, report_error);
    let model_rc = Rc::new(model);

    Ok((
        ModelRc::new(VecModel::from(headings)),
        ModelRc::from(model_rc.clone()),
        model_rc,
    ))
}
