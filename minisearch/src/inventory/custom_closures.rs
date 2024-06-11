
// FnOnce Implementation with own trait i.e unwrap_or_else

pub trait OptionExt<T> {
  fn extract_value_or_else<F>(self, f: F) -> T
  where 
      F: FnOnce() -> T;
  
}

// Implement the trait for Option<T>
impl<T> OptionExt<T> for Option<T> {
  fn extract_value_or_else<F>(self, f: F) -> T
  where
      F: FnOnce() -> T,
  {
      match self {
          Some(x) => x,
          None => f(),
      }
  }
}

pub trait VecExt<T> {
  fn extract_vec_or_else<F>(self, f: F) -> Vec<T>
  where 
      F: FnOnce() -> Vec<T>;
}

impl<T> VecExt<T> for Option<Vec<T>>{
  fn extract_vec_or_else<F>(self, f: F) -> Vec<T>
    where 
      F: FnOnce() -> Vec<T> {
      match self {
        Some(x) => x,
        None => f(),          
      }
  }    
}

// let chhora = Some(String::from("Niketan"));
// let t = chhora.extract_value_or_else(|| "Eno".to_string());

// // it will throw an error because chhora variable has been moved in above closure
// let t2 = chhora.extract_value_or_else(|| "Eno Suity".to_string());

// let chhora = None;
// let t = chhora.extract_value_or_else(|| "Eno".to_string());
// println!("{}", t);

// let scales = Some(vec![12, 23, 34, 45, 56]);

// let my_vec = scales.extract_vec_or_else(|| Vec::new());

// println!("my vec ===> {:?}", my_vec);

// let scales: Option<Vec<i32>> = None;

// let my_vec = scales.extract_vec_or_else(|| Vec::new());

// println!("my vec ===> {:?}", my_vec);

// ============================================= examples of FnMut =============



// use minigrep::inventory::{
//   custom_closures::{
//      OptionExt,
//      VecExt
//  }
// };

// #[derive(Debug)]
// struct Student {
//  name: String,
//  age: i8,
//  marks: i32
// }

// fn main() {

//  let mut bca = vec![
//      Student { name: String::from("Sonu"), age: 25i8, marks: 253},
//      Student { name: String::from("Monu"), age: 35i8, marks: 456},
//      Student { name: String::from("Jonu"), age: 21i8, marks: 587},
//      Student { name: String::from("Tinku"), age: 19i8, marks: 387},
//  ];

//  let mut max = 0;

//  let mut st = |x: &Student| {
//      let mks = x.marks;
//      if mks >= 400 {
//          if mks > max { max = mks ;}
//          true
//      } else {
//          false
//      }
//  };

//  let students_with_first_dev: Vec<Student> = bca.into_iter().filter(&mut st).collect();

//  println!("First Division Students: \n {:?}", students_with_first_dev);

//  println!("Topper Marks : {}", max);

//  let mut bcs = students_with_first_dev.iter();
//  println!("student : {:?}", bcs.next());
//  println!("student : {:?}", bcs.next());

//  println!("bcs : {:?}", bcs);  

// }

