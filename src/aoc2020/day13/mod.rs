pub fn day13p1() -> Option<u32> {
    let input: Vec<_> = include_str!("input.txt").lines().collect();
    let depart_t: u32 = input[0].parse().unwrap();
    let mut ids: Vec<u32> = Vec::new();

    let buses_raw = input[1].split(",");
    for b in buses_raw {
        match b {
            "x" => (),
            _ => {
                let id = b.parse().unwrap();
                ids.push(id);
            }
        }
    }


    let mut min = depart_t;
    let mut min_bus = 0;
    for id in ids.iter() {
        let wait = id - (depart_t % id);
        if wait < min {
            min = wait;
            min_bus = *id;
        }
    }

    Some(min * min_bus)
}

pub fn day13p2() -> Option<u64> {
    let input: Vec<_> = include_str!("input.txt").lines().collect();
    let mut ids: Vec<Option<u64>> = Vec::new();

    let buses_raw = input[1].split(",");
    for b in buses_raw {
        let id = match b {
            "x" => None,
            _ => {
                let bus = b.parse().unwrap();
                Some(bus)
            }
        };
        ids.push(id);
    }

    let mut t = ids[0].unwrap();
    let mut diff_idx = 0;
    let mut diff = t;
    loop {
        let mut found = true;
        for i in (diff_idx + 1)..ids.len() {
            if let Some(bus) = ids[i] {
                if (t + i as u64) % bus != 0 as u64 {
                    found = false;
                    break;
                }
                // Everytime we find a longer sequential match,
                // we can start jumping by the LCM of those values
                diff_idx += 1;
                diff *= bus;
            } else {
                diff_idx += 1;
            }
        }

        if found {
            return Some(t);
        }

        t += diff;
    }
}
