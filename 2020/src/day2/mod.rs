use crate::utils::*;

pub fn day2p1() -> Option<u32> {
    let mut total = 0;
    let input = read_lines("src/day2/input.txt");
    for line in input.iter() {
        let mut occurances = 0;
        let words: Vec<&str> = line.split(" ").collect();
        let range: Vec<&str> = words[0].split("-").collect();
        let min: u32 = range[0].parse().unwrap();
        let max: u32 = range[1].parse().unwrap();
        let target = words[1].chars().nth(0).unwrap();

        for c in words[2].chars() {
            if c == target {
                occurances += 1;
            }
        }

        if min <= occurances && occurances <= max {
            total += 1;
        }
    }

    Some(total)
}

pub fn day2p2() -> Option<u32> {
    let mut total = 0;
    let input = read_lines("src/day2/input.txt");
    for line in input.iter() {
        let mut occurances = 0;
        let words: Vec<&str> = line.split(" ").collect();
        let range: Vec<&str> = words[0].split("-").collect();
        let left: usize = range[0].parse().unwrap();
        let right: usize = range[1].parse().unwrap();
        let target = words[1].chars().nth(0).unwrap();

        if let Some(c) = words[2].chars().nth(left - 1) {
            if c == target {
                occurances += 1;
            }
        }

        if let Some(c) = words[2].chars().nth(right - 1) {
            if c == target {
                occurances += 1;
            }
        }

        if occurances == 1 {
            total += 1;
        }
    }

    Some(total)
}
