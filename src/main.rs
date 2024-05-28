use std::{env, fs::File, io::{self, BufRead, Write}};

use week_preparation_kit::kits::time_conversion;

fn main() {
    let path = env::var("TIME_OUTPUT").unwrap();

    let stdin = io::stdin();
    
    println!("Please Enter time (hh:mm:ssAM/PM)");
    let mut input_iter = stdin.lock().lines();

    let mut fptr = File::create(path).unwrap();
    let time = input_iter.next().unwrap().unwrap();

    let converted_time = time_conversion::run(&time);
    
    writeln!(&mut fptr, "{}", converted_time).ok();
}

