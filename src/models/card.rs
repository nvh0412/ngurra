use crate::{repositories::flash_card::CardQueue, FlashCard};

use super::states::{
    card_state::CardState, learning_state::LearningState, new_state::NewState,
    review_state::ReviewState,
};

pub fn get_current_card_state(card: &FlashCard) -> CardState {
    // get due from the card
    let due = card.due;
    let interval = card.interval;
    let ef = card.ease_factor();

    match *card.get_queue() {
        CardQueue::New => NewState {}.into(),
        CardQueue::Learning => LearningState {
            remaining_steps: 0,
            scheduled_secs: 0,
            elapsed_secs: 0,
            memory_state: None,
        }
        .into(),
        CardQueue::Review => ReviewState {}.into(),
    }
}
