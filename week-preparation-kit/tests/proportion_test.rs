use week_preparation_kit::kits::proportion;
use std::io::{Cursor, Read};

#[test]
fn test_vec_elements_proportion() {
  // Arrange
  let input = "1 54 63 -4 0 -15\n";
  let expected_output = "0.500000\n0.333333\n0.166667\n";
  let mut output = Vec::new();

  // Act
  let input_reader = Cursor::new(input);
  let output_writer = Cursor::new(&mut output);
  let output_writer = &mut output_writer.chain(std::io::sink());

  std::io::stdin().with_reader(input_reader, || {
    std::io::stdout().with_writer(output_writer, || {
      proportion::vec_elements_proportion()
    });
  });

  // Convert output bytes to string
  let result = std::str::from_utf8(&output).unwrap();

  // Assert
  assert_eq!(result, expected_output);

}

