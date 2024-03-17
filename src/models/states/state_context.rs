pub struct StateContext {
    pub new_per_day: u32,
    pub reviews_per_day: u32,
    // learning
    pub graduating_interval_good: u32,
    pub graduating_interval_easy: u32,
    pub initial_ease_factor: f32,

    // reviewing
    pub hard_multiplier: f32,
    pub easy_multiplier: f32,
    pub interval_multiplier: f32,
    pub maximum_review_interval: u32,
}

impl Default for StateContext {
    fn default() -> Self {
        StateContext {
            new_per_day: 20,
            reviews_per_day: 200,
            // learning
            graduating_interval_good: 1,
            graduating_interval_easy: 4,
            initial_ease_factor: 2.5,

            // reviewing
            hard_multiplier: 1.2,
            easy_multiplier: 1.3,
            interval_multiplier: 1.0,
            maximum_review_interval: 36_500,
        }
    }
}

impl StateContext {
    pub(crate) fn min_and_max_review_intervals(&self, minimum: u32) -> (u32, u32) {
        let maximum = self.maximum_review_interval.max(1);
        let minimum = minimum.clamp(1, maximum);
        (minimum, maximum)
    }
}
