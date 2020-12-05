use crate::utils::read_lines;

use std::cmp::max;

const NUM_ROWS: u32 = 128;
const NUM_COLS: u32 = 8;

pub fn day5p1() -> Option<u32> {
    let input = read_lines("src/day5/input.txt");
    let mut highest = 0;

    for line in input.iter() {
        let id = calc_id(line);
        highest = max(highest, id);
    }

    Some(highest)
}

pub fn day5p2() -> Option<u32> {
    let input = read_lines("src/day5/input.txt");
    let mut seats: Vec<bool> = vec![true; (NUM_ROWS * NUM_COLS) as usize];

    for line in input.iter() {
        let id = calc_id(line);
        seats[id as usize] = false;
    }

    for (idx, seat) in seats.iter().enumerate() {
        if *seat {
            if idx == 0 || idx == seats.len() {
                continue;
            } else if seats[idx - 1] || seats[idx + 1] {
                continue;
            } else {
                return Some(idx as u32);
            }
        }
    }

    None
}

fn calc_id(line: &str) -> u32 {
    let mut row = 0;
    let mut col = 0;
    for i in 0..7 {
        let c = line.chars().nth(i).unwrap();
        let diff = NUM_ROWS / (2 << i);
        if c == 'B' {
            row += diff;
        }
    }

    for i in 7..10 {
        let c = line.chars().nth(i).unwrap();
        let diff = NUM_COLS / (2 << (i - 7));
        if c == 'R' {
            col += diff;
        }
    }

    col + row * NUM_COLS
}
