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
        CardQueue::Review => ReviewState { memory_state: None }.into(),
    }
}

pub fn apply_state(card: &mut FlashCard, next: CardState) {
    match next {
        CardState::New(_) => {
            card.set_queue(CardQueue::New);
        }
        CardState::Learning(next_learning_state) => {
            card.set_queue(CardQueue::Learning);
            card.memory_state = next_learning_state.memory_state;
        }
        CardState::Review(next_review_state) => {
            card.set_queue(CardQueue::Review);
            card.memory_state = next_review_state.memory_state;
        }
        _ => {}
    }
}
