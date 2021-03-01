use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args()
        .collect();
    let config = Config::new(&args)
        .unwrap_or_else(
            |err|
            {
                println!("Problem parsing arguments: {}", err);
                process::exit(1);
            }
        );
    println!("Searching for {} in file {}", config.query, config.filename);

    run(config)
}

fn run(config: Config) {
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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            Ok(
                Config {
                    query: args[1].clone(),
                    filename: args[2].clone(),
                }
            )
        }
    }
}
