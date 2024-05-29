use week_preparation_kit::kits::diagonal_difference;

#[test]
fn test_two_by_two() {
  let input = vec![vec![5, 1],vec![8, 9]];
  let expected_output = 5;
  let res = diagonal_difference::run(&input);
  assert_eq!(res, expected_output);
}


#[test]
fn test_three_by_three() {
  let input = vec![vec![1, 5, 1],vec![8, 5, 9], vec![6, 9, 1]];
  let expected_output = 5;
  let res = diagonal_difference::run(&input);
  assert_eq!(res, expected_output);
}

#[test]
fn test_four_by_four() {
  let input = vec![vec![11, 5, 9, 3],vec![4, 8, 5, 9], vec![6, 6, 9, 1], vec![3, 1, 7, 10]];
  let expected_output = 21;
  let res = diagonal_difference::run(&input);
  assert_eq!(res, expected_output);
}