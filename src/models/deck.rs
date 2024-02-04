use rusqlite::Connection;

use crate::{Deck, FlashCard};

pub fn get_decks(conn: &Connection) -> Vec<Deck> {
    let conn = Connection::open("anki-rs.db").unwrap();
    let decks_res = Deck::get_all_decks(&conn);

    match decks_res {
        Ok(mut decks) => {
            decks.iter_mut().for_each(|d| {
                d.cards = FlashCard::get_all_cards_in_deck(d.id.unwrap(), &conn).unwrap();
            });
            decks
        }
        Err(e) => {
            eprintln!("Error getting decks: {}", e);
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use crate::{db::init_db, Deck};

    use super::get_decks;

    #[test]
    fn test_get_decks() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut deck = Deck::new("Test Deck");
        deck.save(&conn).unwrap();

        let decks = get_decks(&conn);

        assert_eq!(decks.len(), 1);
    }
}
