use std::io;
use std::io::prelude::*;

use week_preparation_kit::kits::comparison_sorting;

fn main() {

    let stdin = io::stdin(); 
    
    let mut input_iter = stdin.lock().lines();

    println!("Enter Matrix size: ");
    let matrix_size = input_iter.next().unwrap().unwrap().parse::<i32>().unwrap();

    let mut matrix = Vec::with_capacity(matrix_size as usize);

    matrix = input_iter.next().unwrap().unwrap()
                        .trim()
                        .split(" ")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();
    

    let res = comparison_sorting::run(&matrix);

    println!("Lonely integer => {:?}", res);   
}

