use std::path::PathBuf;

use gpui::{AppContext, Global};

use crate::{errors::Result, storage::sqlite::SqliteStorage, FlashCard};

use super::{
    answer::{self, Answer},
    builder::Builder,
    card::{self, apply_state, get_current_card_state},
    queue::Queue,
};

pub struct CollectionBuilder {
    collection_path: Option<PathBuf>,
}

impl CollectionBuilder {
    pub fn new(col_path: PathBuf) -> Self {
        CollectionBuilder {
            collection_path: Some(col_path),
        }
    }
}

impl Builder for CollectionBuilder {
    type OutputType = Collection;

    fn build(&mut self) -> Result<Collection> {
        let col_path = self
            .collection_path
            .clone()
            .unwrap_or_else(|| PathBuf::from(":memory:"));

        let storage = SqliteStorage::open_or_create(&col_path)?;

        let col = Collection {
            storage,
            col_path,
            card_queues: None,
        };

        Ok(col)
    }
}

pub struct Collection {
    pub storage: SqliteStorage,
    pub col_path: PathBuf,
    pub card_queues: Option<Queue>,
}

impl Collection {
    pub fn init(col: Self, cx: &mut AppContext) {
        cx.set_global(col);
    }

    pub fn answer_card(&self, card_id: u32, answer: Answer) {
        let mut card = FlashCard::load(card_id, &self.storage.conn).unwrap();

        let current_card_state = get_current_card_state(&card);
        let next_state = current_card_state.next_states();

        match answer {
            Answer::Again => {
                let next = next_state.again;
                apply_state(&mut card, next);
            }
            Answer::Hard => {
                let next = next_state.hard;
                apply_state(&mut card, next);
            }
            Answer::Good => {
                let next = next_state.good;
                apply_state(&mut card, next);
            }
            Answer::Easy => {
                let next = next_state.easy;
                apply_state(&mut card, next);
            }
        }

        card.save(&self.storage.conn).unwrap();
    }
}

impl Global for Collection {}

// impl Into<CollectionBuilder> for Collection {
//     fn into(self) -> CollectionBuilder {
//         CollectionBuilder {
//             collection_path: Some(self.col_path),
//             deck_id: None,
//         }
//     }
// }
