use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let data: Vec<Vec<char>> = include_str!("../data/day_2023_11.data")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let galaxies: Vec<(usize, usize)> = data
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, chr)| (chr == &'#').then_some((x, y)))
        })
        .collect();

    let empty_rows: HashSet<usize> = data
        .iter()
        .enumerate()
        .filter(|(_, l)| l.iter().all(|chr| chr == &'.'))
        .map(|(y, _)| y)
        .collect();

    let empty_cols: HashSet<usize> = (0..(data[0].len()))
        .filter(|&x| data.iter().all(|line| line[x] == '.'))
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for item in galaxies.iter().combinations(2) {
        let (x1, y1) = item[0];
        let (x2, y2) = item[1];

        let dx = x1.max(x2) - x1.min(x2);
        let dy = y1.max(y2) - y1.min(y2);
        part1 += dx + dy;
        part2 += dx + dy;

        let (x_min, x_max) = (std::cmp::min(x1, x2), std::cmp::max(x1, x2));
        let (y_min, y_max) = (std::cmp::min(y1, y2), std::cmp::max(y1, y2));

        for idx in *x_min..*x_max {
            if empty_cols.contains(&idx) {
                part1 += 1;
                part2 += 1_000_000 - 1;
            }
        }
        for idx in *y_min..*y_max {
            if empty_rows.contains(&idx) {
                part1 += 1;
                part2 += 1_000_000 - 1;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
