#![warn(dead_code)]
use std::{i32, io::{self, BufRead}};

use week_preparation_kit::kits::zig_zag_sequence;


fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let mut a: Vec<i32> = lines.next().unwrap().unwrap().trim()
                                   .split_whitespace()
                                   .map(|x| x.parse().unwrap())
                                   .collect();
                                
        zig_zag_sequence::execute(&mut a, n);
    }
   
}