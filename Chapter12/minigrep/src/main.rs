use std::env::args;
use std::process::exit;
use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {eprintln!("Problem parsing arguments: {}", err);
    exit(1);});

    if let Err(e) = run(&config) {
        eprintln!("Application error: {}", e);
    }

}

