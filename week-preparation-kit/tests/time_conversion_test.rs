use week_preparation_kit::kits::time_conversion;

#[test]
fn test_time_conversion_with_am() {
  let input_time = "12:12:12AM";
  let expected_time = "00:12:12";

  assert_eq!(time_conversion::run(input_time), expected_time);
}

#[test]
fn test_time_conversion_with_pm() {
  let input_time = "12:12:12PM";
  let expected_time = "12:12:12";

  assert_eq!(time_conversion::run(input_time), expected_time);
}


#[test]
fn test_time_conversion_with_pm_over() {
  let input_time = "07:12:12PM";
  let expected_time = "19:12:12";
  assert_eq!(time_conversion::run(input_time), expected_time);
}