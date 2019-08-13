use std::{fs, error::Error};

pub fn run(config: Config) -> Result< (), Box<dyn Error> > {
    let contents = fs::read_to_string(config.file_name)?;
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

fn search<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(search_string) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let search_string = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(search_string, contents));
    }
}