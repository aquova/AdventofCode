#![allow(dead_code)]
mod aoc2018;
mod aoc2020;
mod utils;

use aoc2018::*;

fn main() {
    if let Some(answer) = day2::day2p2() {
        println!("{}", answer);
    } else {
        println!("No solution found");
    }
}
