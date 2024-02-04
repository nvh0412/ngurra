use rusqlite::{Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deck (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            creation_time DateTime NOT NULL,
            last_studied_date DateTime NOT NULL
        )",
        []
    )?;

    Ok(())
}
