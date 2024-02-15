use rusqlite::{Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS decks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            creation_time DateTime NOT NULL,
            last_studied_date DateTime NOT NULL
        )",
        []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            id INTEGER PRIMARY KEY,
            question TEXT NOT NULL,
            answer TEXT NOT NULL,
            creation_time DateTime NOT NULL,
            last_studied_time DateTime,
            deck_id INTEGER NOT NULL,
            ef FLOAT NOT NULL,
            interval INTEGER NOT NULL,
            FOREIGN KEY(deck_id) REFERENCES decks(id)
        )", []
    )?;

    Ok(())
}
