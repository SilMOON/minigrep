use std::{fs, error::Error};

pub fn run(config: Config) -> Result< (), Box<dyn Error> > {
    let contents = fs::read_to_string(config.file_name)?;
    println!("With text:\n{}", contents);
    Ok( () )

}

pub struct Config {
    pub search_string: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result< Config, &'static str > {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let search_string = args[1].clone();
        let file_name = args[2].clone();
        Ok( Config { search_string, file_name })
    }
}
