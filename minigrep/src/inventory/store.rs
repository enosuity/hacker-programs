use std::collections::HashMap;
use std::io::{self, Read};
use std::error::Error;


#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum ShirtColor {
    Red,
    Blue,
    Green,
    White,
}

#[derive(Debug)]
pub struct Inventory {
  shirts: Vec<ShirtColor>,
  grouping: HashMap<ShirtColor, i8>,
}

impl Inventory {
  pub fn build(colors: &[ShirtColor]) -> Self {    
    Inventory {
      shirts: colors.to_vec(),
      grouping: Self::colors_categories(colors),
    }
  }

  pub fn entry(&self, color_choice: &str) -> Result<ShirtColor, Box<dyn Error>> {
    let choice: ShirtColor = match color_choice {
      "red" => ShirtColor::Red,
      "green" => ShirtColor::Green,
      "blue" => ShirtColor::Blue,
      "white" => ShirtColor::White,
      _ => return Err("Invalid color".into()),
    };

    Ok(choice)
  }

  pub fn gateway(&self) -> Option<ShirtColor> {
    let mut max = std::i8::MIN;
    let mut max_key= None;
    
    for (key, &val) in &self.grouping {
      if val > max {
        max = val;
        max_key = Some(key.clone());
      } 
    }

    max_key
  }

  fn colors_categories(colors: &[ShirtColor]) -> HashMap<ShirtColor, i8> {
    let mut categories = HashMap::new();

    colors.iter().for_each(|item| {
      let mut counter = categories.entry(item.clone()).or_insert(0);
      *counter += 1;
    });

    categories
  }
}

