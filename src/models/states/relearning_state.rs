use crate::models::queue::SchedulingStates;

use super::card_state::CardStateTrait;

#[derive(Clone)]
pub struct ReLearningState {}

impl CardStateTrait for ReLearningState {
    fn next_states(self) -> SchedulingStates {
        todo!()
    }
}
