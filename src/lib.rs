use std::{error::Error, fs};

pub struct Config {
    pub pattern: String,
    pub filepath: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough argunets");
        }

        let pattern = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { pattern, filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;

    let results = search(&config.pattern, &content);
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }
    results
}
