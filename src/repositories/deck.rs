use std::time::SystemTime;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use time::OffsetDateTime;

use crate::FlashCard;

/// Represents a deck in the Anki application.
///
/// A deck has an optional ID, a name, creation time, and last studied date.
/// Decks can be created, loaded, and saved to a database using the provided methods.
///
/// # Examples
///
/// Creating a new deck:
///
/// ```
/// let deck = Deck::new("My Deck");
/// ```
///
/// Loading a deck from the database:
///
/// ```
/// let deck = Deck::load(1);
/// ```
///
/// Saving a deck to the database:
///
/// ```
/// let mut deck = Deck::new("My Deck");
/// deck.save(&connection);
/// ```
pub struct Deck {
    pub id: Option<i32>,
    pub name: String,
    creation_time: SystemTime,
    last_studied_date: SystemTime,
    pub cards: Vec<FlashCard>,
}

impl Deck {
    /// Creates a new deck with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the deck.
    ///
    /// # Returns
    ///
    /// The newly created deck.
    pub fn new(name: &str) -> Deck {
        Deck {
            id: None,
            name: name.to_string(),
            creation_time: SystemTime::now(),
            last_studied_date: SystemTime::now(),
            cards: Vec::new(),
        }
    }

    /// Retrieves all decks from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of all decks, or an error if the operation fails.
    pub fn get_all_decks(conn: &Connection) -> Result<Vec<Deck>> {
        let mut stmt = conn.prepare("SELECT id, name, creation_time, last_studied_date FROM decks")?;

        let decks = stmt.query_map([], |row| {
            let creation_time: String = row.get(2)?;
            let last_studied_date: String = row.get(3)?;

            let creation_time = DateTime::parse_from_rfc3339(&creation_time).unwrap().naive_utc();
            let last_studied_date = DateTime::parse_from_rfc3339(&last_studied_date).unwrap().naive_utc();

            let creation_time = OffsetDateTime::from_unix_timestamp(creation_time.timestamp()).unwrap().into();
            let last_studied_date = OffsetDateTime::from_unix_timestamp(last_studied_date.timestamp()).unwrap().into();

            Ok(Deck {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                creation_time,
                last_studied_date,
                cards: Vec::new(),
            })
        })?;

        let mut result = Vec::new();
        for deck in decks {
            result.push(deck.unwrap());
        }

        Ok(result)
    }

    /// Loads a deck from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the deck to load.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `Result` containing the loaded deck, or an error if the operation fails.
    pub fn load(id: i32, conn: &Connection) -> Result<Deck> {
        let mut stmt = conn.prepare("SELECT id, name, creation_time, last_studied_date FROM decks WHERE id = ?")?;

        let deck = stmt.query_row(&[&id], |row| {
            let creation_time: String = row.get(2)?;
            let last_studied_date: String = row.get(3)?;

            let creation_time = DateTime::parse_from_rfc3339(&creation_time).unwrap().naive_utc();
            let last_studied_date = DateTime::parse_from_rfc3339(&last_studied_date).unwrap().naive_utc();

            let creation_time = OffsetDateTime::from_unix_timestamp(creation_time.timestamp()).unwrap().into();
            let last_studied_date = OffsetDateTime::from_unix_timestamp(last_studied_date.timestamp()).unwrap().into();

            Ok(Deck {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                creation_time,
                last_studied_date,
                cards: Vec::new(),
            })
        })?;

        Ok(deck)
    }

    /// Saves the deck to the database.
    ///
    /// If the deck already has an ID, it will be updated in the database.
    /// Otherwise, a new record will be inserted.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn save(&mut self, conn: &Connection) -> Result<()> {
        let last_studied_date = DateTime::<Utc>::from(self.last_studied_date);

        match self.id {
            Some(id) => {
                conn.execute(
                    "UPDATE decks SET name = ?, last_studied_date = ? WHERE id = ?",
                    &[&self.name, &last_studied_date.to_rfc3339(), &id.to_string()],
                )?;
            }
            None => {
                conn.execute(
                    "INSERT INTO decks (name, creation_time, last_studied_date) VALUES (?, ?, ?)",
                    &[
                        &self.name,
                        &DateTime::<Utc>::from(self.creation_time).to_rfc3339(),
                        &last_studied_date.to_rfc3339(),
                    ],
                )?;

                let id = conn.last_insert_rowid();
                self.id = Some(id as i32);
            }
        }

        Ok(())
    }

    /// Deletes the deck from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the deck to load.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn delete(id: i32, conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM decks WHERE id = ?", &[&id])?;

        Ok(())
    }

    pub fn get_card_stats(&self) -> (i32, i32, i32) {
        let mut due = 0;
        let mut new = 0;
        let mut learning = 0;

        for card in &self.cards {
            if card.is_due() {
                due += 1;
            } else if card.is_learning() {
                learning += 1;
            } else {
                new += 1;
            }
        }

        (due, learning, new)
    }
}

#[cfg(test)]
mod test {
    use crate::db::init_db;

    use super::*;

    #[test]
    fn get_all_decks() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let decks = Deck::get_all_decks(&conn).unwrap();

        assert_eq!(decks.len(), 1);
    }

    #[test]
    fn load() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let loaded_deck = Deck::load(deck.id.unwrap(), &conn).unwrap();

        assert_eq!(deck.name, loaded_deck.name);
    }

    #[test]
    fn save() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        assert!(deck.id.is_some());
    }

    #[test]
    fn delete() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        Deck::delete(deck.id.unwrap(), &conn).unwrap();

        let loaded_deck = Deck::load(deck.id.unwrap(), &conn);

        assert!(loaded_deck.is_err());
        assert_eq!(loaded_deck.err().unwrap().to_string(), "Query returned no rows");
    }
}
