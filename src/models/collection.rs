use std::path::PathBuf;

use gpui::{AppContext, Global};

use crate::{errors::Result, storage::sqlite::SqliteStorage};

use super::{
    builder::Builder,
    queue::{Queue, QueueBuilder},
};

pub struct CollectionBuilder {
    collection_path: Option<PathBuf>,
    deck_id: Option<usize>,
}

impl CollectionBuilder {
    pub fn new(col_path: PathBuf) -> Self {
        CollectionBuilder {
            collection_path: Some(col_path),
            deck_id: None,
        }
    }

    pub fn set_deck_id(&mut self, deck_id: usize) {
        self.deck_id = Some(deck_id);
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
