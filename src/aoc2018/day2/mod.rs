use std::collections::HashSet;

pub fn day2p1() -> Option<u32> {
    let input = include_str!("input.txt").lines();
    let mut twos = 0;
    let mut threes = 0;
    for line in input {
        let mut one: HashSet<char> = HashSet::new();
        let mut two: HashSet<char> = HashSet::new();
        let mut three: HashSet<char> = HashSet::new();
        for c in line.chars() {
            if one.contains(&c) {
                one.remove(&c);
                two.insert(c);
            } else if two.contains(&c) {
                two.remove(&c);
                three.insert(c);
            } else if three.contains(&c) {
                three.remove(&c);
            } else {
                one.insert(c);
            }
        }
        twos += if two.len() > 0 { 1 } else { 0 };
        threes += if three.len() > 0 { 1 } else { 0 };
    }
    Some(twos * threes)
}

pub fn day2p2() -> Option<u32> {
    let input: Vec<_> = include_str!("input.txt").lines().collect();
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let iter = input[i].chars().zip(input[j].chars());
            let pruned: String = iter.filter(|x| (x.0 == x.1)).map(|x| x.0).collect();
            if pruned.len() == input[i].len() - 1 {
                println!("{}", pruned);
            }
        }
    }
    None
}
