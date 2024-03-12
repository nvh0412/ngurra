use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use rusqlite::{
    params,
    types::{FromSql, ValueRef},
    Connection, Result,
};

use time::OffsetDateTime;

#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    New,
    Learning,
    Due,
}

#[derive(Clone)]
pub enum CardQueue {
    New = 0,
    Learning = 1,
    Review = 2,
}

impl FromSql for CardQueue {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        if let ValueRef::Integer(i) = value {
            match i {
                0 => Ok(CardQueue::New),
                1 => Ok(CardQueue::Learning),
                2 => Ok(CardQueue::Review),
                _ => Err(rusqlite::types::FromSqlError::InvalidType),
            }
        } else {
            Err(rusqlite::types::FromSqlError::InvalidType)
        }
    }
}

#[derive(Clone)]
pub struct FlashCard {
    pub id: Option<u32>,
    deck_id: u32,
    question: String,
    answer: String,
    creation_time: SystemTime,
    last_studied_time: Option<SystemTime>,
    ef: f32,
    interval: f64,
    due: i32,
    queue: CardQueue,
}

impl FlashCard {
    pub fn new(deck_id: u32, question: &str, answer: &str, ef: Option<f32>) -> FlashCard {
        FlashCard {
            id: None,
            deck_id,
            question: question.to_string(),
            answer: answer.to_string(),
            creation_time: SystemTime::now(),
            last_studied_time: None,
            ef: ef.unwrap_or(2.5),
            interval: 1.0,
            due: 0,
            queue: CardQueue::New,
        }
    }

    pub fn set_last_studied_time(&mut self, time: SystemTime) {
        self.last_studied_time = Some(time);
    }

    pub fn get_question(&self) -> &str {
        &self.question
    }

    pub fn get_answer(&self) -> &str {
        &self.answer
    }

    pub fn set_queue(&mut self, queue: CardQueue) {
        self.queue = queue;
    }

    /// Retrieves all cards in the specified deck from the database.
    ///
    /// # Arguments
    ///
    /// * `deck_id` - The ID of the deck to retrieve cards from.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `FlashCard` instances.
    pub fn get_all_cards_in_deck(
        deck_id: u32,
        conn: &Connection,
        limit: i16,
    ) -> Result<Vec<FlashCard>> {
        let mut stmt = conn.prepare("SELECT id, question, answer, creation_time, last_studied_time, ef, interval, due, queue FROM cards WHERE deck_id = ?")?;
        let card_iter = stmt.query_map(&[&deck_id.to_string()], |row| {
            let creation_time: String = row.get(3)?;
            let creation_time = DateTime::parse_from_rfc3339(&creation_time)
                .unwrap()
                .naive_utc();
            let creation_time = OffsetDateTime::from_unix_timestamp(creation_time.timestamp())
                .unwrap()
                .into();

            let last_studied_time: Result<String> = row.get(4);

            let last_studied_time = if let Ok(last_studied_time) = last_studied_time {
                let last_studied_time = DateTime::parse_from_rfc3339(&last_studied_time);

                match last_studied_time {
                    Ok(last_studied_time) => {
                        let last_studied_time = last_studied_time.naive_utc();
                        let last_studied_time =
                            OffsetDateTime::from_unix_timestamp(last_studied_time.timestamp())
                                .unwrap()
                                .into();

                        Some(last_studied_time)
                    }
                    Err(e) => {
                        println!("Error parsing last_studied_time: {}", e);
                        None
                    }
                }
            } else {
                None
            };

            Ok(FlashCard {
                id: Some(row.get(0)?),
                deck_id,
                question: row.get(1)?,
                answer: row.get(2)?,
                creation_time,
                last_studied_time,
                ef: row.get(5)?,
                interval: row.get(6)?,
                due: row.get(7).ok().unwrap_or_default(),
                queue: row.get(8)?,
            })
        })?;

        let mut cards = Vec::new();
        for card in card_iter {
            cards.push(card?);
        }

        Ok(cards)
    }

