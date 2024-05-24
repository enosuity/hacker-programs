#![allow(dead_code)]
#[cfg(feature = "proportion")]

use std::io::{self, BufRead};

pub fn vec_elements_proportion() {
    println!("Please Enter elements separated by space: ");
    let stdio = io::stdin();
    let mut lines = stdio.lock().lines();
    

    let arr: Vec<i32> = lines.next().unwrap().unwrap()
                             .trim()
                             .split(' ')   
                             .map(|s| s.parse::<i32>().unwrap())
                             .collect();

    plus_minus(&arr);
}

fn plus_minus(arr: &[i32]) {
    let arr_size = arr.len() as f32;

    let (pos, neg, zr) = arr.iter().fold((0, 0, 0), |(pos, neg, zr), &i| {
        if i == 0 { (pos, neg, zr + 1) }
        else if i < 0 { (pos, neg + 1, zr) }
        else { (pos + 1, neg, zr) }
    });

    let (r1, r2, r3) = (pos as f32 / arr_size,  neg as f32 / arr_size, zr as f32 / arr_size);    

    println!("{:.6}\n{:.6}\n{:.6}", r1, r2, r3);
}