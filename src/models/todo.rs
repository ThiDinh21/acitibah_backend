use chrono::{DateTime, Utc};

use super::common::{ChecklistItem, Difficulty};

pub struct Todo<'a> {
    title: &'a str,
    notes: &'a str,
    checklist: Vec<ChecklistItem<'a>>,
    difficulty: Difficulty,
    tags: Vec<&'a str>,
    due_date: DateTime<Utc>,
    timestamp: DateTime<Utc>,
}
