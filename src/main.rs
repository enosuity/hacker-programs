use std::io::Cursor;

use week_preparation_kit::kits::proportion;

fn main() {
    let input = "1 54 63 -4 0 -15\n";
    let mut cursor = Cursor::new(input);
    let res = proportion::vec_elements_proportion(&mut cursor);   
    dbg!(res);
}

