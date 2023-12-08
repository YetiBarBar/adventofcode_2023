use std::{collections::HashMap, str::FromStr};

use num::Integer;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl<'a> FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, remain) = s.split_once(" = (").unwrap();
        let (left, right) = remain.split_once(", ").unwrap();
        let name = name.trim().to_string();
        let left = left.trim().to_string();
        let right = right.trim_end_matches(')').to_string();
        Ok(Self { name, left, right })
    }
}

fn main() {
    let cycle: Vec<char> = include_str!("../data/day_2023_8.data")
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect();

    let nodes: HashMap<String, Node> = include_str!("../data/day_2023_8.data")
        .lines()
        .skip(2)
        .map(str::parse::<Node>)
        .map(Result::unwrap)
        .map(|node| (node.name.clone(), node))
        .collect();

    println!("Part 1: {}", part1(&cycle, &nodes));
    println!("Part 2: {}", part2(&cycle, &nodes));
}

fn part1(cycle: &[char], nodes: &HashMap<String, Node>) -> usize {
    let mut next_move = cycle.iter().cycle();
    let mut current_node = "AAA".to_string();
    let mut steps = 0;

    while current_node != "ZZZ".to_string() {
        let mve = next_move.next();
        if mve == Some(&'L') {
            current_node = nodes.get(&current_node).unwrap().left.clone();
        } else {
            current_node = nodes.get(&current_node).unwrap().right.clone();
        }
        steps += 1;
    }
    steps
}

fn part2(cycle: &[char], nodes: &HashMap<String, Node>) -> usize {
    let starting_nodes: Vec<String> = nodes.keys().filter(|s| s.ends_with('A')).cloned().collect();
    let sizes: Vec<usize> = starting_nodes
        .iter()
        .map(|s| part2_cycle(s, cycle, nodes))
        .collect();
    sizes.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}

fn part2_cycle(start_node: &str, cycle: &[char], nodes: &HashMap<String, Node>) -> usize {
    let mut next_move = cycle.iter().cycle();
    let mut current_node = start_node.to_string();
    let mut steps = 0;

    while !current_node.ends_with('Z') {
        let mve = next_move.next();
        if mve == Some(&'L') {
            current_node = nodes.get(&current_node).unwrap().left.clone();
        } else {
            current_node = nodes.get(&current_node).unwrap().right.clone();
        }
        steps += 1;
    }
    steps
}
