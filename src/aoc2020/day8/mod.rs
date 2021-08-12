use crate::utils::read_lines;

pub fn day8p1() -> Option<i32> {
    let input: Vec<_> = include_str!("input.txt").lines().collect();
    let mut acc = 0;
    let mut pc = 0;
    let mut visited: Vec<usize> = Vec::new();

    loop {
        if visited.contains(&pc) {
            break;
        } else {
            visited.push(pc);
        }
        let line = &input[pc];
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "acc" => {
                let diff: i32 = words[1].parse().unwrap();
                acc += diff;
                pc += 1;
            },
            "jmp" => {
                let rel: isize = words[1].parse().unwrap();
                let new_pc = (pc as isize) + rel;
                pc = new_pc as usize;
            },
            _ => {
                // Includes nop
                pc += 1;
            }
        }
    }

    Some(acc)
}

pub fn day8p2() -> Option<i32> {
    let input = read_lines("src/day8/input.txt");

    let mut modified_pc = 0;
    while modified_pc < input.len() {
        let mut acc = 0;
        let mut pc = 0;
        let mut visited: Vec<usize> = Vec::new();

        loop {
            if pc >= input.len() {
                return Some(acc);
            }

            if visited.contains(&pc) {
                break;
            } else {
                visited.push(pc);
            }

            let line = &input[pc];
            let words: Vec<&str> = line.split(" ").collect();

            let op = if pc == modified_pc {
                if words[0] == "jmp" {
                    "nop"
                } else if words[0] == "nop" {
                    "jmp"
                } else {
                    words[0]
                }
            } else {
                words[0]
            };

            match op {
                "acc" => {
                    let diff: i32 = words[1].parse().unwrap();
                    acc += diff;
                    pc += 1;
                },
                "jmp" => {
                    let rel: isize = words[1].parse().unwrap();
                    let new_pc = (pc as isize) + rel;
                    pc = new_pc as usize;
                },
                _ => {
                    // Includes nop
                    pc += 1;
                }
            }
        }

        modified_pc += 1;
    }

    None
}
