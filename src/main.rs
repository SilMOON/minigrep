use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("Searching for: \"{}\".\nIn file: {}.", config.search_string, config.file_name);

    let contents = fs::read_to_string(config.file_name)
        .expect("Failed to read the file.");

    println!("With text:\n{}", contents);
}

struct Config {
    search_string: String,
    file_name: String,
}

fn parse_config(args: &[String]) -> Config {
    let search_string = args[1].clone();
    let file_name = args[2].clone();
    Config { search_string, file_name }
}