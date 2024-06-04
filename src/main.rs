use minigrep::inventory::store;
use std::io;


fn main() {
   println!("Enter all colors in store: ");
   
   let stdio = io::stdin();
   let mut lines = stdio.lines();
   let inputs = lines.next().unwrap().unwrap().parse::<String>().unwrap();

   let colors: Vec<store::ShirtColor> = inputs.trim()
                    .split(" ")
                    .map(|x| {
                        match x {
                            "red" => store::ShirtColor::Red,
                            "blue" => store::ShirtColor::Blue,
                            "green" => store::ShirtColor::Green,
                            "white" => store::ShirtColor::White,
                            _ => panic!("Invalid color"),
                        }
                    })
                    .collect();

    let inventory = store::Inventory::build(&colors);

    println!("inventory is registered successfully!");

    println!("Enter your color choice: ");
    let choice = lines.next().unwrap().unwrap();

    let choice = inventory.entry(&choice)
                                      .unwrap_or_else(|_err| inventory.gateway().unwrap());

   println!("Your T-shirt color: {:?}", choice);        
}

