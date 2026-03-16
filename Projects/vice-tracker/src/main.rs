use vice_tracker::data::tracker;

fn main() {
    let mut habit_tracker = tracker::Tracker::new("Test.json");
    habit_tracker.add_habit("Nicotine free");
    match habit_tracker.mark(0,true){
        Ok(_) => {println!("Oznaczono id 0 na true");},
        Err(e) => {println!("Ups, {}",e);},
    }
    match habit_tracker.mark(5,true){
        Ok(_) => {println!("Oznaczono id 5 na true");},
        Err(e) => {println!("Ups, {}",e);},
    }
}