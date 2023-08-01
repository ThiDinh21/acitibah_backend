// pub struct Note {}

pub struct ChecklistItem<'a> {
    title: &'a str,
    is_done: bool,
}

pub enum Difficulty {
    Trivial,
    Easy,
    Medium,
    Hard,
}

pub enum ResetCounter {
    Daily,
    Weekly,
    Monthly,
}

pub enum RepeatType {
    Daily(usize),
    Weekly(usize),
    Monthly(usize),
    Yearly(usize),
}
