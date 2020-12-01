use crate::utils::*;

pub fn day1p1() -> Option<u32> {
    let input = parse_lines_as_u32("src/day1/input.txt");
    let mut found: Vec<bool> = vec![false; 2020];

    for i in input.iter() {
        let other = 2020 - i;
        if found[other as usize] {
            return Some(other * i);
        } else {
            found[*i as usize] = true;
        }
    }

    None
}

pub fn day1p2() -> Option<u32> {
    let input = parse_lines_as_u32("src/day1/input.txt");
    let mut found: Vec<bool> = vec![false; 2020];

    for i in 0..input.len() {
        for j in i..input.len() {
            let other = 2020 - (input[i] as i32) - (input[j] as i32);
            if other > 0 {
                let other = other as u32;
                if found[other as usize] {
                    return Some(other * input[i] * input[j]);
                } else {
                    found[input[i] as usize] = true;
                    found[input[j] as usize] = true;
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1p1() {
        assert_eq!(day1p1(), Some(1010299));
    }

    #[test]
    fn test_day1p2() {
        assert_eq!(day1p2(), Some(42140160));
    }
}
