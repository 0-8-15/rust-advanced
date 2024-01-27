use rusqlite::{self, Connection};

use super::{DATABASE_FILE_NAME};

pub fn open_db() -> Result<Connection, rusqlite::Error>{
    let conn = Connection::open(DATABASE_FILE_NAME)?;
    Ok(conn)
}
