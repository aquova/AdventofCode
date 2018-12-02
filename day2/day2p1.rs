// Advent of Code 2018 - Day 2
// Austin Bricker

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut double_total: u32 = 0;
    let mut triple_total: u32 = 0;

    // Read in file
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    // Iterate thru each line
    for (_, line) in file.lines().enumerate() {
        let mut double_found: bool = false;
        let mut triple_found: bool = false;

        let l = line.unwrap();
        let mut map = HashMap::new();

        // Iterate through line, storing occurance of characters
        for letter in l.chars() {
            let count = map.entry(letter).or_insert(0);
            *count += 1;
        }

        // Once done, see if that line had any triple or double characters
        for (_key, value) in &map {
            if *value == 2 && !double_found {
                double_found = true;
                double_total += 1;
            } else if *value == 3 && !triple_found {
                triple_found = true;
                triple_total += 1;
            }
        }

    }

    println!("Double Total: {}", double_total);
    println!("Triple Total: {}", triple_total);
    println!("Checksum: {}", double_total * triple_total);
}
