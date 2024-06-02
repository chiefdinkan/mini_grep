use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;
    println!("{}", content);

    let found = search(&config.query, &content);
    if found {
        println!("'{}' is present in the file.", config.query);
    } else {
        println!("'{}' is not present in the file.", config.query);
    }

    Ok(())
}

fn search(query: &str, content: &str) -> bool {
    content.contains(query)
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough parameters");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

