use std::collections::HashMap;

fn main() {
    let data: Vec<String> = include_str!("../data/day_2023_14.data")
        .lines()
        .map(str::to_string)
        .collect();
    println!("Part 1: {}", part1(&data));

    println!("Part 2 loop: {:?}", loop_detection(&data));
    println!("Part 2: {}", part2(&data));
}

fn part2(data: &[String]) -> usize {
    // We know we gonna have a loop
    let Some((start_index, loop_size)) = loop_detection(data) else {
        panic!("Can't find a loop!")
    };

    let mut data = data.to_vec();
    for _ in 0..(start_index + (1_000_000_000 - start_index) % loop_size) {
        data = cycle(&data);
    }
    data.iter().rev().zip(1..).fold(0, |mut acc, (line, pos)| {
        acc += line.chars().filter(|chr| chr == &'O').count() * pos;
        acc
    })
}

fn loop_detection(data: &[String]) -> Option<(usize, usize)> {
    let mut hmap = HashMap::new();
    let mut data = data.to_vec();
    for idx in 0.. {
        data = cycle(&data);
        if let Some(first_index) = hmap.insert(data.clone(), idx) {
            return Some((first_index, idx - first_index));
        }
    }
    todo!()
}

fn part1(data: &[String]) -> usize {
    let res = to_north(data);
    res.iter().rev().zip(1..).fold(0, |mut acc, (line, pos)| {
        acc += line.chars().filter(|chr| chr == &'O').count() * pos;
        acc
    })
}

fn cycle(data: &[String]) -> Vec<String> {
    to_east(&to_south(&to_west(&to_north(data))))
}

fn to_north(data: &[String]) -> Vec<String> {
    // First rotate all the strings!
    let mut transposed = vec![String::new(); data[0].len()];
    for line in data {
        for (pos, chr) in line.chars().enumerate() {
            transposed[pos].push(chr);
        }
    }

    let compressed: Vec<String> = transposed
        .iter()
        .map(|line| compress_line_north_and_west(line))
        .collect();
    let mut transposed2 = vec![String::new(); compressed[0].len()];
    for line in &compressed {
        for (pos, chr) in line.chars().enumerate() {
            transposed2[pos].push(chr);
        }
    }
    transposed2
}

fn to_south(data: &[String]) -> Vec<String> {
    let mut transposed = vec![String::new(); data[0].len()];
    for line in data {
        for (pos, chr) in line.chars().enumerate() {
            transposed[pos].push(chr);
        }
    }
    let compressed: Vec<String> = transposed
        .iter()
        .map(|line| compress_line_east_south(line))
        .collect();
    let mut transposed2 = vec![String::new(); compressed[0].len()];
    for line in &compressed {
        for (pos, chr) in line.chars().enumerate() {
            transposed2[pos].push(chr);
        }
    }
    transposed2
}

fn to_east(data: &[String]) -> Vec<String> {
    data.iter()
        .map(|line| compress_line_east_south(line))
        .collect()
}

fn to_west(data: &[String]) -> Vec<String> {
    data.iter()
        .map(|line| compress_line_north_and_west(line))
        .collect()
}

fn compress_line_east_south(input: &str) -> String {
    input
        .split('#')
        .enumerate()
        .map(|(pos, chunk)| {
            let zeros = chunk.chars().filter(|chr| chr == &'O').count();
            let mut start = if pos == 0 {
                String::new()
            } else {
                String::from("#")
            };
            for _ in 0..(chunk.len() - zeros) {
                start.push('.');
            }
            for _ in 0..zeros {
                start.push('O');
            }
            start
        })
        .collect()
}

fn compress_line_north_and_west(input: &str) -> String {
    input
        .split('#')
        .enumerate()
        .map(|(pos, chunk)| {
            let zeros = chunk.chars().filter(|chr| chr == &'O').count();
            let mut start = if pos == 0 {
                String::new()
            } else {
                String::from("#")
            };
            for _ in 0..zeros {
                start.push('O');
            }
            for _ in 0..(chunk.len() - zeros) {
                start.push('.');
            }
            start
        })
        .collect()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn day_14_part1() {
        let data: Vec<String> = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            .lines()
            .map(|line| line.to_string())
            .collect();
        assert_eq!(part1(&data), 136);
    }
}
