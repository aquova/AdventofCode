use std::collections::HashMap;

type RulesType = HashMap<u32, Vec<Vec<Rule>>>;

#[derive(Debug)]
enum Rule {
    Letter(char),
    Ref(u32),
}

pub fn day19p1() -> Option<u32> {
    let input = include_str!("input.txt");
    let sections: Vec<_> = input.split("\n\n").collect();
    let rules = parse_rules(sections[0]);
    let mut count = 0;
    for line in sections[1].lines() {
        let check = traverse_tree(0, 0, line, &rules);
        if check.0 && check.1 >= line.len() {
            count += 1;
        }
    }

    Some(count)
}

fn parse_rules(input: &str) -> RulesType {
    let mut rules: RulesType = HashMap::new();

    for line in input.lines() {
        let first: Vec<_> = line.split(": ").collect();
        let idx: u32 = first[0].parse().unwrap();
        let second = first[1].split("|");

        let mut node: Vec<Vec<Rule>> = Vec::new();
        for r in second {
            let words = r.split(" ");
            let mut foo: Vec<Rule> = Vec::new();
            for w in words {
                match w.parse::<u32>() {
                    Ok(n) => {
                        foo.push(Rule::Ref(n));
                    },
                    Err(_) => {
                        if w != "" {
                            for c in w.chars() {
                                if c.is_alphabetic() {
                                    foo.push(Rule::Letter(c));
                                }
                            }
                        }
                    }
                }
            }
            node.push(foo);
        }
        rules.insert(idx, node);
    }

    rules
}

fn traverse_tree(idx: u32, str_idx: usize, line: &str, rules: &RulesType) -> (bool, usize) {
    let mut found = false;
    let mut new_str_idx = str_idx;
    if let Some(node) = rules.get(&idx) {
        for or_split in node.iter() {
            for (_, n) in or_split.iter().enumerate() {
                match n {
                    Rule::Ref(new_idx) => {
                        let recur = traverse_tree(*new_idx, new_str_idx, line, rules);
                        found = recur.0;
                        if !found {
                            break;
                        }
                        new_str_idx = recur.1;
                    },
                    Rule::Letter(c) => {
                        if let Some(rc) = line.chars().nth(new_str_idx) {
                            if rc == *c {
                                return (true, new_str_idx + 1);
                            } else {
                                return (false, 0);
                            }
                        }
                    }
                }
            }

            if found {
                break;
            }
        }
    }
    (found, new_str_idx)
}
