use std::collections::HashMap;

const P1_TARGET: usize = 2020;
const P2_TARGET: usize = 30_000_000;

pub fn day15p1() -> Option<usize> {
    let mut nums: Vec<usize> = vec![0,14,1,3,7,9];
    let mut idx = nums.len() - 1;

    while idx < P1_TARGET {
        let target = nums[idx];
        let mut found_idx = 0;
        for i in (0..idx).rev() {
            if nums[i] == target {
                found_idx = idx - i;
                break;
            }
        }

        nums.push(found_idx);
        idx += 1;
    }

    Some(nums[P1_TARGET - 1])
}

pub fn day15p2() -> Option<usize> {
    let mut nums: HashMap<usize, usize> = HashMap::new();
    let input = [0, 14, 1, 3, 7]; // This should be the input, minus the final value, which goes in last (below)
    for (i, v) in input.iter().enumerate() {
        nums.insert(*v, i);
    }

    let mut idx = input.len();
    let mut last = 9;

    while idx < P2_TARGET - 1 {
        if let Some(last_seen) = nums.get(&last) {
            let diff = idx - last_seen;
            nums.insert(last, idx);
            last = diff;
        } else {
            nums.insert(last, idx);
            last = 0;
        }

        idx += 1;
    }

    Some(last)
}
