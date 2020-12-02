use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn parse_lines_as_u32(filename: &str) -> Vec<u32> {
    let mut input: Vec<u32> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let my_int = line.unwrap().parse::<u32>().unwrap();
        input.push(my_int);
    }

    input
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        input.push(line.unwrap());
    }

    input
}
