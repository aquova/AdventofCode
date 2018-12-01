// Advent of Code 2018 - Day 1
// Austin Bricker

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Welcome to "I'm terrible at Rust: The Program"
fn main() {
    let mut total: i32 = 0;
    let mut vec = Vec::new();
    let mut needFreq: bool = true;
    // In case duplicate isn't found within loop, keep looping
    for x in 0..1000 {
        // Read in file
        let f = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
        // Iterate thru each line
        for (_num, line) in file.lines().enumerate() {
            let l = line.unwrap(); //.parse().unwrap();
            // Need to check if line begins with +/-
            let first = l.chars().next().unwrap();
            let slice = &l[1..];
            let n: i32 = slice.parse().unwrap();

            if first == '+' {
                total += n;
            } else if first == '-'{
                total -= n;
            }

            if needFreq {
                // Check if that total has already appeared
                for i in 0..vec.len() {
                    if vec[i] == total {
                        println!("A duplicate was found: {}", total);
                        needFreq = false;
                    }
                }
            }
            vec.push(total);
        }

        // If duplicate found, end
        if !needFreq {
            break;
        }
        println!("Iteration {}", x);
    }
    println!("Total value: {}", total);
}
