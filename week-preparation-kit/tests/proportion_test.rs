use week_preparation_kit::kits::proportion::{self, vec_elements_proportion};
use std::io::{Cursor, Read, self, BufRead, Write};

fn prompt<R, W>(mut reader: R, mut writer: W, arr: &str) -> String
where
    R: BufRead,
    W: Write,
{
    write!(&mut writer, "{}", arr).expect("Unable to write");
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}



#[test]
fn test_vec_elements_proportion() {
  // Arrange
  let input = "1 54 63 -4 0 -15\n";
  let expected_output = vec![0.500000, 0.333333, 0.166667];
  let mut cursor = Cursor::new(input);
  let res = vec_elements_proportion(&mut cursor);
  assert_eq!(res, expected_output);
}

