use crate::utils::parse_lines_as_u32;

pub fn day1p1() -> Option<u32> {
    let input = parse_lines_as_u32("src/day1/input.txt");
    let mut total = 0;
    for mass in input.iter() {
        let fuel = (mass / 3) - 2;
        total += fuel;
    }

    Some(total)
}

pub fn day1p2() -> Option<u32> {
    let input = parse_lines_as_u32("src/day1/input.txt");
    let mut total = 0;
    for mass in input.iter() {
        let mass = *mass as i32;
        let mut fuel = mass;
        loop {
            fuel = (fuel / 3) - 2;
            if fuel > 0 {
                total += fuel;
            } else {
                break;
            }
        }
    }

    Some(total as u32)
}
