use clap::Parser;
use minigrep::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Failed to run minigrep: {}", e);
        process::exit(1);
    }
}
