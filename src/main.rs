use minigrep::run;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    run(config).unwrap_or_else(|err|{
        eprintln!("Application error: {err}");
        process::exit(1);
    });
    
}

// IGNORE_CASE=1 cargo run -- Data  hello.txt > my_error.txt