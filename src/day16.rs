use std::collections::HashSet;

use adventofcode_tooling::Matrix2D;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum CellType {
    EmptySpace,
    VerticalSplitter,
    HorizontalSplitter,
    SlashMirror,
    AntiSlashMirror,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::EmptySpace,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '\\' => Self::AntiSlashMirror,
            '/' => Self::SlashMirror,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Beam {
    x: usize,
    y: usize,
    heading: Heading,
}

impl Beam {
    fn nexts(&self, map: &Matrix2D<CellType>) -> Vec<Beam> {
        let mut res = vec![];
        let (next_x, next_y) = match self.heading {
            Heading::Up => (self.x, self.y.wrapping_sub(1)),
            Heading::Down => (self.x, self.y + 1),
            Heading::Left => (self.x.wrapping_sub(1), self.y),
            Heading::Right => (self.x + 1, self.y),
        };
        let next_cell = map.get(next_x, next_y);
        if let Some(cell) = next_cell {
            match cell {
                CellType::EmptySpace => res.push(Beam {
                    x: next_x,
                    y: next_y,
                    heading: self.heading,
                }),
                CellType::VerticalSplitter => match self.heading {
                    Heading::Left | Heading::Right => {
                        res.push(Beam {
                            x: next_x,
                            y: next_y,
                            heading: Heading::Up,
                        });
                        res.push(Beam {
                            x: next_x,
                            y: next_y,
                            heading: Heading::Down,
                        });
                    }
                    _ => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: self.heading,
                    }),
                },
                CellType::HorizontalSplitter => match self.heading {
                    Heading::Up | Heading::Down => {
                        res.push(Beam {
                            x: next_x,
                            y: next_y,
                            heading: Heading::Left,
                        });
                        res.push(Beam {
                            x: next_x,
                            y: next_y,
                            heading: Heading::Right,
                        });
                    }
                    _ => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: self.heading,
                    }),
                },
                CellType::SlashMirror => match self.heading {
                    Heading::Up => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Right,
                    }),
                    Heading::Down => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Left,
                    }),
                    Heading::Left => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Down,
                    }),
                    Heading::Right => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Up,
                    }),
                },
                CellType::AntiSlashMirror => match self.heading {
                    Heading::Up => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Left,
                    }),
                    Heading::Down => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Right,
                    }),
                    Heading::Left => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Up,
                    }),
                    Heading::Right => res.push(Beam {
                        x: next_x,
                        y: next_y,
                        heading: Heading::Down,
                    }),
                },
            }
        }
        res
    }
}

fn compute_entry(x: usize, y: usize, heading: Heading, map: &Matrix2D<CellType>) -> Vec<Beam> {
    let mut res = vec![];

    match (map.get(x, y).unwrap(), heading) {
        (CellType::VerticalSplitter, Heading::Left | Heading::Right) => {
            res.push(Beam {
                x,
                y,
                heading: Heading::Up,
            });
            res.push(Beam {
                x,
                y,
                heading: Heading::Down,
            });
        }
        (CellType::HorizontalSplitter, Heading::Up | Heading::Down) => {
            res.push(Beam {
                x,
                y,
                heading: Heading::Left,
            });
            res.push(Beam {
                x,
                y,
                heading: Heading::Right,
            });
        }
        (CellType::SlashMirror, Heading::Up) | (CellType::AntiSlashMirror, Heading::Down) => res
            .push(Beam {
                x,
                y,
                heading: Heading::Right,
            }),
        (CellType::SlashMirror, Heading::Down) | (CellType::AntiSlashMirror, Heading::Up) => res
            .push(Beam {
                x,
                y,
                heading: Heading::Left,
            }),
        (CellType::SlashMirror, Heading::Left) | (CellType::AntiSlashMirror, Heading::Right) => res
            .push(Beam {
                x,
                y,
                heading: Heading::Down,
            }),
        (CellType::SlashMirror, Heading::Right) | (CellType::AntiSlashMirror, Heading::Left) => res
            .push(Beam {
                x,
                y,
                heading: Heading::Up,
            }),
        (
            CellType::EmptySpace | CellType::VerticalSplitter | CellType::HorizontalSplitter,
            heading,
        ) => {
            res.push(Beam { x, y, heading });
        }
    }
    res
}

fn part(map: &Matrix2D<CellType>, start_x: usize, start_y: usize, start_heading: Heading) -> usize {
    let mut visited = HashSet::new();

    let mut adjacents = compute_entry(start_x, start_y, start_heading, map);

    loop {
        let mut new_adjacents = vec![];
        for beam in adjacents {
            visited.insert(beam.clone());
            new_adjacents.extend(beam.nexts(map).into_iter());
        }
        if new_adjacents.is_empty() {
            break;
        }

        adjacents = new_adjacents
            .into_iter()
            .filter(|beam| !visited.contains(beam))
            .collect();
    }
    let hset = visited
        .into_iter()
        .map(|beam| (beam.x, beam.y))
        .collect::<HashSet<_>>();
    hset.len()
}

fn part2(map: &Matrix2D<CellType>) -> usize {
    let max1 = (0..map.height)
        .map(|idx| part(map, idx, 0, Heading::Down))
        .max()
        .unwrap_or(0);
    let max2 = (0..map.height)
        .map(|idx| part(map, idx, map.width - 1, Heading::Up))
        .max()
        .unwrap_or(0);
    let max3 = (0..map.width)
        .map(|idx| part(map, 0, idx, Heading::Right))
        .max()
        .unwrap_or(0);
    let max4 = (0..map.height)
        .map(|idx| part(map, map.width - 1, idx, Heading::Left))
        .max()
        .unwrap_or(0);

    max1.max(max2).max(max3).max(max4)
}

fn main() {
    let data: Vec<&str> = include_str!("../data/day_2023_16.data").lines().collect();
    let len_x = data[0].chars().count();
    let len_y = data.len();

    let data: Vec<CellType> = data
        .iter()
        .flat_map(|line| line.chars())
        .map(std::convert::Into::into)
        .collect();

    let mut data_matrix = Matrix2D::new(len_x, len_y);
    data_matrix.values = data;

    println!("Part 1: {}", part(&data_matrix, 0, 0, Heading::Right));
    println!("Part 2: {}", part2(&data_matrix));
}