    pub fn for_each_new_card_in_deck<F>(
        conn: &Connection,
        deck_id: u32,
        queue: CardQueue,
        mut func: F,
    ) -> Result<()>
    where
        F: FnMut(&FlashCard) -> (),
    {
        let mut stmt = conn.prepare(include_str!("query_cards_in_deck_by_queue.sql"))?;

        let mut rows = stmt.query(params![deck_id, queue as i8])?;
        while let row = rows.next()? {
            if let None = row {
                break;
            }

            let row = row.unwrap();

            let creation_time: String = row.get(4)?;
            let creation_time = DateTime::parse_from_rfc3339(&creation_time)
                .unwrap()
                .naive_utc();
            let creation_time = OffsetDateTime::from_unix_timestamp(creation_time.timestamp())
                .unwrap()
                .into();

            let last_studied_time: Result<String> = row.get(5);

            let last_studied_time = if let Ok(last_studied_time) = last_studied_time {
                let last_studied_time = DateTime::parse_from_rfc3339(&last_studied_time);

                match last_studied_time {
                    Ok(last_studied_time) => {
                        let last_studied_time = last_studied_time.naive_utc();
                        let last_studied_time =
                            OffsetDateTime::from_unix_timestamp(last_studied_time.timestamp())
                                .unwrap()
                                .into();

                        Some(last_studied_time)
                    }
                    Err(e) => {
                        println!("Error parsing last_studied_time: {}", e);
                        None
                    }
                }
            } else {
                None
            };

            let card = FlashCard {
                id: Some(row.get(0)?),
                deck_id: row.get(1)?,
                question: row.get(2)?,
                answer: row.get(3)?,
                creation_time,
                last_studied_time,
                ef: row.get(6)?,
                interval: row.get(7)?,
                due: row.get(8)?,
                queue: row.get(9)?,
            };

            func(&card);
        }

        Ok(())
    }

    /// Loads a card from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the card to load.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `Result` containing the loaded card, or an error if the operation fails.
    pub fn load(id: u32, conn: &Connection) -> Result<FlashCard> {
        let mut stmt = conn.prepare(
            "SELECT id, deck_id, question, answer, creation_time, last_studied_time, ef, interval, queue, due FROM cards WHERE id = ?"
        )?;

        let deck = stmt.query_row(&[&id], |row| {
            let creation_time: String = row.get(4)?;
            let creation_time = DateTime::parse_from_rfc3339(&creation_time)
                .unwrap()
                .naive_utc();
            let creation_time = OffsetDateTime::from_unix_timestamp(creation_time.timestamp())
                .unwrap()
                .into();

            let last_studied_time: Result<String> = row.get(5);

            let last_studied_time = if let Ok(last_studied_time) = last_studied_time {
                let last_studied_time = DateTime::parse_from_rfc3339(&last_studied_time)
                    .unwrap()
                    .naive_utc();

                Some(
                    OffsetDateTime::from_unix_timestamp(last_studied_time.timestamp())
                        .unwrap()
                        .into(),
                )
            } else {
                None
            };

            Ok(FlashCard {
                id: Some(row.get(0)?),
                deck_id: row.get(1)?,
                question: row.get(2)?,
                answer: row.get(3)?,
                creation_time,
                last_studied_time,
                ef: row.get(6)?,
                interval: row.get(7)?,
                due: row.get(8).ok().unwrap_or_default(),
                queue: row.get(9)?,
            })
        })?;

        Ok(deck)
    }

    /// Save a card to the database
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
        let last_studied_time = if let Some(last_studied_time) = self.last_studied_time {
            DateTime::<Utc>::from(last_studied_time).to_rfc3339()
        } else {
            DateTime::<Utc>::from(SystemTime::now()).to_rfc3339()
        };

        match self.id {
            Some(id) => {
                conn.execute(
                    "UPDATE cards SET question = ?, answer = ?, deck_id = ?, ef = ?, interval = ?, last_studied_time = ? WHERE id = ?",
                    &[
                        &self.question,
                        &self.answer,
                        &self.deck_id.to_string(),
                        &self.ef.to_string(),
                        &self.interval.to_string(),
                        &last_studied_time.to_string(),
                        &id.to_string()
                    ]
                )?;
            }
            None => {
                let mut stmt = conn.prepare_cached(
                    "INSERT INTO cards (question, answer, creation_time, deck_id, ef, interval, last_studied_time, queue, due) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )?;

                stmt.execute(params![
                    self.question,
                    self.answer,
                    DateTime::<Utc>::from(self.creation_time).to_rfc3339(),
                    self.deck_id.to_string(),
                    self.ef.to_string(),
                    self.interval.to_string(),
                    last_studied_time.to_string(),
                    self.queue.clone() as i8,
                    self.due
                ])?;

                let id = conn.last_insert_rowid();
                self.id = Some(id as u32);
            }
        }

        Ok(())
    }

