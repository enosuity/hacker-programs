use std::io;
use std::io::prelude::*;

use week_preparation_kit::kits::diagonal_difference;

fn main() {

    let stdin = io::stdin();
    
    
    let mut input_iter = stdin.lock().lines();

    println!("Enter Matrix size: ");
    let matrix_size = input_iter.next().unwrap().unwrap().parse::<i8>().unwrap();

    let mut matrix = Vec::with_capacity(matrix_size as usize);

    for i in 0..matrix_size as usize {
        matrix.push(Vec::with_capacity(matrix_size as usize));

        matrix[i] = input_iter.next().unwrap().unwrap()
                              .trim()
                              .split(" ")
                              .map(|x| x.parse::<i32>().unwrap())
                              .collect();
    }

    let res = diagonal_difference::run(&matrix);

    println!("Lonely integer => {:?}", res);   
}

