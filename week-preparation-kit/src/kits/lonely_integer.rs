#![allow(dead_code)]
#[cfg(feature = "lonely-int")]

use std::collections::HashMap;

pub fn run(a: &[i32]) -> i32 {  
  let hs: HashMap<i32, i32> = a.iter().fold(HashMap::new(), |mut hs, &x|{
    let counter = hs.entry(x).or_insert(0);
    println!("counter ==> {}", counter);
    *counter += 1;
    hs 
  });

  for (&k, &v) in &hs {
    if v == 1 {
      return k;
    }
  }
  0
}


// =============== how to use main code ==============


// use std::io;
// use std::io::prelude::*;

// use week_preparation_kit::kits::lonely_integer;

// fn main() {

//     let stdin = io::stdin();
    
//     println!("Please Enter  vector elements:");
//     let mut input_iter = stdin.lock().lines();

//     let mut elements = input_iter.next().unwrap().unwrap();

//     let arr: Vec<i32> = elements.trim_end()
//             .split(" ")
//             .map(|elem| elem.to_string().parse::<i32>().unwrap())
//             .collect();

//     println!("arr => {:?}", arr);

//     let res = lonely_integer::run(&arr);

//     println!("Lonely integer => {:?}", res);   
// }

