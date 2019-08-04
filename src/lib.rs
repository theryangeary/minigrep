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
        .filter(|line| match config.insensitive {
            false => line.contains(&config.query),
            true => line.to_lowercase().contains(&config.query.to_lowercase()),
        })
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

        Config::build(args)
    }

    fn build(args: Vec<String>) -> Result<Config, &'static str> {
        Ok(Config {
            query: String::from(&args[args.len() - 2]),
            filename: String::from(&args[args.len() - 1]),
            insensitive: args
                .iter()
                .filter(|flag| **flag == String::from("-i"))
                .collect::<Vec<&String>>()
                .len()
                > 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let config = Config::build(vec![String::from("duct"), String::from("")]).unwrap();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(&config, contents));
    }

    #[test]
    fn case_insensitive() {
        let config = Config::build(vec![
            String::from("-i"),
            String::from("rUsT"),
            String::from(""),
        ])
        .unwrap();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(&config, contents));
    }
}
