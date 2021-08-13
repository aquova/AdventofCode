use regex::Regex;
use std::collections::HashSet;

const GRID_SIZE: usize = 1001;

pub fn day3p1() -> Option<u32> {
    let input = include_str!("input.txt").lines();
    let re = Regex::new(r"^\#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for line in input {
        let captures = re.captures(line).unwrap();
        let x: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let y: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let w: usize = captures.get(3).unwrap().as_str().parse().unwrap();
        let h: usize = captures.get(4).unwrap().as_str().parse().unwrap();

        for dx in x..(x + w) {
            for dy in y..(y + h) {
                grid[dy][dx] += 1;
            }
        }
    }

    let mut total = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if grid[y][x] > 1 {
                total += 1;
            }
        }
    }

    Some(total)
}

pub fn day3p2() -> Option<u32> {
    let input = include_str!("input.txt").lines();
    let re = Regex::new(r"^\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let num_lines = input.clone().collect::<String>().len();
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    let mut invalid: HashSet<usize> = HashSet::new();

    for line in input {
        let captures = re.captures(line).unwrap();
        let idx: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let x: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let y: usize = captures.get(3).unwrap().as_str().parse().unwrap();
        let w: usize = captures.get(4).unwrap().as_str().parse().unwrap();
        let h: usize = captures.get(5).unwrap().as_str().parse().unwrap();

        for dx in x..(x + w) {
            for dy in y..(y + h) {
                if grid[dy][dx] != 0 {
                    invalid.insert(grid[dy][dx]);
                    invalid.insert(idx);
                }
                grid[dy][dx] = idx;
            }
        }
    }

    for i in 1..num_lines {
        if !invalid.contains(&i) {
            return Some(i as u32);
        }
    }
    None
}
