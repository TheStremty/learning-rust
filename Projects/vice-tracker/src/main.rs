use std::io;
use std::process::exit;
struct Habit{
    name: String,
    goal_days: u32,
    current_days: u32,
    is_active: bool,
}
impl Habit{
    fn new(name: String, goal_days: u32, current_days: u32, is_active: bool) -> Habit{
        Habit{name,goal_days,current_days,is_active}
    }
    fn add_day(&mut self){
        self.current_days += 1;
    }
    fn reset(&mut self){
        self.current_days = 0;
    }
    fn get_status(&self) -> bool{
        self.current_days >= self.goal_days
    }
    fn print_report(&self){
        println!("[{}] {}/{} days.", self.name, self.current_days, self.goal_days);
    }
}

fn main(){
    let mut habits = Vec::<Habit>::new();
    let mut choice = readline();

    //konwersja na int + shadowing
    let choice: u32 = match choice.parse() {
        Ok(option) => option,
        Err(_) => exit(1),
    };

    match choice {
        1 => {
            habits.push(add_habit());
            println!("Pomyślnie utworzono nowy nawyk!");
        },
        2 => {
            if !habits.is_empty() {
                for h in &habits{ h.print_report() }
            }
            else {
                println!("Brak nawyków!");
            }
        },
        _ => exit(1),
    }


}

fn add_habit() -> Habit {
    println!("Dodawanie nawyku.\nPodaj nazwę nawyku: ");
    let name = readline();
    println!("Cel (ile dni do końca): ");
    let goal_days: u32 = match readline().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
    };
    Habit::new(name, goal_days, 0, true)
}

fn readline() -> String{
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}