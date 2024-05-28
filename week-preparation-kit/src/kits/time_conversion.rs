#![allow(unused)]
#[cfg(feature="time-conversion")]

pub fn run(user_time: &str) -> String {
  let (time, zone) = user_time.split_at(8);
    
  let mut time_entities: Vec<&str> = time.split(":").collect();    
  let mut hours = time_entities[0].parse::<u8>().unwrap();

  if zone.to_uppercase() == "PM" && hours != 12 {
      hours += 12;
  } else if zone.to_uppercase() == "AM" && hours == 12 {
      hours = 0;
  }
  
  let hr = format!("{:02}", hours);
  time_entities[0] = &hr;
  
  time_entities.join(":")  
}


// // Command to be used to see the output

// TIME_OUTPUT=hello.txt cargo run 


// // below code for main f

// use std::{env, fs::File, io::{self, BufRead, Write}};

// use week_preparation_kit::kits::time_conversion;

// fn main() {
//     let path = env::var("TIME_OUTPUT").unwrap();

//     let stdin = io::stdin();
    
//     println!("Please Enter time (hh:mm:ssAM/PM)");
//     let mut input_iter = stdin.lock().lines();

//     let mut fptr = File::create(path).unwrap();
//     let time = input_iter.next().unwrap().unwrap();

//     let converted_time = time_conversion::run(&time);
    
//     writeln!(&mut fptr, "{}", converted_time).ok();
// }
