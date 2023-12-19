use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .collect();
        Ok(Self {
            x: splits[0].trim_start_matches("x=").parse().unwrap(),
            m: splits[1].trim_start_matches("m=").parse().unwrap(),
            a: splits[2].trim_start_matches("a=").parse().unwrap(),
            s: splits[3].trim_start_matches("s=").parse().unwrap(),
        })
    }
}

impl Part {
    fn rate(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
    fn valide_part(&self, rules: &HashMap<&str, Vec<&str>>) -> bool {
        let mut current_condition = "in";
        loop {
            if current_condition == "A" {
                return true;
            }
            if current_condition == "R" {
                return false;
            }
            let conditions = rules.get(current_condition).expect("Condition not found");
            let mut found = false;
            for &condition in conditions {
                if condition == "A" {
                    return true;
                }
                if condition == "R" {
                    return false;
                }
                if let Some((cond, nxt)) = condition.split_once(':') {
                    let left_value = match cond.as_bytes()[0] {
                        b'x' => self.x,
                        b'm' => self.m,
                        b'a' => self.a,
                        b's' => self.s,
                        _ => panic!(),
                    };
                    let op = cond.as_bytes()[1] as char;
                    let val = cond[2..]
                        .parse::<usize>()
                        .expect("Condition does not have valid number");
                    if op == '>' {
                        if left_value > val {
                            current_condition = nxt;
                            found = true;
                            break;
                        }
                    } else if op == '<' && left_value < val {
                        current_condition = nxt;
                        found = true;
                        break;
                    }
                } else {
                    current_condition = condition;
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
    }
}

fn main() {
    let (workflows, parts) = include_str!("../data/day_2023_19.data")
        .split_once("\n\n")
        .unwrap();

    let workflows: HashMap<&str, Vec<&str>> = workflows
        .lines()
        .map(|line| line.split_once('{').unwrap())
        .map(|(name, flows)| {
            (
                name,
                flows.trim_end_matches('}').split(',').collect::<Vec<_>>(),
            )
        })
        .collect();

    let parts: Vec<Part> = parts.lines().map(str::parse).map(Result::unwrap).collect();

    println!("Part 1: {}", part1(&parts, &workflows));
    println!("Part 2: {}", part2(&workflows));
}

fn part1(parts: &[Part], workflows: &HashMap<&str, Vec<&str>>) -> usize {
    parts
        .iter()
        .filter(|part| part.valide_part(workflows))
        .map(Part::rate)
        .sum()
}
fn count_combinations_ranges(parts: &HashMap<char, [u32; 2]>) -> u64 {
    // Count all the accepted parts in x, m, a and s ranges
    parts
        .values()
        .map(|res| u64::from(res[1]) - u64::from(res[0]) + 1)
        .product()
}

fn part2(workflows: &HashMap<&str, Vec<&str>>) -> u64 {
    // Start with all ranges
    let part_ranges: HashMap<char, [u32; 2]> = vec![
        ('x', [1, 4000]),
        ('m', [1, 4000]),
        ('a', [1, 4000]),
        ('s', [1, 4000]),
    ]
    .into_iter()
    .collect();

    let mut stack = Vec::new();
    stack.push((part_ranges, "in"));
    let mut valid_part_count = 0;
    while let Some((mut ranges, condition)) = stack.pop() {
        if condition == "A" {
            valid_part_count += count_combinations_ranges(&ranges);
            continue;
        } else if condition == "R" {
            continue;
        }

        let conditions = workflows.get(condition).unwrap();
        for &condition in conditions {
            if condition == "A" {
                valid_part_count += count_combinations_ranges(&ranges);
                continue;
            } else if condition == "R" {
                continue;
            }

            if let Some((cond, nxt)) = condition.split_once(':') {
                let a = cond.chars().next().unwrap();
                let op = cond.chars().nth(1).unwrap();
                let val = cond[2..].parse::<u32>().unwrap();
                let vals = ranges.get(&a).unwrap();
                let low = vals[0];
                let high = vals[1];
                let mut new_ranges = ranges.clone();
                if op == '>' {
                    new_ranges.insert(a, [val + 1, high]);
                    ranges.insert(a, [low, val]);
                } else {
                    new_ranges.insert(a, [low, val - 1]);
                    ranges.insert(a, [val, high]);
                }
                stack.push((new_ranges, nxt));
            } else {
                stack.push((ranges, condition));
                break;
            }
        }
    }
    valid_part_count
}
