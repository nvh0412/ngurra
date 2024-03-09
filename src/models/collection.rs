use std::path::PathBuf;

use gpui::{AppContext, Global};

use crate::{errors::Result, storage::sqlite::SqliteStorage};

pub struct CollectionBuilder {
    collection_path: Option<PathBuf>,
}

impl CollectionBuilder {
    pub fn new(col_path: PathBuf) -> Self {
        CollectionBuilder {
            collection_path: Some(col_path)
        }
    }

    pub fn build(&mut self) -> Result<Collection> {
        let col_path = self.collection_path.clone().unwrap_or_else(|| PathBuf::from(":memory:"));

        let storage = SqliteStorage::open_or_create(&col_path)?;

        Ok(Collection {
            storage,
            col_path
        })
    }

}

pub struct Collection {
    pub storage: SqliteStorage,
    pub col_path: PathBuf
}

impl Collection {
    pub fn init(col: Self, cx: &mut AppContext) {
        cx.set_global(col);
    }
}

impl Global for Collection {}

impl Into<CollectionBuilder> for Collection {
    fn into(self) -> CollectionBuilder {
        CollectionBuilder {
            collection_path: Some(self.col_path)
        }
    }
}
