trait Reversal {
    fn rev(&self) -> String;
}

impl Reversal for String {
    fn rev(&self) -> String {
        let mut rev_str = String::new();
        for byte in (*self).chars().rev() {
            rev_str.push(byte);
        }
        rev_str
    }
}

#[derive(Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    pub id: usize,
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top_border: Option<usize>,
    pub bot_border: Option<usize>,
    pub left_border: Option<usize>,
    pub right_border: Option<usize>,
}

impl Tile {
    pub fn new(id: usize, top: String, bottom: String, left: String, right: String) -> Self {
        Self {
            id,
            top,
            bottom,
            left,
            right,
            top_border: None,
            bot_border: None,
            left_border: None,
            right_border: None,
        }
    }

    pub fn set_borders(&mut self, other: &Tile) {
        let my_sides = self.get_sides();
        let other_sides = other.get_sides();

        'breakout: for s in my_sides.iter() {
            for o in other_sides.iter() {
                if s.0 == o.0 || s.0.rev() == o.0 {
                    self.set_border(other.id, s.1);
                    break 'breakout;
                }
            }
        }
    }

    pub fn get_sides(&self) -> Vec<(String, Side)> {
        let mut sides: Vec<(String, Side)> = Vec::new();

        sides.push((self.top.clone(), Side::Top));
        sides.push((self.bottom.clone(), Side::Bottom));
        sides.push((self.left.clone(), Side::Left));
        sides.push((self.right.clone(), Side::Right));

        sides
    }

    pub fn set_border(&mut self, id: usize, side: Side) {
        match side {
            Side::Top => self.top_border = Some(id),
            Side::Bottom => self.bot_border = Some(id),
            Side::Left => self.left_border = Some(id),
            Side::Right => self.right_border = Some(id),
        }
    }

    pub fn count_borders(&self) -> usize {
        let mut count = 0;
        if self.top_border.is_some() { count += 1; }
        if self.bot_border.is_some() { count += 1; }
        if self.left_border.is_some() { count += 1; }
        if self.right_border.is_some() { count += 1; }

        count
    }
}

pub fn day20p1() -> Option<usize> {
    let input = include_str!("input.txt");
    let raw_tiles: Vec<_> = input.split("\n\n").collect();
    let mut tiles = parse_tiles(&raw_tiles);
    let corners = find_corners(&mut tiles);

    let mut output = 1;
    for c in corners.iter() {
        output *= c;
    }

    Some(output)
}

fn parse_tiles(raw_tiles: &[&str]) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    for t in raw_tiles {
        let lines: Vec<_> = t.lines().collect();
        let header = lines[0];
        let idx: usize = header[5..(header.len() - 1)].parse().unwrap();
        let top = lines[1].to_string();
        let bottom = lines[lines.len() - 1].to_string();
        let mut left = String::new();
        let mut right = String::new();
        for i in 1..lines.len() {
            let l = lines[i];
            left.push(l.chars().nth(0).unwrap());
            right.push(l.chars().last().unwrap());
        }

        let tile = Tile::new(idx, top, bottom, left, right);
        tiles.push(tile);
    }
    tiles
}

fn find_corners(tiles: &mut [Tile]) -> Vec<usize> {
    let mut corners = Vec::new();
    let len = tiles.len();
    for i in 0..len {
        let my_tile = &mut tiles[i];
        for j in 0..len {
            if *my_tile != tiles[j] {
                my_tile.set_borders(&tiles[j]);
            }
        }

        if tiles[i].count_borders() == 2 {
            corners.push(tiles[i].id);
        }
    }

    corners
}
