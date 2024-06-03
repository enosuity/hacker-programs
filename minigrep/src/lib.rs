use std::{error::Error, fs, path};
use std::env;

#[allow(unused)]

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments ");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
                    query: query,
                    file_path,
                    ignore_case,
                })
        
    }    
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let lines: Vec<&str> = if config.ignore_case {
        search_with_case_ignore(&config.query, &content)
    }  else {
        search(&config.query, &content)
    };

    for line in lines {
        println!("{}", line);
    }    

    Ok(())
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results        
}

fn search_with_case_ignore<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line.trim());
        }
    }
    results        
}

#[cfg(test)]
mod minigrep {
    use super::*;
    #[test]
    fn search_with_test() {
        let content = "\
        In other words, 
        we tell Rust that the data returned by the search function will live
        as long as the data passed into the search function in the contents argument.
        This is important! The data referenced by a slice needs to be valid for the reference to be valid;
        if the compiler assumes we’re making string slices of query rather than contents,
        it will do its safety checking incorrectly.";

        let query = "slice";
        let expected_res = vec![
            "This is important! The data referenced by a slice needs to be valid for the reference to be valid;",
            "if the compiler assumes we’re making string slices of query rather than contents,"
        ];
        assert_eq!(search(query, content), expected_res);        
    }

    #[test]
    fn search_with_capital_with_test() {
        let content = "\
        In other words, 
        we tell Rust that the data returned by the search function will live
        as long as the data passed into the search function in the contents argument.
        This is important! The data referenced by a slice needs to be valid for the reference to be valid;
        if the compiler assumes we’re making string slices of query rather than contents,
        it will do its safety checking incorrectly.";

        let query = "SLICE";
        let expected_res: Vec<&str> = vec![];
        
        assert_eq!(search(query, content), expected_res);    
    }

    #[test]
    fn search_case_insensitive_with_test() {
        let content = "\
        In other words, 
        we tell Rust that the data returned by the search function will live
        as long as the data passed into the search function in the contents argument.
        This is important! The data referenced by a slice needs to be valid for the reference to be valid;
        if the compiler assumes we’re making string slices of query rather than contents,
        it will do its safety checking incorrectly.";

        let query = "Slice";
        let expected_res = vec![
            "This is important! The data referenced by a slice needs to be valid for the reference to be valid;",
            "if the compiler assumes we’re making string slices of query rather than contents,"
        ];
        assert_eq!(search_with_case_ignore(query, content), expected_res);
        
    }

    #[test]
    fn search_case_sensitive_with_test() {
        let content = "\
        In other words, 
        we tell Rust that the data returned by the search function will live
        as long as the data passed into the search function in the contents argument.
        This is important! The data referenced by a slice needs to be valid for the reference to be valid;
        if the compiler assumes we’re making string slices of query rather than contents,
        it will do its safety checking incorrectly.";

        let query = "slice";
        let expected_res = vec![
            "This is important! The data referenced by a slice needs to be valid for the reference to be valid;",
            "if the compiler assumes we’re making string slices of query rather than contents,"
        ];
        assert_eq!(search_with_case_ignore(query, content), expected_res);
        
    }
    
}