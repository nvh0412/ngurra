use crate::models::queue::SchedulingStates;

use super::{
    card_state::{CardState, CardStateTrait},
    learning_state::LearningState,
};

#[derive(Clone)]
pub struct NewState {}

impl CardStateTrait for NewState {
    fn next_states(self) -> SchedulingStates {
        let next_state = LearningState {
            remaining_steps: 0,
            scheduled_secs: 0,
            elapsed_secs: 0,
            memory_state: None,
        }
        .next_states();

        SchedulingStates {
            current: self.into(),
            ..next_state
        }
    }
}

impl Into<CardState> for NewState {
    fn into(self) -> CardState {
        CardState::New(self)
    }
}
