use chrono::{DateTime, Utc};

use super::common::{Difficulty, ResetCounter};

pub struct Habit<'a> {
    title: &'a str,
    notes: &'a str,
    is_positive: bool,
    is_negative: bool,
    difficulty: Difficulty,
    tags: Vec<&'a str>,
    reset_counter: ResetCounter,
    timestamp: DateTime<Utc>,
}

impl<'a> Habit<'a> {
    pub fn new() {
        todo!()
    }
}
