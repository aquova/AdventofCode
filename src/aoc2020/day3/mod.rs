use crate::utils::*;

pub fn day3p1() -> Option<usize> {
    let input = read_lines("src/day3/input.txt");
    Some(tree_check_helper(&input, 3, 1))
}

pub fn day3p2() -> Option<usize> {
    let mut tree_prod: usize = 1;
    let input = read_lines("src/day3/input.txt");

    tree_prod *= tree_check_helper(&input, 1, 1);
    tree_prod *= tree_check_helper(&input, 3, 1);
    tree_prod *= tree_check_helper(&input, 5, 1);
    tree_prod *= tree_check_helper(&input, 7, 1);
    tree_prod *= tree_check_helper(&input, 1, 2);

    Some(tree_prod)
}

fn tree_check_helper(input: &[String], dx: usize, dy: usize) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    let wrapping_width = input[0].len();

    while y < input.len() {
        let c = input[y].chars().nth(x).unwrap();
        if c == '#' {
            trees += 1;
        }
        x = (x + dx) % wrapping_width;
        y += dy;
    }

    trees
}
