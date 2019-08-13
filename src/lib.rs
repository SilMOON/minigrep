use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result< (), Box<dyn Error> > {
    let contents = fs::read_to_string(config.file_name)?;
    let results = if config.case_sensitive {
        search(&config.search_string, &contents)
    } else {
        search_case_insensitive(&config.search_string, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok( () )
}

pub struct Config {
    pub search_string: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result< Config, &'static str > {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let search_string = args[1].clone();
        let file_name = args[2].clone();

        let case_sensitive;
        if args.len() > 3 {
            if args[3] == String::from("--insensitive") {
                case_sensitive = false;
            } else if args[3] == String::from("--sensitive") {
                case_sensitive = true;
            } else {
                return Err("Found invalid arguments!");
            }
        } else {
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        }

        Ok( Config { search_string, file_name, case_sensitive })
    }
}

fn search<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(search_string) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results: Vec<&str> = Vec::new();
    let search_string = search_string.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&search_string) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search_string = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], 
                    search(search_string, contents));
    }


    #[test]
    fn case_insensitive() {
        let search_string = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
                    search_case_insensitive(search_string, contents));
    }

}
