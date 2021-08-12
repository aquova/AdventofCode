use std::collections::HashSet;

const CYCLES: usize = 6;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: Option<isize>,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize, w: Option<isize>) -> Self {
        Self {x: x, y: y, z: z, w: w}
    }

    pub fn gen_neighbors(&self) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();

        for xi in (self.x - 1)..=(self.x + 1) {
            for yi in (self.y - 1)..=(self.y + 1) {
                for zi in (self.z - 1)..=(self.z + 1) {
                    if let Some(w) = self.w {
                        for wi in (w - 1)..=(w + 1) {
                            if xi != self.x || yi != self.y || zi != self.z || wi != w {
                                neighbors.push(Point::new(xi, yi, zi, Some(wi)));
                            }
                        }
                    } else {
                        if xi != self.x || yi != self.y || zi != self.z {
                            neighbors.push(Point::new(xi, yi, zi, None));
                        }
                    }
                }
            }
        }

        neighbors
    }

    pub fn count_neighbors(&self, pts: &[Point]) -> usize {
        let n = self.gen_neighbors();
        let mut count = 0;

        for pt in n.iter() {
            if pts.contains(&pt) {
                count += 1;
            }
        }

        count
    }
}

pub fn day17(part1: bool) -> Option<usize> {
    let input: Vec<_> = include_str!("input.txt").lines().collect();

    let mut points: Vec<Point> = Vec::new();
    for y in 0..input.len() {
        let line: Vec<_> = input[y].chars().collect();
        for x in 0..line.len() {
            if line[x] == '#' {
                if part1 {
                    points.push(Point::new(x as isize, y as isize, 0, None));
                } else {
                    points.push(Point::new(x as isize, y as isize, 0, Some(0)));
                }
            }
        }
    }

    for _ in 0..CYCLES {
        let mut next: Vec<Point> = Vec::new();
        let mut possible_new: HashSet<Point> = HashSet::new();
        for pt in points.iter() {
            for n in pt.gen_neighbors().iter() {
                possible_new.insert(*n);
            }
        }

        for pt in possible_new.iter() {
            let n_count = pt.count_neighbors(&points);
            if points.contains(&pt) {
                if n_count == 2 || n_count == 3 {
                    next.push(*pt);
                }
            } else {
                if n_count == 3 {
                    next.push(*pt);
                }
            }
        }
        points = next;
    }

    Some(points.len())
}
