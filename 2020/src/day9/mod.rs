use std::collections::VecDeque;
use std::cmp::{min, max};

const ARRAY_LEN: usize = 25;

pub fn day9p1() -> Option<usize> {
    let input: Vec<_> = include_str!("input.txt").lines().collect();
    let mut rolling: VecDeque<usize> = VecDeque::new();

    for line in input.iter() {
        let val: usize = line.parse().unwrap();

        if rolling.len() < ARRAY_LEN {
            rolling.push_back(val);
        } else {
            let mut found = false;
            'find_loop: for i in 0..rolling.len() {
                for j in i..rolling.len() {
                    if rolling.get(i).unwrap() + rolling.get(j).unwrap() == val {
                        found = true;
                        break 'find_loop;
                    }
                }
            }

            if found {
                rolling.pop_front();
                rolling.push_back(val);
            } else {
                return Some(val);
            }
        }
    }

    None
}

pub fn day9p2() -> Option<usize> {
    let input: Vec<_> = include_str!("input.txt").lines().collect();
    if let Some(invalid) = day9p1() {
        for i in 0..input.len() {
            let start_val: usize = input[i].parse().unwrap();
            let mut sum = start_val;
            let mut min_val = start_val;
            let mut max_val = start_val;
            for j in (i + 1)..input.len() {
                let end_val: usize = input[j].parse().unwrap();
                min_val = min(min_val, end_val);
                max_val = max(max_val, end_val);
                sum += end_val;
                if sum == invalid {
                    return Some(min_val + max_val);
                } else if sum > invalid {
                    break;
                }
            }
        }
    }

    None
}
