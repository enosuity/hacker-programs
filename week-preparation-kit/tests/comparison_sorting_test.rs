use week_preparation_kit::kits::comparison_sorting;

#[test]
fn test_counting_sort() {
  let input = vec![1, 1, 3, 2, 1];
  let expected_output = vec![1, 1, 1, 2, 3];
  let res = comparison_sorting::run(&input);
  assert_eq!(res, expected_output);
}