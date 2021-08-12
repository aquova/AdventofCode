use crate::utils::*;

pub fn day6p1() -> Option<u32> {
    let input = read_lines_strip_newlines("src/day6/input.txt");
    let mut total = 0;

    let ascii = (0..26).map(|x| (x + b'a') as char);

    for line in input.iter() {
        for c in ascii.clone() {
            if line.contains(c) {
                total += 1;
            }
        }
    }

    Some(total)
}

pub fn day6p2() -> Option<u32> {
    let input = read_lines("src/day6/input.txt");
    let mut total = 0;
    let mut tally = [true; 26];
    let ascii = (0..26).map(|x| (x + b'a') as char);

    for line in input.iter() {
        if line == "" {
            for i in 0..26 {
                if tally[i] {
                    total += 1;
                }
            }
            tally = [true; 26];
        } else {
            for c in ascii.clone() {
                if !line.contains(c) {
                    tally[((c as u8) - b'a') as usize] = false;
                }
            }
        }
    }

    Some(total)
}
