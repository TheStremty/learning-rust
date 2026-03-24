use std::collections::HashSet;
use std::error::Error;
use crate::data::tracker::Tracker;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use chrono::NaiveDate;
use crate::data::habits::Habit;

pub fn save(tracker: &Tracker) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(&tracker.db_path)?;

    for habit in tracker.get_habits() {
        writeln!(f, "SOH")?;
        writeln!(f, "{}", habit.id)?;
        writeln!(f, "{}", habit.name)?;

        for date in &habit.successes {
            writeln!(f, "{}", date.format("%d.%m.%Y"))?;
        }
        writeln!(f, "EOH")?;
    }
    Ok(())
}

pub fn load(tracker: &mut Tracker) -> Result<(), Box<dyn Error>> {

    let f = File::open(&tracker.db_path)?;
    let reader = BufReader::new(f);

    enum ParseState {
        Waiting, // For SOH
        Id, // Next line = ID
        Name, // Next line = name
        Dates, // Next lines = dates until EOH
    }

    let mut id: u32 = 0;
    let mut name = String::new();
    let mut successes: HashSet<NaiveDate> = HashSet::new();

    let mut state = ParseState::Waiting;

    let mut habits: Vec<Habit> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        match state {
            ParseState::Waiting => if line == "SOH" { state = ParseState::Id; successes.clear()},
            ParseState::Id => {id = line.parse()?; state = ParseState::Name; },
            ParseState::Name => {name = line; state = ParseState::Dates;},
            ParseState::Dates => if line != "EOH" {
                successes.insert(
                    NaiveDate::parse_from_str(&line, "%d.%m.%Y")?
                );
            } else {
                let mut habit = Habit::new(id, &name);
                habit.successes = std::mem::take(&mut successes);
                habits.push(habit);
                state = ParseState::Waiting;
            }
        }
    }

    tracker.set_habits(habits);

    Ok(())
}