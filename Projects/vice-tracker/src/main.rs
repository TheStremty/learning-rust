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
    let mut coffee_habit = Habit::new(String::from("Picie kawy"), 30,0,true);
    coffee_habit.add_day();
    coffee_habit.print_report();
}