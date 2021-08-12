extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;

const NEEDED_FIELDS: [&'static str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

pub fn day4p1() -> Option<u32> {
    let input = parse_file("src/day4/input.txt");
    let mut valid = 0;

    for pass in input.iter() {
        if check_all_fields(pass) {
            valid += 1;
        }
    }

    Some(valid)
}

pub fn day4p2() -> Option<u32> {
    let input = parse_file("src/day4/input.txt");
    let mut valid = 0;

    let hgt_re = Regex::new(r"(\d+)(in|cm)").unwrap();
    let hcl_re = Regex::new(r"\#[0-9A-Fa-f]{6}").unwrap();
    let ecl_re = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let pid_re = Regex::new(r"\b\d{9}$").unwrap();

    'pass_loop: for pass in input.iter() {
        if !check_all_fields(pass) {
            continue;
        }

        let fields: Vec<_> = pass.split(" ").collect();
        for f in fields.iter() {
            let data: Vec<&str> = f.split(":").collect();

            match data[0] {
                "byr" => {
                    let year: u32 = data[1].parse().unwrap();
                    if year < 1920 || year > 2002 {
                        continue 'pass_loop;
                    }
                },
                "iyr" => {
                    let year: u32 = data[1].parse().unwrap();
                    if year < 2010 || year > 2020 {
                        continue 'pass_loop;
                    }
                },
                "eyr" => {
                    let year: u32 = data[1].parse().unwrap();
                    if year < 2020 || year > 2030 {
                        continue 'pass_loop;
                    }
                },
                "hgt" => {
                    let caps = hgt_re.captures(data[1]);
                    if let Some(c) = caps {
                        let hgt: u32 = (&c[1]).parse().unwrap();
                        match &c[2] {
                            "in" => {
                                if hgt < 59 || hgt > 76 {
                                    continue 'pass_loop;
                                }
                            },
                            "cm" => {
                                if hgt < 150 || hgt > 193 {
                                    continue 'pass_loop;
                                }
                            },
                            _ => {
                                continue 'pass_loop;
                            }
                        }
                    } else {
                        continue 'pass_loop;
                    }
                },
                "hcl" => {
                    if !hcl_re.is_match(data[1]) {
                        continue 'pass_loop;
                    }
                },
                "ecl" => {
                    if !ecl_re.is_match(data[1]) {
                        continue 'pass_loop;
                    }
                },
                "pid" => {
                    if !pid_re.is_match(data[1]) {
                        continue 'pass_loop;
                    }
                },
                _ => {
                    // Do nothing
                }
            }
        }

        valid +=  1;
    }

    Some(valid)
}

fn check_all_fields(pass: &str) -> bool {
    for s in NEEDED_FIELDS.iter() {
        if !pass.contains(s) {
            return false;
        }
    }

    true
}

fn parse_file(filename: &str) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    let mut curr_str: String = String::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();

        if l.len() == 0 {
            input.push(curr_str.clone());
            curr_str.clear();
        } else {
            curr_str = format!("{} {}", curr_str, l);
        }
    }
    input.push(curr_str);

    input
}
