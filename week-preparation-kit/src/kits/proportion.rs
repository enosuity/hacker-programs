#![allow(dead_code)]
#[cfg(feature = "proportion")]

use std::io::BufRead;

pub fn vec_elements_proportion<R: BufRead>(input: &mut R) -> Vec<f32>{
    // println!("Please Enter elements separated by space: ");
    // let stdio = io::stdin();
    // let mut lines = stdio.lock().lines();

    let mut input_string = String::new();
    input.read_line(&mut input_string).expect("Failed to read line");
    

    let arr: Vec<f32> = input_string
                             .trim()
                             .split(' ')   
                             .map(|s| s.parse::<f32>().unwrap())
                             .collect();

    plus_minus(&arr)
}

fn plus_minus(arr: &[f32]) -> Vec<f32>{
    let arr_size = arr.len() as f32;

    let (pos, neg, zr) = arr.iter().fold((0, 0, 0), |(pos, neg, zr), &i| {
        if i == 0.0 { (pos, neg, zr + 1) }
        else if i < 0.0 { (pos, neg + 1, zr) }
        else { (pos + 1, neg, zr) }
    });

    let (r1, r2, r3) = (pos as f32 / arr_size,  neg as f32 / arr_size, zr as f32 / arr_size);    

    vec![r1, r2, r3]
        .iter()
        .map(|num| (num * 1_000_000.0).round() / 1_000_000.0)
        .collect()
}