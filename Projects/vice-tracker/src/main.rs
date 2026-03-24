use vice_tracker::data::persistence::{save, load};
use vice_tracker::data::tracker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tworzymy nowy tracker i dodajemy habit
    let mut habit_tracker = tracker::Tracker::new("vice.data");
    habit_tracker.add_habit("Nicotine free");

    // Oznaczamy istniejący habit
    match habit_tracker.mark(0, true) {
        Ok(_) => println!("Oznaczono id 0 na true"),
        Err(e) => println!("Ups, {}", e),
    }

    // Próba oznaczenia nieistniejącego habit (id 5)
    match habit_tracker.mark(5, true) {
        Ok(_) => println!("Oznaczono id 5 na true"),
        Err(e) => println!("Ups, {}", e),
    }

    // Zapis do pliku
    //habit_tracker.save();

    // Test: wczytanie z pliku do nowego trackera
    let mut loaded_tracker = tracker::Tracker::new("vice.data");
    loaded_tracker.load();

    // Wypisz, co zostało wczytane
    for habit in loaded_tracker.get_habits() {
        println!("Habit {}: '{}', sukcesy: {:?}", habit.id, habit.name, habit.successes);
        let s = loaded_tracker.get_streak(habit.id);
        if s > 0 {
            println!("🔥 {} - Seria: {} dni!", habit.name, s);
        }
    }

    Ok(())
}