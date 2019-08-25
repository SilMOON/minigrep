use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error in parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for: \"{}\".\nIn file: {}.\n", config.search_string, config.file_name);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

