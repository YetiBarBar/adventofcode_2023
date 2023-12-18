use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    direction_part1: Direction,
    count_part1: i64,
    direction_part2: Direction,
    count_part2: i64,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split_ascii_whitespace().collect();
        let direction_part1 = match splits.first() {
            Some(&"L") => Direction::Left,
            Some(&"R") => Direction::Right,
            Some(&"U") => Direction::Up,
            Some(&"D") => Direction::Down,
            _ => return Err(()),
        };
        let count_part1: i64 = splits[1].parse().unwrap();
        let count_part2: String = splits[2].chars().skip(2).take(5).collect();
        let count_part2 = i64::from_str_radix(&count_part2, 16).unwrap();
        let direction_part2 = match splits[2].chars().nth(7) {
            // 0 means R, 1 means D, 2 means L, and 3 means U
            Some('0') => Direction::Right,
            Some('1') => Direction::Down,
            Some('2') => Direction::Left,
            Some('3') => Direction::Up,
            _ => {
                return Err(());
            }
        };

        Ok(Self {
            direction_part1,
            count_part1,
            direction_part2,
            count_part2,
        })
    }
}

fn shoelace(vertices: &[(i64, i64)], boundary: i64) -> i64 {
    let area: i64 = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();
    area.abs() / 2 + boundary / 2 + 1
}

fn main() {
    let data: Vec<Instruction> = include_str!("../data/day_2023_18.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut x1: i64 = 0;
    let mut y1: i64 = 0;
    let mut x2: i64 = 0;
    let mut y2: i64 = 0;

    let mut points_part1 = vec![];
    let mut points_part2 = vec![];
    let mut boundary_part1 = 1;
    let mut boundary_part2 = 1;
    for instr in data {
        match instr.direction_part1 {
            Direction::Up => y1 += instr.count_part1,
            Direction::Down => y1 -= instr.count_part1,
            Direction::Left => x1 -= instr.count_part1,
            Direction::Right => x1 += instr.count_part1,
        }
        boundary_part1 += instr.count_part1;
        points_part1.push((x1, y1));
        match instr.direction_part2 {
            Direction::Up => y2 += instr.count_part2,
            Direction::Down => y2 -= instr.count_part2,
            Direction::Left => x2 -= instr.count_part2,
            Direction::Right => x2 += instr.count_part2,
        }
        boundary_part2 += instr.count_part2;
        points_part2.push((x2, y2));
    }
    println!("Part 1: {}", shoelace(&points_part1, boundary_part1));
    println!("Part 2: {}", shoelace(&points_part2, boundary_part2));
}