    /// Deletes the card from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn delete(&self, conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM cards WHERE id = ?", &[&self.id])?;

        Ok(())
    }

    pub fn get_status(&self) -> Status {
        let current_time = SystemTime::now();
        let interval_duration = Duration::from_secs((self.interval * 24.0 * 60.0 * 60.0) as u64);

        if let Some(last_studied_time) = self.last_studied_time {
            if current_time > last_studied_time + interval_duration {
                Status::Due
            } else {
                Status::Learning
            }
        } else {
            Status::New
        }
    }

    pub fn is_due(&self) -> bool {
        self.get_status() == Status::Due
    }

    pub fn is_learning(&self) -> bool {
        self.get_status() == Status::Learning
    }

    pub fn rate(&mut self, rating: u8) {
        let q = self.ef;

        let ef = q + (0.1 - (5 - rating) as f32 * (0.08 + (5 - rating) as f32 * 0.02));
        let ef = if ef < 1.3 { 1.3 } else { ef };

        self.ef = ef;
        self.interval = self.interval * ef as f64;
        self.last_studied_time = Some(SystemTime::now());
    }
}

#[cfg(test)]
mod test {
    use crate::{db::init_db, Deck};

    use super::*;

    #[test]
    fn get_all_cards_in_deck() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let mut card = FlashCard::new(
            deck.id.unwrap(),
            "What is the capital of France?",
            "Paris",
            None,
        );
        card.save(&conn).unwrap();

        let cards = FlashCard::get_all_cards_in_deck(deck.id.unwrap(), &conn, 10).unwrap();
        assert_eq!(cards.len(), 1);
    }

    #[test]
    fn load() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let mut card = FlashCard::new(
            deck.id.unwrap(),
            "What is the capital of France?",
            "Paris",
            None,
        );
        card.save(&conn).unwrap();

        let loaded_card = FlashCard::load(card.id.unwrap(), &conn).unwrap();
        assert_eq!(card.question, loaded_card.question);
        assert_eq!(card.answer, loaded_card.answer);
    }

    #[test]
    fn save() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let mut card = FlashCard::new(
            deck.id.unwrap(),
            "What is the capital of France?",
            "Paris",
            None,
        );
        card.save(&conn).unwrap();

        assert_eq!(card.id, Some(1));
    }

    #[test]
    fn delete() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let mut card = FlashCard::new(
            deck.id.unwrap(),
            "What is the capital of France?",
            "Paris",
            None,
        );
        card.save(&conn).unwrap();

        card.delete(&conn).unwrap();

        let cards = FlashCard::get_all_cards_in_deck(deck.id.unwrap(), &conn, 10).unwrap();
        assert_eq!(cards.len(), 0);
    }

    #[test]
    fn get_status() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let mut card = FlashCard::new(
            deck.id.unwrap(),
            "What is the capital of France?",
            "Paris",
            None,
        );

        assert_eq!(card.get_status(), Status::New);

        card.last_studied_time = Some(SystemTime::now());
        card.save(&conn).unwrap();

        assert_eq!(card.get_status(), Status::Learning);

        card.last_studied_time = Some(SystemTime::now() - Duration::from_secs(60 * 60 * 24 * 2));
        card.interval = 0.2;
        card.save(&conn).unwrap();

        assert_eq!(card.get_status(), Status::Due);
    }

    #[test]
    fn rate() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let mut card = FlashCard::new(
            deck.id.unwrap(),
            "What is the capital of France?",
            "Paris",
            None,
        );

        card.last_studied_time = Some(SystemTime::now() - Duration::from_secs(60 * 60 * 24 * 2));
        card.save(&conn).unwrap();

        let old_interval = card.interval;

        card.rate(5);
        assert_eq!(card.ef, 2.6);
        assert_eq!(card.interval, old_interval * card.ef as f64);

        card.rate(4);
        assert_eq!(card.ef, 2.6);

        card.rate(3);
        assert_eq!(card.ef, 2.46);

        card.rate(2);
        assert_eq!(card.ef, 2.14);
    }
}
