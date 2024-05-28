use std::io::Cursor;

use week_preparation_kit::kits::min_max_sum;

fn main() {
    let input = vec![5, 5, 5, 5, 5];

    dbg!(min_max_sum::perform(&input));
}

