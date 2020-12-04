use crate::intcode::Intcode;
use crate::utils::parse_csv_as_u32;

const P2_TARGET: u32 = 19690720;

pub fn day2p1() -> Option<u32> {
    let input = parse_csv_as_u32("src/day2/input.txt");
    let mut ic = Intcode::new();
    ic.reset(input);
    let output = ic.run();
    Some(output)
}

pub fn day2p2() -> Option<u32> {
    let input = parse_csv_as_u32("src/day2/input.txt");
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
