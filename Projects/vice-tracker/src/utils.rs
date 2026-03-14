use std::io;
pub fn readline() -> String{
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}
pub fn clear_console(){
    print!("\x1B[2J\x1B[1;1H");
}