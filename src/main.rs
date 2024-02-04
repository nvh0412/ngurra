mod models;
mod db;

use db::init_db;
pub use models::flash_card::FlashCard;
pub use models::deck::Deck;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("anki-rs.db")?;

    init_db(&conn)
}
