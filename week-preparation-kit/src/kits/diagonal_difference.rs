#![allow(unused)]
#[cfg(feature = "diagonal-diff")]

pub fn run(matrix: &[Vec<i32>]) -> i32 {
  let (s1, s2) = matrix.iter().enumerate().fold((0, 0), |(mut s1, mut s2), (index, list)| {
    s1 += list.get(index).copied().unwrap_or(0);
    s2 += list.get(list.len() - (index + 1)).copied().unwrap_or(0);
    (s1, s2)
  });
  
  (s1 - s2).abs() as i32
}

// ============ code in main.rs ===================

// use std::io;
// use std::io::prelude::*;

// use week_preparation_kit::kits::diagonal_difference;

// fn main() {

//     let stdin = io::stdin();
    
    
//     let mut input_iter = stdin.lock().lines();

//     println!("Enter Matrix size: ");
//     let matrix_size = input_iter.next().unwrap().unwrap().parse::<i8>().unwrap();

//     let mut matrix = Vec::with_capacity(matrix_size as usize);

//     for i in 0..matrix_size as usize {
//         matrix.push(Vec::with_capacity(matrix_size as usize));

//         matrix[i] = input_iter.next().unwrap().unwrap()
//                               .trim()
//                               .split(" ")
//                               .map(|x| x.parse::<i32>().unwrap())
//                               .collect();
//     }

//     let res = diagonal_difference::run(&matrix);

//     println!("Lonely integer => {:?}", res);   
// }

