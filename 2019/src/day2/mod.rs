use crate::intcode::Intcode;
use crate::utils::parse_csv_as_i32;

const P2_TARGET: i32 = 19690720;

pub fn day2p1() -> Option<i32> {
    let input = parse_csv_as_i32("src/day2/input.txt");
    let mut ic = Intcode::new();
    ic.reset(input);
    let output = ic.run();
    Some(output)
}

pub fn day2p2() -> Option<i32> {
    let input = parse_csv_as_i32("src/day2/input.txt");
    let mut ic = Intcode::new();

    for i in 0..100 {
        for j in 0..100 {
            ic.reset(input.clone());
            ic.write_ram(1, i);
            ic.write_ram(2, j);
            let output = ic.run();

            if output == P2_TARGET {
                return Some(100 * i + j);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2p1() {
        assert_eq!(day2p1(), Some(3654868));
    }

    #[test]
    fn test_day2p2() {
        assert_eq!(day2p2(), Some(7014));
    }
}
