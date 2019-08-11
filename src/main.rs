use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error in parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for: \"{}\".\nIn file: {}.\n", config.search_string, config.file_name);
    run(config);
}

fn run(config: Config) -> Result< (), Box<dyn Error> > {
    let contents = fs::read_to_string(config.file_name)
        .expect("Failed to read the file.");

    println!("With text:\n{}", contents);

    Ok( () )

}

struct Config {
    search_string: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result< Config, &'static str > {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let search_string = args[1].clone();
        let file_name = args[2].clone();
        Ok( Config { search_string, file_name })
    }
}
