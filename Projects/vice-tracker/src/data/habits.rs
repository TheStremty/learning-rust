use std::collections::HashSet;
use chrono::NaiveDate;

pub struct Habit {
    pub id: u32,
    pub name: String,
    pub successes: HashSet<NaiveDate>,
}

impl Habit {
    pub fn new(id: u32, name: &str) -> Self {
        let name = name.to_string();
        Self {
            id,
            name,
            successes: HashSet::new(),
        }
    }
}