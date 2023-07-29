use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // return a result with a Config on success and &'static str on error
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        // use clone() to return a copy
        let query = args[1].clone();
        let file_path = args[2].clone();

        // return as a Config struct
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value if reading the file fails
    let contents = fs::read_to_string(config.file_path)?;

    // .expect("should have beenable to read the file!");

    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    // pass unit type to Ok is idomatic way to say we're using run()
    // just for it's side effects
    Ok(())
    // dbg!(args);
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // lines() returns an iterator
    // string is split on '\n' or '\r\n'
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
