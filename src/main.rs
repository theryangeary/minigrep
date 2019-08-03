use std::env;
use std::fs;

fn main() {
    let config = Config::new();

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new() -> Config {
        let args: Vec<String> = env::args().collect();

        let query = String::from(&args[1]);
        let filename = String::from(&args[2]);

        Config { query, filename }
    }
}
