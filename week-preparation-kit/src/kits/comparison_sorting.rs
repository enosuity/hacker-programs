#![allow(unused)]
#[cfg(feature = "comparison-sort")]

pub fn run(list: &[i32]) -> Vec<i32> {
  let max_elem = *list.iter().max().unwrap() as usize;
  let mut arr_size = (max_elem  + 1);
  
  let mut count_output = vec![0; arr_size ];

  for &elem in list {
    count_output[elem as usize] += 1
  }

  for i in 1..count_output.len() {
    count_output[i] += count_output[i -1];
  }

  let mut output: Vec<i32> = vec![0; list.len()];

  list.iter().rev().for_each(|x| {
    let pos = count_output[*x as usize] - 1;
    output[pos] = *x;
    count_output[*x as usize] -= 1;
  });

  output
}

// ================ main code ==================

// use std::io;
// use std::io::prelude::*;

// use week_preparation_kit::kits::comparison_sorting;

// fn main() {

//     let stdin = io::stdin(); 
    
//     let mut input_iter = stdin.lock().lines();

//     println!("Enter Matrix size: ");
//     let matrix_size = input_iter.next().unwrap().unwrap().parse::<i32>().unwrap();

//     let mut matrix = Vec::with_capacity(matrix_size as usize);

//     matrix = input_iter.next().unwrap().unwrap()
//                         .trim()
//                         .split(" ")
//                         .map(|x| x.parse::<i32>().unwrap())
//                         .collect();
    

//     let res = comparison_sorting::run(&matrix);

//     println!("Lonely integer => {:?}", res);   
// }

