struct Habit{
    name: String,
    goal_days: u32,
    current_days: u32,
    is_active: bool,
}
impl Habit{
    pub fn new(name: String, goal_days: u32, current_days: u32, is_active: bool) -> Habit{
        Habit{name,goal_days,current_days,is_active}
    }
    pub fn add_day(&mut self){
        self.current_days += 1;
    }
    pub fn reset(&mut self){
        self.current_days = 0;
    }
    pub fn get_status(&self) -> bool{
        self.current_days >= self.goal_days
    }
    pub fn print_report(&self){
        println!("[{}] {}/{} days.", self.name, self.current_days, self.goal_days);
    }
}