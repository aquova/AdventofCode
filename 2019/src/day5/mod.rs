use crate::intcode::Intcode;
use crate::utils::parse_csv_as_i32;

pub fn day5() -> Option<i32> {
    let input = parse_csv_as_i32("src/day5/input.txt");
    let mut ic = Intcode::new();
    ic.reset(input);
    let output = ic.run();
    Some(output)
}
