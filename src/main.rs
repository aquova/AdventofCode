mod aoc2018;
mod aoc2020;
mod utils;

use aoc2018::*;

fn main() {
    if let Some(answer) = day1::day1p2() {
        println!("{}", answer);
    } else {
        println!("No solution found");
    }
}
