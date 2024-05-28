
#[cfg(feature="min_and_max")]
pub fn perform(list: &[i64]) -> (i64, i64) {

  let res: Vec<i64> = list.iter().enumerate().fold(Vec::new(), |mut res, (index, &x)| {
    let mut existing = list.clone().to_vec();
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