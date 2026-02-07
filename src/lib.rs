use std::error::Error;
use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Teeny-tiny grep implementation in Rust")]
pub struct Config {
    /// Pattern to search for
    pattern: String,

    /// Path to file
    filepath: String,

    /// Perform case insensitive matching
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,

    /// Display line number
    #[arg(short, long, default_value_t = false)]
    display_line_number: bool,

    /// Print num lines of trailing context after each match
    #[arg(short, long)]
    after_context: Option<u32>,

    /// Print num lines of leading context before each match
    #[arg(short, long)]
    before_context: Option<u32>,

    /// Print num lines of leading and trailing context surrounding each match
    #[arg(short, long)]
    context: Option<u32>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;

    let results = search(&config.pattern, &content);
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }
    results
}
