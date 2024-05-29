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