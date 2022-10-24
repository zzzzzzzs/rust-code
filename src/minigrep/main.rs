extern crate core;

use core::panicking::panic;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    let query = &args[1];
    let file_name = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}


struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}