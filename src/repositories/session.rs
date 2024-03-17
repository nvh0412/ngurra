use rusqlite::{Connection, Result};

pub struct Session;

impl Session {
    pub fn get_creation_stamp(conn: &Connection) -> Result<i64> {
        conn.prepare_cached("select creation_stamp from sessions")?
            .query_row([], |row| row.get(0))
            .map_err(Into::into)
    }
}
