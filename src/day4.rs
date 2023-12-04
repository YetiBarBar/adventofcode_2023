use std::{collections::HashMap, str::FromStr};

use adventofcode_tooling::AocError;

struct Card {
    card_id: usize,
    winning_cards: Vec<usize>,
    hand: Vec<usize>,
}

impl Card {
    fn evaluate_part1(&self) -> usize {
        let count = self.winning_count();
        if count == 0 {
            0
        } else {
            2_usize.pow(count - 1)
        }
    }

    fn winning_count(&self) -> u32 {
        self.hand
            .iter()
            .filter(|item| self.winning_cards.contains(&item))
            .count() as u32
    }
}

impl FromStr for Card {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_id, remain) = s.split_once(':').ok_or(AocError::ParsingError)?;
        let card_id = card_id
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| AocError::ParsingError)?;

        let (winning_cards, hand) = remain.split_once('|').ok_or(AocError::ParsingError)?;

        let winning_cards = get_cards(winning_cards).map_err(|_| AocError::ParsingError)?;
        let hand = get_cards(&hand).map_err(|_| AocError::ParsingError)?;

        Ok(Self {
            card_id,
            winning_cards,
            hand: hand,
        })
    }
}

fn get_cards(input: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    input
        .trim()
        .split(' ')
        .map(str::trim)
        .filter(|split| !split.is_empty())
        .map(str::parse)
        .collect()
}

fn part1(deck: &[Card]) -> usize {
    deck.iter().map(|card| card.evaluate_part1()).sum()
}

fn part2(deck: &[Card]) -> usize {
    let mut hmap: HashMap<usize, usize> = HashMap::new();
    for card in deck {
        let score = card.winning_count();
        *hmap.entry(card.card_id).or_default() += 1;
        let to_add = *hmap.get(&card.card_id).unwrap();
        for idx in 1..=score {
            *hmap.entry(card.card_id + idx as usize).or_default() += to_add;
        }
    }
    hmap.values().sum()
}

fn main() {
    let deck = include_str!("../data/day_2023_4.data")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Card>, _>>()
        .unwrap();

    println!("Part 1: {}", part1(&deck));
    println!("Part 2: {}", part2(&deck));
}
