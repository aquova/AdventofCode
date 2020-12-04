use std::fs::File;
use std::io::{BufReader, BufRead};

enum Direc {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

struct Point<T> {
    pub x: T,
    pub y: T,
}

struct Wire {
    dist: i32,
    ang: Direc,
}

impl Wire {
    pub fn follow(&self, p: Point<i32>) -> Point<i32> {
        match self.ang {
            Direc::NORTH => { Point { x: p.x, y: p.y - self.dist } },
            Direc::SOUTH => { Point { x: p.x, y: p.y + self.dist } },
            Direc::EAST => { Point { x: p.x + self.dist, y: p.y } },
            Direc::WEST => { Point { x: p.x - self.dist, y: p.y } },
        }
    }
}

pub fn day3p1() -> Option<u32> {
    let mut wire1: Vec<Wire> = Vec::new();
    let mut wire2: Vec<Wire> = Vec::new();

    let file = File::open("src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);
    None
}

fn manhattan_dist(p1: Point<i32>, p2: Point<i32>) -> u32 {
    let dx = (p1.x - p2.x).abs();
    let dy = (p1.y - p2.y).abs();
    (dx + dy) as u32
}
