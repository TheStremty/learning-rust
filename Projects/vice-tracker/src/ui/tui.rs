use crate::utils::readline;
use crate::utils::clear_console;
use std::io::{self, Write};
pub struct Menu {
    title: String,
    options: Vec<String>,
    prompt: String,
}
impl Menu {
    pub fn new(title: &str, options: Vec<&str>, prompt: &str) -> Menu {
        Self {
            title: title.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
            prompt: prompt.to_string(),
        }
    }
    pub fn display(&self) {
        println!("{}", self.title);
        for (i, option) in self.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
        println!("");
    }
    pub fn get_choice(&self) -> usize {
        loop {
            print!("{}",self.prompt);
            let _ = io::stdout().flush();

            let choice = readline();

            if let Some(index) = choice
                .parse::<usize>()
                .ok()
                .filter(|&i| i > 0 && i <= self.options.len())
                .map(|i| i - 1)
            {
                return index;
            }

            clear_console();
            self.display();
            continue;

        }
    }
}