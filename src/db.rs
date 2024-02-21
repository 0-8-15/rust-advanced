use rusqlite::{self, Connection, functions::FunctionFlags};
use uuid::{Uuid};

use super::{DATABASE_FILE_NAME};

pub fn open_db() -> Result<Connection, rusqlite::Error>{
    let conn = Connection::open(DATABASE_FILE_NAME)?;
    conn.create_scalar_function(
        "uuid",
        0,
        FunctionFlags::SQLITE_INNOCUOUS,
        |_ctx| {
            let value = Uuid::new_v4().to_string();
            Ok(value)
        },
    )?;
    Ok(conn)
}
