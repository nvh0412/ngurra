pub struct SchedTimingToday {
    pub now: i64,
    /// The number of days that have passed since the collection was created.
    pub days_elapsed: u32,
    /// Timestamp of the next day rollover.
    pub next_day_at: i64,
}
