use regex::Regex;
use std::collections::HashMap;

trait Bitwise {
    fn set_bit(&mut self, bit: usize);
    fn clear_bit(&mut self, bit: usize);
}

impl Bitwise for u64 {
    fn set_bit(&mut self, bit: usize) {
        let mut mask = 0b1;
        mask <<= bit;
        *self |= mask;
    }

    fn clear_bit(&mut self, bit: usize) {
        let mut mask = 0b1;
        mask <<= bit;
        *self &= !mask;
    }
}

pub fn day14p1() -> Option<u64> {
    let input = include_str!("input.txt").lines();
    let mask_re = Regex::new(r"^mask = (.+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut ram: HashMap<usize, u64> = HashMap::new();

    let mut set_mask: u64 = 0;
    let mut clear_mask: u64 = 0;
    for line in input {
        match &line[0..3] {
            "mas" => {
                let caps = mask_re.captures(line).unwrap();
                let mask = caps.get(1).unwrap().as_str();
                set_mask = 0;
                clear_mask = 0;
                for c in mask.chars() {
                    set_mask = set_mask.wrapping_shl(1);
                    clear_mask = clear_mask.wrapping_shl(1);
                    // Set mask - 1 -> 1, 0 & X -> 0
                    // Clear mask - 0 -> 0, 1 & X -> 1
                    match c {
                        'X' => {
                            set_mask |= 0;
                            clear_mask |= 1;
                        },
                        '1' => {
                            set_mask |= 1;
                            clear_mask |= 1;
                        },
                        '0' => {
                            set_mask |= 0;
                            clear_mask |= 0;
                        },
                        _ => unreachable!()
                    }
                }
            },
            "mem" => {
                let caps = mem_re.captures(line).unwrap();
                let addr: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let mut val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
                val |= set_mask;
                val &= clear_mask;
                ram.insert(addr, val);
            },
            _ => unreachable!()
        }
    }

    let mut sum = 0;
    for (_, val) in ram {
        sum += val;
    }

    Some(sum)
}

pub fn day14p2() -> Option<u64> {
    let input = include_str!("input.txt").lines();
    let mask_re = Regex::new(r"^mask = (.+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut ram: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";
    for line in input {
        match &line[0..3] {
            "mas" => {
                let caps = mask_re.captures(line).unwrap();
                mask = caps.get(1).unwrap().as_str();
            },
            "mem" => {
                let caps = mem_re.captures(line).unwrap();
                let raw_addr: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
                let val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
                let mut addrs: Vec<u64> = Vec::new();
                addrs.push(raw_addr);
                for (i, c) in mask.chars().rev().enumerate() {
                    match c {
                        '1' => {
                            for a in addrs.iter_mut() {
                                a.set_bit(i);
                            }
                        },
                        'X' => {
                            for j in 0..addrs.len() {
                                let mut a = addrs[j];
                                let mut b = a;
                                a.set_bit(i);
                                b.clear_bit(i);
                                addrs[j] = a;
                                addrs.push(b);
                            }

                        },
                        // 0 also causes no change
                        _ => ()
                    }
                }
                for a in addrs {
                    ram.insert(a, val);
                }
            },
            _ => ()
        }
    }

    let mut sum = 0;
    for (_, val) in ram {
        sum += val;
    }

    Some(sum)
}
