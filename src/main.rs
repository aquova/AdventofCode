mod aoc2020;
mod utils;

use aoc2020::*;

fn main() {
    if let Some(answer) = day19::day19p1() {
        println!("{}", answer);
    } else {
        println!("No solution found");
    }
}
