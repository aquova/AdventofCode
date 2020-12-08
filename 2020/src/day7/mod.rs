extern crate regex;

use crate::utils::read_lines;
use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn day7p1() -> Option<usize> {
    let input = read_lines("src/day7/input.txt");
    let line_re = Regex::new(r"^(.+) contain (.+)\.$").unwrap();
    let bag_re = Regex::new(r"^\d?\s?(.+) bag").unwrap();
    let target = String::from("shiny gold");

    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    let mut found: HashSet<String> = HashSet::new();

    for line in input.iter() {
        let caps = line_re.captures(line).unwrap();

        let outer = caps.get(1).unwrap().as_str();
        let outerb = bag_re.captures(outer).unwrap();
        let val = outerb.get(1).unwrap().as_str();

        let inner = caps.get(2).unwrap().as_str();
        let innerb = inner.split(", ");
        for i in innerb {
            let b = bag_re.captures(i).unwrap();
            let key = b.get(1).unwrap().as_str();
            match bags.get_mut(key) {
                Some(b) => {
                    b.push(val.to_string());
                },
                None => {
                    bags.insert(key.to_string(), vec![val.to_string()]);
                }
            }
        }
    }

    bag_id_helper(&target, &mut found, &bags);

    Some(found.len() - 1)
}

pub fn day7p2() -> Option<usize> {
    let input = read_lines("src/day7/input.txt");
    let line_re = Regex::new(r"^(.+) contain (.+)\.$").unwrap();
    let bag_re = Regex::new(r"^(\d?)\s?(.+) bag").unwrap();
    let target = String::from("shiny gold");

    let mut bags: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in input.iter() {
        let caps = line_re.captures(line).unwrap();

        let outer = caps.get(1).unwrap().as_str();
        let outerb = bag_re.captures(outer).unwrap();
        let outer_bag = outerb.get(2).unwrap().as_str();

        let inner = caps.get(2).unwrap().as_str();
        let innerb = inner.split(", ");
        let mut inner_bags: Vec<(usize, String)> = Vec::new();
        for i in innerb {
            let b = bag_re.captures(i).unwrap();
            let num = b.get(1).unwrap().as_str().parse::<usize>();
            if let Ok(n) = num {
                let name = b.get(2).unwrap().as_str();
                inner_bags.push((n, name.to_string()));
            }
        }
        bags.insert(outer_bag.to_string(), inner_bags);
    }

    let out = bag_sum_helper(&target, &bags);

    Some(out - 1)
}

fn bag_id_helper(bag: &String, set: &mut HashSet<String>, map: &HashMap<String, Vec<String>>) {
    if set.contains(bag) {
        return;
    }

    set.insert((*bag).clone());

    if let Some(bags) = map.get(bag) {
        for b in bags.iter() {
            bag_id_helper(b, set, map);
        }
    }
}

fn bag_sum_helper(bag: &String, map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let bags = map.get(bag).unwrap();
    if bags.len() == 0 {
        1
    } else {
        let mut total = 1;
        for b in bags.iter() {
            total += b.0 * bag_sum_helper(&b.1, map);
        }
        total
    }
}
