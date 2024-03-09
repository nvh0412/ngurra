use std::{collections::HashMap, time::SystemTime};

use chrono::{DateTime, Utc};
use rusqlite::{named_params, Connection, Result};
use time::OffsetDateTime;

use crate::FlashCard;

use super::flash_card::CardQueue;

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
    pub id: Option<u32>,
    pub name: String,
    creation_time: SystemTime,
    pub cards: Vec<FlashCard>,
    pub stats: Option<DeckStat>,
}

pub struct DeckStat {
    pub id: Option<u32>,
    pub due: u32,
    pub learning: u32,
    pub new: u32,
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
            cards: Vec::new(),
            stats: None,
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
        let mut stmt = conn.prepare("SELECT id, name, creation_time FROM decks")?;

        let decks = stmt.query_map([], |row| {
            let creation_time: String = row.get(2)?;

            let creation_time = DateTime::parse_from_rfc3339(&creation_time)
                .unwrap()
                .naive_utc();

            let creation_time = OffsetDateTime::from_unix_timestamp(creation_time.timestamp())
                .unwrap()
                .into();

            Ok(Deck {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                creation_time,
                cards: Vec::new(),
                stats: None,
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
    pub fn load(id: u32, conn: &Connection) -> Result<Deck> {
        let mut stmt = conn.prepare("SELECT id, name, creation_time FROM decks WHERE id = ?")?;

        let deck = stmt.query_row(&[&id], |row| {
            let creation_time: String = row.get(2)?;

            let creation_time = DateTime::parse_from_rfc3339(&creation_time)
                .unwrap()
                .naive_utc();

            let creation_time = OffsetDateTime::from_unix_timestamp(creation_time.timestamp())
                .unwrap()
                .into();

            Ok(Deck {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                creation_time,
                cards: Vec::new(),
                stats: None,
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
        match self.id {
            Some(id) => {
                conn.execute(
                    "UPDATE decks SET name = ? WHERE id = ?",
                    &[&self.name, &id.to_string()],
                )?;
            }
            None => {
                conn.execute(
                    "INSERT INTO decks (name, creation_time) VALUES (?, ?)",
                    &[
                        &self.name,
                        &DateTime::<Utc>::from(self.creation_time).to_rfc3339(),
                    ],
                )?;

                let id = conn.last_insert_rowid();
                self.id = Some(id as u32);
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
    pub fn delete(id: u32, conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM decks WHERE id = ?", &[&id])?;

        Ok(())
    }

    /// Retrieves the statistics for the deck.
    ///
    /// The statistics include the number of due, learning, and new cards in the deck.
    ///
    /// # Returns
    ///
    /// DeckStat - The statistics for the deck.
    pub fn get_deck_stats(&self) -> DeckStat {
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

        DeckStat {
            id: self.id,
            due,
            learning,
            new,
        }
    }

    pub fn get_decks_stats(conn: &Connection) -> Result<HashMap<u32, DeckStat>> {
        let params = named_params! {
            ":new_queue" : CardQueue::New as u8,
            ":learn_queue" : CardQueue::Learning as u8,
            ":review_queue" : CardQueue::Review as u8,
        };

        conn.prepare(include_str!("query_decks_stats.sql"))?
            .query_and_then(params, row_to_deck_stat)?
            .collect()
    }
}

fn row_to_deck_stat(row: &rusqlite::Row) -> Result<(u32, DeckStat)> {
    let deck_id = row.get(0)?;
    let new = row.get(1)?;
    let learning = row.get(2)?;
    let due = row.get(3)?;

    Ok((
        deck_id,
        DeckStat {
            id: Some(deck_id),
            new,
            learning,
            due,
        },
    ))
}

#[cfg(test)]
mod test {
    use std::time::Duration;

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
        assert_eq!(
            loaded_deck.err().unwrap().to_string(),
            "Query returned no rows"
        );
    }

    #[test]
    fn get_deck_stats() {
        let mut deck = Deck::new("Test Deck");

        let mut due_card = FlashCard::new(1, "Front", "Back", Some(0.2));
        due_card.set_last_studied_time(SystemTime::now() - Duration::from_secs(24 * 60 * 60 * 2));
        deck.cards.push(due_card);

        let mut learning_card = FlashCard::new(1, "Front", "Back", None);
        learning_card.set_last_studied_time(SystemTime::now());
        deck.cards.push(learning_card);

        let new_card = FlashCard::new(1, "Front", "Back", None);
        deck.cards.push(new_card);

        let stats = deck.get_deck_stats();

        assert_eq!(stats.new, 1);
        assert_eq!(stats.learning, 1);
        assert_eq!(stats.due, 1);
    }

    #[test]
    fn get_decks_stats() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let mut due_card = FlashCard::new(1, "Front", "Back", Some(0.2));
        due_card.set_queue(CardQueue::Review);
        due_card.save(&conn).unwrap();

        let mut learning_card = FlashCard::new(1, "Front", "Back", None);
        learning_card.set_queue(CardQueue::Learning);
        learning_card.save(&conn).unwrap();

        let mut new_card = FlashCard::new(1, "Front", "Back", None);
        new_card.save(&conn).unwrap();

        let stats = Deck::get_decks_stats(&conn).unwrap();

        assert_eq!(stats.len(), 1);
        let deck_stat = stats.get(&deck.id.unwrap()).unwrap();
        assert_eq!(deck_stat.due, 1);
        assert_eq!(deck_stat.new, 1);
        assert_eq!(deck_stat.learning, 1);
    }
}
