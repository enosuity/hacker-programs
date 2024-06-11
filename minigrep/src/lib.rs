//! # minigrep
//!
//! `minigrep` is a collection of utilities to make search 
//! as like grep.

/// Adds more modules like store and custom closures .
// --snip--

use std::{error::Error, fs};
use std::env;

pub mod inventory;


#[allow(unused)]




#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments ");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     let ignore_case = env::var("IGNORE_CASE").is_ok();

    //     Ok(Config {
    //                 query: query,
    //                 file_path,
    //                 ignore_case,
    //             })
        
    // }
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get any file path"),
        };

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

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line.trim());
    //     }
    // }
    // results 
    contents.lines() 
            .map(|line| line.trim())  
            .filter(|line| line.contains(query))
            .collect()
            
}

fn search_with_case_ignore<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query.to_lowercase()) {
    //         results.push(line.trim());
    //     }
    // }
    // results
    contents.lines()
            .map(|line| line.trim())
            .filter(|&line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()        
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


// ============== main fn ==================

// use minigrep::run;
// use std::{env, process};



// fn main() {
//     // =========== using string vector =============
//     // let args: Vec<String> = env::args().collect();

//     // let config = minigrep::Config::build(args).unwrap_or_else(|err| {
//     //     eprintln!("Problem parsing arguments: {}", err);
//     //     process::exit(1);
//     // });    

//     // ============= same thing we can do using Iterator ===============    
//     let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });


//     run(config).unwrap_or_else(|err|{
//         eprintln!("Application error: {err}");
//         process::exit(1);
//     });
    
// }