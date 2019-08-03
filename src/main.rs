use std::env;
use std::fs;

fn main() {
    let (query, filename) = parse_config();

    println!("Search for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    let query = String::from(&args[1]);
    let filename = String::from(&args[2]);

    (query, filename)
}
