use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    File::open(config.filename)?
        .read_to_string(&mut contents)?;
    println!("with text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
