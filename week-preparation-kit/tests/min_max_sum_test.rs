use week_preparation_kit::kits::min_max_sum;

#[test]
fn test_perform() {
  let input = vec![1, 2, 3, 4, 5];
  assert_eq!(min_max_sum::perform(&input), (10, 14));
}
