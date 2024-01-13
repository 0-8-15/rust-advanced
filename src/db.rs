use rusqlite::{self, Connection};

pub fn open_db() -> Result<Connection, rusqlite::Error>{
    let conn = Connection::open("pets.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS pet (
        id    INTEGER PRIMARY KEY,
        name  TEXT NOT NULL,
        photo  BLOB
        )",
    (), // empty list of parameters.
    )?;
    Ok(conn)
}
