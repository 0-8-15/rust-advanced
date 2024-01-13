use rusqlite::{self, Connection};

pub fn open_db() -> Result<Connection, rusqlite::Error>{
    let conn = Connection::open("pets.db")?;
    Ok(conn)
}
