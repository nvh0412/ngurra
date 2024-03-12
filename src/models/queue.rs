use std::collections::VecDeque;

use rusqlite::Connection;

use crate::{errors::Result, repositories::flash_card::CardQueue, FlashCard};

use super::{builder::Builder, collection::Collection, deck};

#[derive(Debug)]
pub struct Stats {
    pub new: usize,
    pub learning: usize,
    pub review: usize,
}

#[derive(Debug)]
pub struct Queue {
    pub stats: Stats,
    pub core: VecDeque<QueueEntry>,
}

#[derive(Debug, Clone)]
pub struct QueueEntry {
    pub card_id: u32,
}

pub struct QueueBuilder {
    deck_id: u32,
    new: Vec<FlashCard>,
    review: Vec<FlashCard>,
    learning: Vec<FlashCard>,
}

impl QueueBuilder {
    pub fn new(deck_id: u32) -> Self {
        QueueBuilder {
            deck_id,
            new: vec![],
            review: vec![],
            learning: vec![],
        }
    }

    pub fn collect_cards(&mut self, col: &Collection) {
        self.collect_new_cards(&col.storage.conn);
    }

    fn collect_new_cards(&mut self, conn: &Connection) {
        FlashCard::for_each_new_card_in_deck(&conn, self.deck_id, CardQueue::New, |card| {
            self.new.push(card.clone());
        })
        .unwrap_or_else(|e| {
            println!("Error collecting new cards: {:?}", e);
        });

        FlashCard::for_each_new_card_in_deck(&conn, self.deck_id, CardQueue::Learning, |card| {
            self.learning.push(card.clone());
        })
        .unwrap_or_else(|e| {
            println!("Error collecting learning cards: {:?}", e);
        });

        FlashCard::for_each_new_card_in_deck(&conn, self.deck_id, CardQueue::Review, |card| {
            self.review.push(card.clone());
        })
        .unwrap_or_else(|e| {
            println!("Error collecting learning cards: {:?}", e);
        });
    }
}

impl Builder for QueueBuilder {
    type OutputType = Queue;

    fn build(&mut self) -> Result<Queue> {
        let new_count = 0;
        let learning_count = 0;
        let review_count = 0;

        let mut core_queue: VecDeque<QueueEntry> = VecDeque::new();

        self.review.iter().for_each(|card| {
            core_queue.push_back(QueueEntry {
                card_id: card.id.unwrap(),
            });
        });

        self.learning.iter().for_each(|card| {
            core_queue.push_back(QueueEntry {
                card_id: card.id.unwrap(),
            });
        });

        self.new.iter().for_each(|card| {
            core_queue.push_back(QueueEntry {
                card_id: card.id.unwrap(),
            });
        });

        Ok(Queue {
            stats: Stats {
                new: new_count,
                learning: learning_count,
                review: review_count,
            },
            core: core_queue,
        })
    }
}
