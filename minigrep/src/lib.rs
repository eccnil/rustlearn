//! # mygrep
//!
//! this is a crate produced during the reading of [the rust book](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
//!
//! it implements something like grep

use std::error::Error;
use std::{fs, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// loads configuration from parameters and from environment variables
    ///
    /// 1. first parameter goes to: **query string**
    /// 2. second parameter goeas to: **file path** to be searched into
    ///
    /// aditionaly the `IGNORE_CASE` environment variable is captured
    /// only mather if is set or not, ignoring its value
    ///
    /// # Usage
    ///
    /// ```
    /// use minigrep::Config;
    /// use std::env;
    /// let config = Config::build(env::args());
    /// ```
    pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        let _executable_name = args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query, file_path, ignore_case})
    }
}

pub fn run (config: Config) -> Result <(), Box<dyn Error>> {
    //! executable for searching
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn  search_case_insensitive <'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search (query, contents));
    }

    #[test]
    fn case_insensitive () {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
