use crate::models::queue::SchedulingStates;

use super::card_state::{CardState, CardStateTrait};

#[derive(Clone)]
pub struct ReviewState {}

impl CardStateTrait for ReviewState {
    fn next_states(self) -> SchedulingStates {
        todo!()
    }
}

impl Into<CardState> for ReviewState {
    fn into(self) -> CardState {
        CardState::Review(self)
    }
}
