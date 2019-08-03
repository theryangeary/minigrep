use std::error::Error;
use std::env;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = String::from(&args[1]);
        let filename = String::from(&args[2]);

        Ok(Config { query, filename })
    }
}
