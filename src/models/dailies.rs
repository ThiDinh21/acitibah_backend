use chrono::{DateTime, Utc};

use super::common::{ChecklistItem, Difficulty, RepeatType};

pub struct Dailies<'a> {
    title: &'a str,
    note: &'a str,
    checklist: Vec<ChecklistItem<'a>>,
    difficulty: Difficulty,
    start_date: DateTime<Utc>,
    repeat_type: RepeatType,
    // repeat_on: todo!,
    tags: Vec<&'a str>,
    timestamp: DateTime<Utc>,
}
