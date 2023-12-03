use adventofcode_tooling::AocError;
use std::{cmp::max, str::FromStr};

#[derive(Debug)]
struct Turn {
    colors: Vec<Color>,
}

#[derive(Debug)]
enum Color {
    Red(usize),
    Blue(usize),
    Green(usize),
}

impl FromStr for Color {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_once(' ').ok_or(AocError::ParsingError)?;
        let value = parts.0.trim().parse().map_err(|_| AocError::ParsingError)?;
        match parts.1 {
            "red" => Ok(Self::Red(value)),
            "green" => Ok(Self::Green(value)),
            "blue" => Ok(Self::Blue(value)),
            _ => Err(AocError::ParsingError),
        }
    }
}

impl FromStr for Turn {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<Color>, _>>()?;
        Ok(Self { colors })
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    turns: Vec<Turn>,
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, turns) = s.split_once(": ").ok_or(AocError::ParsingError)?;
        let id = id
            .trim_start_matches("Game ")
            .parse()
            .map_err(|_| AocError::ParsingError)?;
        let turns: Vec<Turn> = turns
            .split(';')
            .map(str::parse)
            .collect::<Result<Vec<Turn>, _>>()?;
        Ok(Self { id, turns })
    }
}

impl Turn {
    fn is_turn_valid_part1(&self, red: usize, green: usize, blue: usize) -> bool {
        self.colors.iter().all(|item| match item {
            Color::Red(value) => red >= *value,
            Color::Green(value) => green >= *value,
            Color::Blue(value) => blue >= *value,
        })
    }
}

impl Game {
    fn power(&self) -> usize {
        let mut red_min = 0;
        let mut blue_min = 0;
        let mut green_min = 0;

        for turn in &self.turns {
            for color in &turn.colors {
                match color {
                    Color::Red(red) => red_min = max(*red, red_min),
                    Color::Blue(blue) => blue_min = max(*blue, blue_min),
                    Color::Green(green) => green_min = max(green_min, *green),
                }
            }
        }
        red_min * blue_min * green_min
    }
}

fn part1(data: &[Game]) -> usize {
    data.iter()
        .filter_map(|game| {
            game.turns
                .iter()
                .all(|turn| turn.is_turn_valid_part1(12, 13, 14))
                .then_some(game.id)
        })
        .sum()
}

fn main() {
    let data: Vec<Game> = include_str!("../data/day_2023_2.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part2(data: &[Game]) -> usize {
    data.iter().map(Game::power).sum()
}
