use std::collections::HashMap;

use adventofcode_tooling::Matrix2D;

#[derive(Debug)]
struct EnginePart {
    value: usize,
    symbol: char,
    symbol_position: (usize, usize),
}

fn matrix_to_engine_parts(data: &Matrix2D<char>) -> Vec<EnginePart> {
    let mut current_value = vec![];
    let mut current_detected_char: Option<char> = None;
    let mut engine_collection = vec![];
    let mut symbol_position = (0, 0);

    for (y, line) in data.rows().into_iter().enumerate() {
        for (x, chr) in line.into_iter().enumerate() {
            if chr.is_ascii_digit() {
                current_value.push(chr);
                if current_detected_char.is_none() {
                    let delta_index = [
                        (
                            x.checked_sub(1).unwrap_or(data.width + 1),
                            y.checked_sub(1).unwrap_or(data.height + 1),
                        ),
                        (x.checked_sub(1).unwrap_or(data.width + 1), y),
                        (x.checked_sub(1).unwrap_or(data.width + 1), y + 1),
                        (x, y.checked_sub(1).unwrap_or(data.height + 1)),
                        (x, y + 1),
                        (x + 1, y.checked_sub(1).unwrap_or(data.height + 1)),
                        (x + 1, y),
                        (x + 1, y + 1),
                    ];
                    for position in &delta_index {
                        let new_char = data.get(position.0, position.1).unwrap_or('.');
                        if !new_char.is_ascii_digit() && new_char != '.' {
                            current_detected_char = Some(new_char);
                            symbol_position = *position;
                        }
                    }
                }
            } else {
                if !current_value.is_empty() {
                    if let Some(current_detected_char) = current_detected_char {
                        engine_collection.push(EnginePart {
                            value: current_value
                                .iter()
                                .copied()
                                .collect::<String>()
                                .parse()
                                .unwrap(),
                            symbol: current_detected_char,
                            symbol_position,
                        });
                    }
                }
                current_detected_char = None;
                current_value.clear();
            }
        }

        if !current_value.is_empty() {
            if let Some(current_detected_char) = current_detected_char {
                engine_collection.push(EnginePart {
                    value: current_value
                        .iter()
                        .copied()
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                    symbol: current_detected_char,
                    symbol_position,
                });
            }
        }
        current_detected_char = None;
        current_value.clear();
    }
    engine_collection
}

fn main() {
    let input_data: Vec<&str> = include_str!("../data/day_2023_3.data").lines().collect();
    let line_len = input_data[0].len();
    let col_count = input_data.len();
    let chars = input_data.iter().flat_map(|line| line.chars()).collect();

    let data: Matrix2D<char> = Matrix2D {
        width: line_len,
        height: col_count,
        values: chars,
    };

    let engines = matrix_to_engine_parts(&data);
    println!("Part 1: {}", part1(&engines));
    println!("Part 2: {}", part2(&engines));
}

fn part1(engines: &[EnginePart]) -> usize {
    engines.iter().map(|engine| engine.value).sum()
}

fn part2(engines: &[EnginePart]) -> usize {
    let mut hmap = HashMap::new();
    for engine in engines.iter().filter(|engine| engine.symbol == '*') {
        hmap.entry(engine.symbol_position)
            .or_insert(vec![])
            .push(engine);
    }
    hmap.values()
        .map(|engines| {
            if engines.len() == 2 {
                engines.iter().map(|engine| engine.value).product::<usize>()
            } else {
                0
            }
        })
        .sum()
}
