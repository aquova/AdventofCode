use std::collections::HashSet;

pub fn day1p1() -> Option<u32> {
    let input = include_str!("input.txt").lines();
    let mut freq = 0;
    for line in input {
        let diff = line.parse::<i32>().unwrap();
        freq += diff;
    }

    Some(freq as u32)
}

pub fn day1p2() -> Option<u32> {
    let input = include_str!("input.txt").lines();
    let mut freq = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(freq);
    loop {
        for line in input.clone() {
            let diff = line.parse::<i32>().unwrap();
            freq += diff;
            if !seen.insert(freq) {
                return Some(freq as u32);
            }
        }
    }
}
