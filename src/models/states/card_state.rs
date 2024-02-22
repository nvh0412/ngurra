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
            CardState::New(state) => {
                let next_states = LearningState {
                    remaining_steps: 0,
                    scheduled_secs: 0,
                    elapsed_secs: 0,
                    memory_state: None,
                }
                .next_states();

                SchedulingStates {
                    current: state.into(),
                    ..next_states
                }
            }
            CardState::Learning(state) => state.next_states(),
            CardState::Review(state) => state.next_states(),
            CardState::ReLearning(state) => state.next_states(),
        }
    }
}
