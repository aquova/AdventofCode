use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

// I made it to the end of the problem before noticing that 'Ticket' is a bad name for this
// But oh well.
struct Ticket {
    idx: Option<usize>,
    lower: RangeInclusive<u32>,
    higher: RangeInclusive<u32>,
}

impl Ticket {
    pub fn new(ll: u32, lh: u32, hl: u32, hh: u32) -> Self {
        Self {
            idx: None,
            lower: RangeInclusive::new(ll, lh),
            higher: RangeInclusive::new(hl, hh),
        }
    }

    pub fn contains(&self, val: &u32) -> bool {
        self.lower.contains(val) || self.higher.contains(val)
    }

    pub fn set_idx(&mut self, idx: usize) {
        self.idx = Some(idx);
    }

    pub fn is_missing_idx(&self) -> bool {
        self.idx.is_none()
    }

    pub fn get_idx(&self) -> Option<usize> {
        self.idx
    }
}

pub fn day16p1() -> Option<u32> {
    let input = include_str!("input.txt");
    let sections: Vec<_> = input.split("\n\n").collect();
    let valid_re = Regex::new(r"^.+: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut ticket_ranges: Vec<RangeInclusive<u32>> = Vec::new();
    for line in sections[0].lines() {
        let caps = valid_re.captures(line).unwrap();
        let lower_lower: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let lower_higher: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let higher_lower: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let higher_higher: u32 = caps.get(4).unwrap().as_str().parse().unwrap();
        let lower = RangeInclusive::new(lower_lower, lower_higher);
        let higher = RangeInclusive::new(higher_lower, higher_higher);
        ticket_ranges.push(lower);
        ticket_ranges.push(higher);
    }

    ticket_ranges.sort_by(|a, b| a.end().cmp(&b.end()));

    let mut error_rate = 0;
    let mut ticket_lines = sections[2].lines();
    // Skip the first line, it's just a header
    ticket_lines.next();
    for line in ticket_lines {
        let vals: Vec<u32> = line.split(",").map(|v| v.parse().unwrap()).collect();
        for v in vals.iter() {
            let mut found = false;
            for t in ticket_ranges.iter() {
                if t.contains(&v) {
                    found = true;
                    break;
                } else if t.end() > v {
                    break;
                }
            }

            if !found {
                error_rate += v;
            }
        }
    }

    Some(error_rate)
}

pub fn day16p2() -> Option<usize> {
    let input = include_str!("input.txt");
    let sections: Vec<_> = input.split("\n\n").collect();
    let valid_re = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    // Populate hashmap of names to ticket objects
    let mut tickets: HashMap<String, Ticket> = HashMap::new();
    for line in sections[0].lines() {
        let caps = valid_re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let lower_lower: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let lower_higher: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let higher_lower: u32 = caps.get(4).unwrap().as_str().parse().unwrap();
        let higher_higher: u32 = caps.get(5).unwrap().as_str().parse().unwrap();
        let ticket = Ticket::new(lower_lower, lower_higher, higher_lower, higher_higher);
        tickets.insert(name, ticket);
    }

    // Iterate through and only keep data we know to be valid (has no value that belongs to no range at all)
    let mut valid_data: Vec<Vec<u32>> = Vec::new();
    let mut ticket_lines = sections[2].lines();
    // Skip the first line, it's just a header
    ticket_lines.next();
    for line in ticket_lines {
        let vals: Vec<u32> = line.split(",").map(|v| v.parse().unwrap()).collect();
        let mut valid = true;
        for v in vals.iter() {
            let mut found = false;
            for (_, t) in tickets.iter() {
                if t.contains(v) {
                    found = true;
                    break;
                }
            }
            if !found {
                valid = false;
                break;
            }
        }
        if valid {
            valid_data.push(vals);
        }
    }

    // Go thru each column at a time, trying to find which Ticket matches that column
    // Repeat until all indices are known
    let mut available_idx: Vec<bool> = vec![true; valid_data[0].len()];
    loop {
        let mut possible_fields: Vec<String> = Vec::new();
        for k in tickets.keys() {
            possible_fields.push(k.to_string());
        }

        let mut not_finished = false;
        for a in available_idx.iter() {
            not_finished |= a;
        }

        if !not_finished {
            break;
        }

        for i in 0..valid_data[0].len() {
            if !available_idx[i] {
                continue;
            }
            let mut fields = possible_fields.clone();
            for j in 0..valid_data.len() {
                let val = valid_data[j][i];
                fields.retain(|v| {
                    let t = tickets.get(v).unwrap();
                    t.is_missing_idx() && t.contains(&val)
                });
                if fields.len() == 1 {
                    let match_name = (&fields[0]).to_string();
                    tickets.get_mut(&match_name).unwrap().set_idx(i);
                    available_idx[i] = false;
                }
            }
        }
    }

    // Grab the relevant sections in our own ticket and multiply together
    let mut solution = 1;
    let targets: [String; 6] = [
        "departure location".to_string(),
        "departure station".to_string(),
        "departure platform".to_string(),
        "departure track".to_string(),
        "departure date".to_string(),
        "departure time".to_string(),
    ];

    let my_ticket: Vec<usize> = sections[1].lines().nth(1).unwrap().split(",").map(|v| v.parse().unwrap()).collect();

    for t in targets.iter() {
        let data = tickets.get(t).unwrap();
        let idx = data.get_idx().unwrap();
        solution *= my_ticket[idx];
    }

    Some(solution)
}
