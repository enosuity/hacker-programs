use week_preparation_kit::kits::lonely_integer;

#[test]
fn test_lonely_integer() {
  let input = vec![45, 12,56,89,54,56,54,21,12,89,45,45];
  let expected_output = 21;
  let res = lonely_integer::run(&input);
  assert_eq!(res, expected_output);
}

#[test]
fn test_lonely_integer_zero() {
  let input = vec![];
  let expected_output = 0;
  let res = lonely_integer::run(&input);
  assert_eq!(res, expected_output);
}

#[test]
fn test_lonely_integer_multiple_uniq() {
  let input = vec![1,2,2,8,1];
  let expected_output = 8;
  let res = lonely_integer::run(&input);
  assert_eq!(res, expected_output);
}