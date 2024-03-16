use crate::models::queue::SchedulingStates;

use super::{
    learning_state::LearningState, new_state::NewState, relearning_state::ReLearningState,
    review_state::ReviewState,
};

#[derive(Clone)]
pub enum CardState {
    New(NewState),
    Learning(LearningState),
    Review(ReviewState),
    ReLearning(ReLearningState),
}

pub trait CardStateTrait {
    fn next_states(self) -> SchedulingStates;
}

impl CardState {
    pub fn next_states(self) -> SchedulingStates {
        match self {
            CardState::New(state) => state.next_states(),
            CardState::Learning(state) => state.next_states(),
            CardState::Review(state) => state.next_states(),
            CardState::ReLearning(state) => state.next_states(),
        }
    }
}
