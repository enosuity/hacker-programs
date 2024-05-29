use std::{env, fs::File, io::{self, BufRead, Write}};

use week_preparation_kit::kits::lonely_integer;

fn main() {

    let stdin = io::stdin();
    
    println!("Please Enter  vector elements:");
    let mut input_iter = stdin.lock().lines();

    let mut elements = input_iter.next().unwrap().unwrap();

    let arr: Vec<i32> = elements.trim_end()
            .split(" ")
            .map(|elem| elem.to_string().parse::<i32>().unwrap())
            .collect();

    println!("arr => {:?}", arr);

    let res = lonely_integer::run(&arr);

    println!("Lonely integer => {:?}", res);   
}

