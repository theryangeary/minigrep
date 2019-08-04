use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename[..])?;

    for line in search(&config, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&config.query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub insensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = String::from(&args[1]);
        let filename = String::from(&args[2]);
        let insensitive = false;

        Ok(Config { query, filename, insensitive })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let config = Config {
            query: String::from("duct"),
            filename: String::from(""),
            insensitive: false,
        };
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(&config, contents));
    }
}
