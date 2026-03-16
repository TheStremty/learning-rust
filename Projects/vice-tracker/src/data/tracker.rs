use crate::data::habits::Habit;
use chrono::Local;
pub struct Tracker {
    habits: Vec<Habit>,
    pub db_path: String,
}

impl Tracker {
    pub fn new(db_path: &str) -> Self {
        let db_path = db_path.to_string();
        let habits = Vec::new();
        Self { habits, db_path }
    }
    pub fn add_habit(&mut self, name: &str) {
        let max_id = self.habits.iter()
            .map(|h| h.id)
            .max()
            .map_or(0, |id| id+1);
        self.habits.push(Habit::new(max_id, name));
    }

    pub fn mark(&mut self, id: u32, done: bool) -> Result<(), String> {
        let today = Local::now().date_naive();

        let Some(habit) = self.habits.iter_mut().find(|h| h.id == id) else {
            return Err(format!("No such habit id: {}", id));
        };
        if done {
            habit.successes.insert(today);
        } else {
            habit.successes.remove(&today);
        }
        Ok(())
    }
}