use std::env::var;
use std::error::Error;
use std::fs::read_to_string;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = if var("IGNORE_CASE").is_ok(){ true } else { false };
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(&config.file_path)?;
    let results = match config.ignore_case{
        false => search(&config.query, &contents),
        true => search_case_insensitive(&config.query, &contents),
    };
    results.iter().for_each(|x| println!("{}", x));
    Ok(())


}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let q = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&q)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}