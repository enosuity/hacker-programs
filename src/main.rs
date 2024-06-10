#![warn(dead_code)]
use std::{i32, io::{self, BufRead}};

use week_preparation_kit::kits::tower_breaker;


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let res = tower_breaker::execute(n, m);
        println!("{}", res);
    }
   
}
