use crate::errors::Result;

use super::{builder::Builder, collection::Collection};

#[derive(Debug)]
pub struct Stats {
    pub new: usize,
    pub learning: usize,
    pub review: usize,
}

#[derive(Debug)]
pub struct Queue {
    stats: Stats,
}

pub struct QueueBuilder {}

impl QueueBuilder {
    pub fn new(col: &Collection, deck_id: usize) -> Self {
        QueueBuilder {}
    }
}

impl Builder for QueueBuilder {
    type OutputType = Queue;

    fn build(&mut self) -> Result<Queue> {
        Ok(Queue {
            stats: Stats {
                new: 0,
                learning: 0,
                review: 0,
            },
        })
    }
}
