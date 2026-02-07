use minigrep::Config;
use std::{env, process};

fn main() {
    let agrs: Vec<String> = env::args().collect();

    let config = Config::new(&agrs).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Failed to run minigrep: {}", e);
        process::exit(1);
    }
}
