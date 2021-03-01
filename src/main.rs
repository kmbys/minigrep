use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args()
        .collect();
    let config = Config::new(&args);
    println!("Searching for {} in file {}", config.query, config.filename);

    let mut contents = String::new();
    File::open(config.filename)
        .expect("file not found")
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("with text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}
