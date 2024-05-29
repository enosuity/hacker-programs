#![allow(unused)]
#[cfg(feature = "diagonal-diff")]

pub fn run(matrix: &[Vec<i32>]) -> i32 {
  let (s1, s2) = matrix.iter().enumerate().fold((0, 0), |(mut s1, mut s2), (index, list)| {
    s1 += list.get(index).copied().unwrap_or(0);
    s2 += list.get(list.len() - (index + 1)).copied().unwrap_or(0);
    (s1, s2)
  });
  
  (s1 - s2).abs() as i32
}