use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args()
        .collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {} in file {}", query, filename);

    let mut contents = String::new();
    File::open(filename)
        .expect("file not found")
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("with text:\n{}", contents);
}
