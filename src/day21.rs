use std::collections::{HashMap, HashSet};

fn main() {
    let data: HashMap<(i64, i64), char> = include_str!("../data/day_2023_21.data")
        .lines()
        .zip(0..)
        .flat_map(|(line, y)| line.chars().zip(0..).map(move |(chr, x)| ((x, y), chr)))
        .collect();

    let start_point = *data.iter().find(|(_, chr)| chr == &&'S').unwrap().0;

    println!("Part 1: {}", part1(&data, start_point, 64));
    println!("Part 2: {}", part2(&data, start_point, 26_501_365 / 131));
}

fn part1(data: &HashMap<(i64, i64), char>, start: (i64, i64), turns: usize) -> i64 {
    let mut adjacents = std::collections::HashSet::new();
    adjacents.insert(start);

    for _ in 0..turns {
        adjacents = compute_adjacents(data, &adjacents);
    }

    adjacents
        .into_iter()
        .map(|pos| data.get(&pos))
        .filter(|item| item != &Some(&'#'))
        .count() as i64
}

fn part2(data: &HashMap<(i64, i64), char>, start: (i64, i64), n: i64) -> i64 {
    println!("Part 2 only works due to specificity of data (no '#' on frontier or in the middle lines of the map)");
    // u_n = u0 + n * d1 + n * (n-1) * d' / 2.
    println!("Formula seems to be wrong here...");
    let u0 = part1(data, start, 65);
    let u1 = part1(data, start, 65 + 131);
    let u2 = part1(data, start, 65 + 2 * 131);
    let n = 202300;
    // u0 + n * (u1 - u0) + n * (n - 1) * (u2 + u0) / 2
    let det_a: f64 = -2.0;
    let det_a0: f64 = -u0 as f64 + 2.0 * u1 as f64 - u2 as f64;
    let det_a1: f64 = 3.0 * u0 as f64 - 4.0 * u1 as f64 + u2 as f64;
    let det_a2: f64 = -2.0 * u0 as f64;
    let x0: i64 = (det_a0 / det_a) as i64;
    let x1: i64 = (det_a1 / det_a) as i64;
    let x2: i64 = (det_a2 / det_a) as i64;
    x0 * n * n + x1 * n + x2
}

fn compute_adjacents(
    data: &HashMap<(i64, i64), char>,
    current: &std::collections::HashSet<(i64, i64)>,
) -> std::collections::HashSet<(i64, i64)> {
    let mut new_adjacents = HashSet::new();

    for point in current {
        if data.get(&(point.0.rem_euclid(131), point.1.rem_euclid(131))) != Some(&'#') {
            new_adjacents.insert((point.0 - 1, point.1));
            new_adjacents.insert((point.0 + 1, point.1));
            new_adjacents.insert((point.0, point.1 - 1));
            new_adjacents.insert((point.0, point.1 + 1));
        }
    }
    new_adjacents
}
