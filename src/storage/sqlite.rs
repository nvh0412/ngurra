use std::path::Path;

use rusqlite::Connection;

use crate::errors::Result;

pub struct SqliteStorage {
    pub conn: Connection,
}

pub fn open_or_create_db(path: &Path) -> Connection {
    let db = Connection::open(path).unwrap_or_else(|e| {
        println!("Error opening db: {:?}", e);
        Connection::open_in_memory().unwrap()
    });

    db.busy_timeout(std::time::Duration::from_secs(0));
    db.set_prepared_statement_cache_capacity(50);

    db
}

impl SqliteStorage {
    pub fn open_or_create(path: &Path) -> Result<Self> {
        let db = open_or_create_db(path);

        Ok(Self { conn: db })
    }
}
