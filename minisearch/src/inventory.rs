pub mod store;
pub mod custom_closures;

// zip iterator - usage

pub fn vec_mixing() {
  let names = vec!["Chunnu", "Munnu", "Gunnu"];
    let ages = vec![12, 35, 45];
    let marks = vec![562, 458, 521];

    let groups: Vec<_> = names.iter()
                              .zip(ages.iter())
                              .zip(marks.iter())
                              .map(|((name, age), mark)| (name, age, mark))    
                              .collect();

    println!("grouping ====> {:?}", groups);
}