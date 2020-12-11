#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Seats {
    Occupied,
    Empty,
    Floor
}

type Grid = Vec<Vec<Seats>>;

pub fn day11p1() -> Option<u32> {
    let raw = include_str!("input.txt").lines();

    let mut layout = raw.map(|v| {
        let mut row: Vec<Seats> = Vec::new();
        for c in v.chars().into_iter() {
            let s = match c {
                'L' => Seats::Empty,
                '#' => Seats::Occupied,
                _ => Seats::Floor,
            };
            row.push(s);
        }
        row
    }).collect();

    // Keep ticking until we're in equilibrium
    loop {
        let new_layout = tick(&layout, false);
        if new_layout == layout {
            break;
        } else {
            layout = new_layout;
        }
    }

    // Count number of occupied seats
    let mut total = 0;
    for row in layout.iter() {
        for v in row.iter() {
            if *v == Seats::Occupied {
                total += 1;
            }
        }
    }

    Some(total)
}

pub fn day11p2() -> Option<u32> {
    let input = include_str!("input.txt").lines();

    let mut layout = input.map(|v| {
        let mut row: Vec<Seats> = Vec::new();
        for c in v.chars().into_iter() {
            let s = match c {
                'L' => Seats::Empty,
                '#' => Seats::Occupied,
                _ => Seats::Floor,
            };
            row.push(s);
        }
        row
    }).collect();

    // Keep ticking until we're in equilibrium
    loop {
        let new_layout = tick(&layout, true);
        if new_layout == layout {
            break;
        } else {
            layout = new_layout;
        }
    }

    // Count number of occupied seats
    let mut total = 0;
    for row in layout.iter() {
        for v in row.iter() {
            if *v == Seats::Occupied {
                total += 1;
            }
        }
    }

    Some(total)
}

fn tick(prev: &Grid, queen_search: bool) -> Grid {
    let mut next: Grid = Vec::new();
    for y in 0..prev.len() {
        let mut row: Vec<Seats> = Vec::new();
        for x in 0..prev[y].len() {
            row.push(prev[y][x]);
            match prev[y][x] {
                Seats::Empty => {
                    if queen_search && occupied_queen(x, y, prev) == 0 {
                        row[x] = Seats::Occupied;
                    } else if !queen_search && occupied_neighbors(x, y, prev) == 0 {
                        row[x] = Seats::Occupied;
                    }
                },
                Seats::Occupied => {
                    if queen_search && occupied_queen(x, y, prev) >= 5 {
                        row[x] = Seats::Empty;
                    } else if !queen_search && occupied_neighbors(x, y, prev) >= 4 {
                        row[x] = Seats::Empty;
                    }
                },
                _ => {}
            }
        }
        next.push(row);
    }

    next
}

fn occupied_neighbors(x_pos: usize, y_pos: usize, grid: &Grid) -> usize {
    let mut total = 0;
    let x_i = x_pos as isize;
    let y_i = y_pos as isize;
    for x in (x_i - 1)..=(x_i + 1) {
        for y in (y_i - 1)..=(y_i + 1) {
            // Can't go out of bounds, also don't check ourself
            if x >= 0 && y >= 0 && y < grid.len() as isize && x < grid[0].len() as isize {
                if x != x_i || y != y_i {
                    if grid[y as usize][x as usize] == Seats::Occupied { total += 1 };
                }
            }
        }
    }
    total
}

fn occupied_queen(x_pos: usize, y_pos: usize, grid: &Grid) -> usize {
    let mut total = 0;

    // We'll do this the hard way, rather than trying to be cute to get indices
    for y in (y_pos + 1)..grid.len() {
        match grid[y][x_pos] {
            Seats::Occupied => {
                total += 1;
                break;
            },
            Seats::Empty => break,
            _ => ()
        }
    }

    if y_pos > 0 {
        for y in (0..=(y_pos - 1)).rev() {
            match grid[y][x_pos] {
                Seats::Occupied => {
                    total += 1;
                    break;
                },
                Seats::Empty => break,
                _ => ()
            }
        }
    }

    for x in (x_pos + 1)..grid[y_pos].len() {
        match grid[y_pos][x] {
            Seats::Occupied => {
                total += 1;
                break;
            },
            Seats::Empty => break,
            _ => ()
        }
    }

    if x_pos > 0 {
        for x in (0..=(x_pos - 1)).rev() {
            match grid[y_pos][x] {
                Seats::Occupied => {
                    total += 1;
                    break;
                },
                Seats::Empty => break,
                _ => ()
            }
        }
    }

    for i in 1..grid.len() {
        if (y_pos + i >= grid.len()) || (x_pos + i >= grid[y_pos].len()) {
            break;
        }
        match grid[y_pos + i][x_pos + i] {
            Seats::Occupied => {
                total += 1;
                break;
            },
            Seats::Empty => break,
            _ => ()
        }
    }

    for i in 1..grid.len() {
        if i > x_pos || i > y_pos {
            break;
        }

        match grid[y_pos - i][x_pos - i] {
            Seats::Occupied => {
                total += 1;
                break;
            },
            Seats::Empty => break,
            _ => ()
        }
    }

    for i in 1..grid.len() {
        if y_pos + i >= grid.len() || i > x_pos {
            break;
        }

        match grid[y_pos + i][x_pos - i] {
            Seats::Occupied => {
                total += 1;
                break;
            },
            Seats::Empty => break,
            _ => ()
        }
    }

    for i in 1..grid.len() {
        if x_pos + i >= grid[y_pos].len() || i > y_pos {
            break;
        }

        match grid[y_pos - i][x_pos + i] {
            Seats::Occupied => {
                total += 1;
                break;
            },
            Seats::Empty => break,
            _ => ()
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11p1() {
        assert_eq!(Some(2275), day11p1());
    }

    #[test]
    fn test_day11p2() {
        assert_eq!(Some(2121), day11p2());
    }
}
