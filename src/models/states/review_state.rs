use fsrs::MemoryState;

use crate::models::queue::SchedulingStates;

use super::{
    card_state::{CardState, CardStateTrait},
    state_context::StateContext,
};

pub const INITIAL_EASE_FACTOR: f32 = 2.5;

#[derive(Clone)]
pub struct ReviewState {
    pub scheduled_days: u32,
    pub memory_state: Option<MemoryState>,
    pub ease_factor: f32,
}

impl Default for ReviewState {
    fn default() -> Self {
        ReviewState {
            scheduled_days: 0,
            memory_state: None,
            ease_factor: INITIAL_EASE_FACTOR,
        }
    }
}

impl CardStateTrait for ReviewState {
    fn next_states(self) -> SchedulingStates {
        let (hard_interval, good_interval, easy_interval) = self.passing_review_intervals();

        SchedulingStates {
            again: self.answer_again(0).into(),
            hard: self.answer_hard(hard_interval).into(),
            good: self.answer_good(good_interval).into(),
            easy: self.answer_easy(easy_interval).into(),
            current: self.into(),
        }
    }
}

impl ReviewState {
    fn passing_review_intervals(&self) -> (u32, u32, u32) {
        let current_interval = self.scheduled_days as f32;
        let ctx = StateContext::default();

        // hard
        let hard_factor = ctx.hard_multiplier;
        let hard_minimum = if hard_factor <= 1.0 {
            0
        } else {
            self.scheduled_days + 1
        };
        let hard_interval =
            Self::constrain_passing_interval(&ctx, current_interval * hard_factor, hard_minimum);

        let good_minimum = if hard_factor <= 1.0 {
            self.scheduled_days + 1
        } else {
            hard_interval + 1
        };
        let good_interval = Self::constrain_passing_interval(
            &ctx,
            (current_interval / 2.0) * self.ease_factor,
            good_minimum,
        );

        let easy_interval = Self::constrain_passing_interval(
            &ctx,
            (current_interval) * self.ease_factor * ctx.easy_multiplier,
            good_interval + 1,
        );

        (hard_interval, good_interval, easy_interval)
    }

    fn answer_again(&self, scheduled_days: u32) -> ReviewState {
        ReviewState {
            memory_state: None,
            scheduled_days,
            ..self.clone()
        }
    }
    fn answer_hard(&self, scheduled_days: u32) -> ReviewState {
        ReviewState {
            memory_state: None,
            scheduled_days,
            ..self.clone()
        }
    }

    fn answer_good(&self, scheduled_days: u32) -> ReviewState {
        ReviewState {
            memory_state: None,
            scheduled_days,
            ..self.clone()
        }
    }

    fn answer_easy(&self, scheduled_days: u32) -> ReviewState {
        ReviewState {
            memory_state: self.memory_state.clone(),
            scheduled_days,
            ..self.clone()
        }
    }

    fn constrain_passing_interval(ctx: &StateContext, interval: f32, minimum: u32) -> u32 {
        let interval = interval * ctx.interval_multiplier;

        let (minimum, maximum) = ctx.min_and_max_review_intervals(minimum);

        (interval.round() as u32).clamp(minimum, maximum)
    }
}

impl Into<CardState> for ReviewState {
    fn into(self) -> CardState {
        CardState::Review(self)
    }
}
