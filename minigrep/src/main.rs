use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing args: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}