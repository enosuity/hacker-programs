#![allow(unused)]
#[cfg(feature = "tower-breaker-winner")]

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn execute(num: i32, height: i32) -> i32 { 
  if height == 1 || num % 2 == 0 {
    2
  } else {
    1
  }
  // let mut list = vec![height as i32; num as usize];
  // let stopper = vec![1; num as usize];
  // let mut counter: i32 = 0;
  // let mut winner = vec![];

  // loop {
  //   for (index, item) in (1..=num).enumerate() {
  //     let optimal_div = max_divisors(list[index]);
  //     list[index] = optimal_div;
  //     winner.push(item);
  //     counter += 1;
  //   }
  //   if list == stopper {
  //     counter += 1;
  //     break;
  //   }      
  // }
  // winner.extend_from_slice(&winner.clone());   
  // winner[counter as usize] 
}

// fn max_divisors(height: i32) -> i32 {
//   let mut list: Vec<i32> = vec![];
//   let sqrt_height = (height as f64).sqrt() as i32;

//   for num in 1..=sqrt_height {
//     if height % num == 0 {
//       list.push(num);
//       if num != height / num && (height / num != height) {
//           list.push(height / num);
//       }
//     } 
//   }
//   *list.iter().max().unwrap()
// }


// =================== main.rs ======================

// #![warn(dead_code)]
// use std::{i32, io::{self, BufRead}};

// use week_preparation_kit::kits::tower_breaker;


// fn main() {
//     let stdin = io::stdin();
//     let mut stdin_iterator = stdin.lock().lines();

//     let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

//     for _ in 0..t {
//         let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
//             .split(' ')
//             .map(|s| s.to_string())
//             .collect();

//         let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

//         let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

//         let res = tower_breaker::execute(n, m);
//         println!("{}", res);
//     }
   
// }
