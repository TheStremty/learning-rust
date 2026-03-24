use crate::data::habits::Habit;
use chrono::{Local, Duration};
use crate::data;

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

    pub fn get_habits(&self) -> &Vec<Habit> {
        &self.habits
    }
    pub fn set_habits(&mut self, habits: Vec<Habit>) {
        self.habits = habits;
    }

    pub fn get_streak(&self, id: u32) -> u32 {
        let Some(habit) = self.habits.iter().find(|h| h.id == id) else {
            return 0;
        };

        let today = Local::now().date_naive();
        let yesterday = today - Duration::days(1);

        let mut current_date = if habit.successes.contains(&today) {
            today
        } else if habit.successes.contains(&yesterday) {
            yesterday
        } else {
            return 0;
        };

        let mut streak = 0;

        while habit.successes.contains(&current_date) {
            streak += 1;
            current_date = current_date - Duration::days(1);
        }

        streak
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

    pub fn save(&self) {
        data::persistence::save(self).unwrap();
    }

    pub fn load(&mut self) {
        data::persistence::load(self).unwrap();
    }

}