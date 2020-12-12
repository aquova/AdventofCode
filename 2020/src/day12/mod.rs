use regex::Regex;

#[derive(Debug)]
enum Directions {
    North,
    South,
    East,
    West
}

impl Directions {
    pub fn left(&mut self, deg: i32) {
        let ang = self.ang2deg();
        let new_deg = ((ang + 360) - deg) % 360;
        self.deg2ang(new_deg);
    }

    pub fn right(&mut self, deg: i32) {
        let ang = self.ang2deg();
        let new_deg = (ang + deg) % 360;
        self.deg2ang(new_deg);
    }

    fn ang2deg(&self) -> i32 {
        match self {
            Directions::North => 0,
            Directions::South => 180,
            Directions::West => 270,
            Directions::East => 90,
        }
    }

    fn deg2ang(&mut self, deg: i32) {
        match deg {
            0 => *self = Directions::North,
            90 => *self = Directions::East,
            180 => *self = Directions::South,
            270 => *self = Directions::West,
            _ => unreachable!(),
        }
    }
}

trait Rotate {
    fn rotate_waypoint(&mut self, deg: i32);
}

impl Rotate for (i32, i32) {
    fn rotate_waypoint(&mut self, deg: i32) {
        let deg = (deg + 360) % 360;
        let wp = self.clone();
        match deg {
            0 => (),
            90 => {
                self.0 = -wp.1;
                self.1 = wp.0;
            },
            180 => {
                self.0 = -wp.0;
                self.1 = -wp.1;
            },
            270 => {
                self.0 = wp.1;
                self.1 = -wp.0;
            },
            _ => unreachable!(),
        }
    }
}

pub fn day12p1() -> Option<i32> {
    let input = include_str!("input.txt").lines();
    let re = Regex::new(r"^(.)(\d+)$").unwrap();

    let mut curr_dir = Directions::East;
    let mut curr_pos: (i32, i32) = (0, 0);
    for line in input {
        let caps = re.captures(line).unwrap();
        let raw_direc = caps.get(1).unwrap().as_str();
        let dist: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        match raw_direc {
            "L" => curr_dir.left(dist),
            "R" => curr_dir.right(dist),
            "F" => {
                match curr_dir {
                    Directions::West => curr_pos.0 -= dist,
                    Directions::East => curr_pos.0 += dist,
                    Directions::North => curr_pos.1 -= dist,
                    Directions::South => curr_pos.1 += dist,
                }
            },
            "W" => curr_pos.0 -= dist,
            "E" => curr_pos.0 += dist,
            "N" => curr_pos.1 -= dist,
            "S" => curr_pos.1 += dist,
            _ => unreachable!(),
        };
    }

    Some(curr_pos.0.abs() + curr_pos.1.abs())
}

pub fn day12p2() -> Option<i32> {
    let input = include_str!("input.txt").lines();
    let re = Regex::new(r"^(.)(\d+)$").unwrap();

    let mut curr_pos: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, -1);
    for line in input {
        let caps = re.captures(line).unwrap();
        let raw_direc = caps.get(1).unwrap().as_str();
        let dist: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        match raw_direc {
            "F" => {
                for _ in 0..dist {
                    curr_pos.0 += waypoint.0;
                    curr_pos.1 += waypoint.1;
                }
            },
            "R" => waypoint.rotate_waypoint(dist),
            "L" => waypoint.rotate_waypoint(-dist),
            "W" => waypoint.0 -= dist,
            "E" => waypoint.0 += dist,
            "N" => waypoint.1 -= dist,
            "S" => waypoint.1 += dist,
            _ => unreachable!(),
        }
    }

    Some(curr_pos.0.abs() + curr_pos.1.abs())
}
