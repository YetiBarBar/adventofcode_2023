use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PipeDirection {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthEastBend,
    SouthWestBend,
    Start,
    Empty,
}

impl std::fmt::Display for PipeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PipeDirection::VerticalPipe => '|',
                PipeDirection::HorizontalPipe => '-',
                PipeDirection::NorthEastBend => '└',
                PipeDirection::NorthWestBend => '┘',
                PipeDirection::SouthEastBend => '┌',
                PipeDirection::SouthWestBend => '┐',
                PipeDirection::Start => 'S',
                PipeDirection::Empty => ' ',
            }
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl PipeDirection {
    fn east(self) -> bool {
        matches!(
            self,
            crate::PipeDirection::HorizontalPipe
                | crate::PipeDirection::NorthEastBend
                | crate::PipeDirection::SouthEastBend
                | crate::PipeDirection::Start
        )
    }

    fn west(self) -> bool {
        matches!(
            self,
            crate::PipeDirection::HorizontalPipe
                | crate::PipeDirection::NorthWestBend
                | crate::PipeDirection::SouthWestBend
                | crate::PipeDirection::Start
        )
    }

    fn south(self) -> bool {
        matches!(
            self,
            crate::PipeDirection::VerticalPipe
                | crate::PipeDirection::SouthEastBend
                | crate::PipeDirection::SouthWestBend
                | crate::PipeDirection::Start
        )
    }

    fn north(self) -> bool {
        matches!(
            self,
            crate::PipeDirection::VerticalPipe
                | crate::PipeDirection::NorthEastBend
                | crate::PipeDirection::NorthWestBend
                | crate::PipeDirection::Start
        )
    }

    fn can_connect(self, next: PipeDirection, direction: Direction) -> bool {
        match direction {
            Direction::North => self.north() && next.south(),
            Direction::South => self.south() && next.north(),
            Direction::East => self.east() && next.west(),
            Direction::West => self.west() && next.east(),
        }
    }
}

fn main() {
    let mut v = vec![];
    let data: HashMap<(usize, usize), PipeDirection> = include_str!("../data/day_2023_10.data")
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, chr)| {
                let pipe = match chr {
                    'S' => PipeDirection::Start,
                    '.' => PipeDirection::Empty,
                    '-' => PipeDirection::HorizontalPipe,
                    '|' => PipeDirection::VerticalPipe,
                    'L' => PipeDirection::NorthEastBend,
                    'F' => PipeDirection::SouthEastBend,
                    'J' => PipeDirection::NorthWestBend,
                    '7' => PipeDirection::SouthWestBend,
                    _ => panic!(),
                };
                ((x, y), pipe)
            })
        })
        .collect();

    let _data: HashMap<(usize, usize), PipeDirection> = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, chr)| {
                let pipe = match chr {
                    'S' => PipeDirection::Start,
                    '.' => PipeDirection::Empty,
                    '-' => PipeDirection::HorizontalPipe,
                    '|' => PipeDirection::VerticalPipe,
                    'L' => PipeDirection::NorthEastBend,
                    'F' => PipeDirection::SouthEastBend,
                    'J' => PipeDirection::NorthWestBend,
                    '7' => PipeDirection::SouthWestBend,
                    _ => panic!(),
                };
                ((x, y), pipe)
            })
        })
        .collect();

    let (&x_start, &y_start) = data
        .iter()
        .find(|&(_, pipe)| pipe == &PipeDirection::Start)
        .map(|((x, y), _)| (x, y))
        .unwrap();

    let mut visited = HashSet::new();

    let _ = dig(
        x_start,
        y_start,
        &data,
        &mut visited,
        Direction::East,
        &mut v,
    );

    println!("Part 1: {}", v.len() / 2);

    for y in 0..140 {
        for x in 0..140 {
            if v.contains(&(x, y)) {
                print!("{}", data.get(&(x, y)).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn dig(
    pos_x: usize,
    pos_y: usize,
    map: &HashMap<(usize, usize), PipeDirection>,
    visited: &mut HashSet<(usize, usize)>,
    last_direction: Direction,
    v: &mut Vec<(usize, usize)>,
) -> Option<usize> {
    if map.get(&(pos_x, pos_y)) == Some(&PipeDirection::Start) && visited.contains(&(pos_x, pos_y))
    {
        return Some(1);
    }

    visited.insert((pos_x, pos_y));
    let north = map.get(&(pos_x, pos_y.wrapping_sub(1))).copied();
    let east = map.get(&(pos_x + 1, pos_y)).copied();
    let west = map.get(&(pos_x.wrapping_sub(1), pos_y)).copied();
    let south = map.get(&(pos_x, pos_y + 1)).copied();
    let current = map.get(&(pos_x, pos_y)).copied().unwrap();

    if let Some(north) = north {
        if current.can_connect(north, Direction::North) && last_direction != Direction::South {
            if let Some(value) = dig(pos_x, pos_y - 1, map, visited, Direction::North, v) {
                v.push((pos_x, pos_y));
                return Some(1 + value);
            }
        }
    }

    if let Some(south) = south {
        if current.can_connect(south, Direction::South) && last_direction != Direction::North {
            if let Some(value) = dig(pos_x, pos_y + 1, map, visited, Direction::South, v) {
                v.push((pos_x, pos_y));
                return Some(1 + value);
            }
        }
    }

    if let Some(east) = east {
        if current.can_connect(east, Direction::East) && last_direction != Direction::West {
            if let Some(value) = dig(pos_x + 1, pos_y, map, visited, Direction::East, v) {
                v.push((pos_x, pos_y));
                return Some(1 + value);
            }
        }
    }
    if let Some(west) = west {
        if current.can_connect(west, Direction::West) && last_direction != Direction::East {
            if let Some(value) = dig(pos_x - 1, pos_y, map, visited, Direction::West, v) {
                v.push((pos_x, pos_y));
                return Some(1 + value);
            }
        }
    }

    None
}
