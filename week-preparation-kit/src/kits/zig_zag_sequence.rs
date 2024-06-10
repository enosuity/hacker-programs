#![allow(unused)]
#[cfg(feature = "zig-zag-sq")]


pub fn execute(a: &mut [i32], n: usize) -> (){
    a.sort();
    let mid = (n - 1) / 2; // line1
    a.swap(mid, n - 1);

    let mut st = mid + 1;
    let mut ed = n - 2; // line2

    while st <= ed {
        a.swap(st, ed);
        st += 1;
        ed -= 1; // line3
    }

    for (i, &val) in a.iter().enumerate() {
        if i == n - 1 {
            print!("{}", val);
        } else {
            print!("{} ", val);
        }
    }
    println!();  
}


// // ============== main ===============

// #![warn(dead_code)]
// use std::{i32, io::{self, BufRead}};

// use week_preparation_kit::kits::zig_zag_sequence;


// fn main() {
//     let stdin = io::stdin();
//     let mut lines = stdin.lock().lines();

//     let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
//     for _ in 0..t {
//         let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
//         let mut a: Vec<i32> = lines.next().unwrap().unwrap().trim()
//                                    .split_whitespace()
//                                    .map(|x| x.parse().unwrap())
//                                    .collect();
                                
//         zig_zag_sequence::execute(&mut a, n);
//     }
   
// }