use fsrs::MemoryState;

use super::{
    card_state::{CardState, CardStateTrait},
    review_state::ReviewState,
    state_context::StateContext,
};
use crate::models::queue::SchedulingStates;

#[derive(Clone)]
pub struct LearningState {
    pub remaining_steps: u32,
    pub scheduled_secs: u32,
    pub elapsed_secs: u32,
    pub memory_state: Option<MemoryState>,
}

impl CardStateTrait for LearningState {
    fn next_states(self) -> SchedulingStates {
        SchedulingStates {
            again: self.answer_again().into(),
            hard: self.answer_hard().into(),
            good: self.answer_good().into(),
            easy: self.answer_easy().into(),
            current: self.into(),
        }
    }
}

impl LearningState {
    fn answer_again(&self) -> LearningState {
        LearningState {
            remaining_steps: 0,
            scheduled_secs: 60,
            elapsed_secs: 0,
            memory_state: None,
        }
    }
    fn answer_hard(&self) -> LearningState {
        LearningState {
            remaining_steps: 0,
            scheduled_secs: 60,
            elapsed_secs: 0,
            memory_state: None,
        }
    }

    fn answer_good(&self) -> ReviewState {
        let ctx = StateContext::default();

        let next = ReviewState {
            ease_factor: ctx.initial_ease_factor,
            memory_state: None,
            ..Default::default()
        };

        next.next_states().hard.into()
    }

    fn answer_easy(&self) -> ReviewState {
        let ctx = StateContext::default();

        ReviewState {
            ease_factor: ctx.initial_ease_factor,
            memory_state: self.memory_state.clone(),
            ..Default::default()
        }
    }
}

impl Into<CardState> for LearningState {
    fn into(self) -> CardState {
        CardState::Learning(self)
    }
}
