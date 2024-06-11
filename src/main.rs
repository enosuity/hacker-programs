use minisearch::run;
use std::{env, process};



fn main() {
    // =========== using string vector =============
    // let args: Vec<String> = env::args().collect();

    // let config = minigrep::Config::build(args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });    

    // ============= same thing we can do using Iterator ===============    
    let config = minisearch::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    run(config).unwrap_or_else(|err|{
        eprintln!("Application error: {err}");
        process::exit(1);
    });
    
}