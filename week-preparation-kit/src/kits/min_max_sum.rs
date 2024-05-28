
#[cfg(feature="min_and_max")]

pub fn perform(list: &[i64]) -> (i64, i64) {
  let res: Vec<i64> = list.iter().enumerate().fold(Vec::new(), |mut res, (index, _)| {
    let mut existing = list.to_vec();
    existing.remove(index);
    res.push(existing.iter().sum());    
    res    
  });

  if let (Some(&min), Some(&max)) = (res.iter().min(), res.iter().max()) {
    (max, min)
  } else {
    (0, 0)
  }  
}

// // to test in the main function replace with following code.
// fn main() {
//   let input = vec![5, 1, 5, 50, 510];

//   dbg!(min_max_sum::perform(&input));
// }