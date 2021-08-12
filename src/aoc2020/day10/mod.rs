use std::collections::HashMap;

type P2cache = HashMap<usize, u64>;

pub fn day10p1() -> Option<u32> {
    let raw = include_str!("input.txt").lines();
    let mut input: Vec<u32> = raw.map(|v| v.parse::<u32>().unwrap()).collect();
    input.sort();

    let mut single = 0;
    let mut triple = 1; // Start at 1 since we always add triple at the end
    match input[0] {
        1 => single += 1,
        3 => triple += 1,
        _ => unreachable!()
    }

    for i in 1..input.len() {
        match input[i] - input[i - 1] {
            1 => single += 1,
            3 => triple += 1,
            _ => unreachable!()
        }
    }

    Some(single * triple)
}

pub fn day10p2() -> Option<u64> {
    let raw = include_str!("input.txt").lines();
    let mut nums: Vec<u32> = raw.map(|v| v.parse::<u32>().unwrap()).collect();
    nums.sort();

    // Prepend 0 and append final + 3
    let mut input: Vec<u32> = vec![0];
    input.append(&mut nums);
    let target = input[input.len() - 1] + 3;
    input.push(target);

    let mut known: P2cache = P2cache::new();
    let total = search(0, &input, &mut known);
    Some(total)
}

fn search(idx: usize, input: &[u32], known: &mut P2cache) -> u64 {
    if idx == input.len() - 1 {
        return 1;
    }

    let output = if let Some(v) = known.get(&idx) {
        *v
    } else {
        let mut out = 0;
        for i in (idx + 1)..input.len() {
            if input[i] - input[idx] <= 3 {
                out += search(i, input, known);
            } else {
                break;
            }
        }

        known.insert(idx, out);
        out
    };

    output
}
