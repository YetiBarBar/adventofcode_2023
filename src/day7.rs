use std::{collections::HashMap, str::FromStr};
fn main() {
    let raw: Vec<Hand> = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let data: Vec<Hand> = include_str!("../data/day_2023_7.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    println!("Test: {}", compute(&raw));
    println!("Part 1: {}", compute(&data));
    let raw: Vec<Hand2> = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let data: Vec<Hand2> = include_str!("../data/day_2023_7.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    println!("Test 2: {}", compute(&raw));
    println!("Part 2: {}", compute(&data));
}

trait CamelHand {
    fn bid(&self) -> usize;
}
impl CamelHand for Hand {
    fn bid(&self) -> usize {
        self.bid
    }
}
impl CamelHand for Hand2 {
    fn bid(&self) -> usize {
        self.bid
    }
}

fn compute<T: CamelHand + Ord + Clone>(hands: &[T]) -> usize {
    let mut hands = hands.to_vec();
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..)
        .map(|(hand, position)| hand.bid() * position)
        .sum()
}

#[derive(Eq, PartialEq, Clone)]
struct Hand {
    cards: HashMap<char, usize>,
    first_card: u32,
    second_card: u32,
    third_card: u32,
    fourth_cards: u32,
    fifth_cards: u32,
    bid: usize,
    cardinality: usize,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').expect("Cards <-> bid failed");
        let bid = bid.parse().unwrap();
        let first_card = get_char(cards, 0);
        let second_card = get_char(cards, 1);
        let third_card = get_char(cards, 2);
        let fourth_cards = get_char(cards, 3);
        let fifth_cards = get_char(cards, 4);

        let cards: HashMap<char, usize> = cards.chars().fold(HashMap::new(), |mut acc, chr| {
            *acc.entry(chr).or_default() += 1;
            acc
        });
        let cardinality = cards.values().map(|v| v.pow(2)).sum();
        Ok(Self {
            cards,
            first_card,
            second_card,
            third_card,
            fourth_cards,
            fifth_cards,
            bid,
            cardinality,
        })
    }
}

fn get_char(cards: &str, position: usize) -> u32 {
    let first_card = cards
        .chars()
        .nth(position)
        .map(|chr| match chr {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'J' => 12,
            'T' => 11,
            x => x.to_digit(10).unwrap(),
        })
        .unwrap();
    first_card
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cardinality
            .cmp(&other.cardinality)
            .then_with(|| self.first_card.cmp(&other.first_card))
            .then_with(|| self.second_card.cmp(&other.second_card))
            .then_with(|| self.third_card.cmp(&other.third_card))
            .then_with(|| self.fourth_cards.cmp(&other.fourth_cards))
            .then_with(|| self.fifth_cards.cmp(&other.fifth_cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Hand2 {
    cards: HashMap<char, usize>,
    first_card: u32,
    second_card: u32,
    third_card: u32,
    fourth_cards: u32,
    fifth_cards: u32,
    bid: usize,
    cardinality: usize,
}
impl FromStr for Hand2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').expect("Cards <-> bid failed");
        let bid = bid.parse().unwrap();
        let first_card = get_char2(cards, 0);
        let second_card = get_char2(cards, 1);
        let third_card = get_char2(cards, 2);
        let fourth_cards = get_char2(cards, 3);
        let fifth_cards = get_char2(cards, 4);

        let mut cards: HashMap<char, usize> = cards.chars().fold(HashMap::new(), |mut acc, chr| {
            *acc.entry(chr).or_default() += 1;
            acc
        });

        let j_count = *cards.get(&'J').unwrap_or(&0);
        let cardinality = if j_count == 5 {
            25
        } else {
            let (&max_chr, _) = cards
                .iter()
                .filter(|(chr, _)| **chr != 'J')
                .max_by_key(|(_, b)| **b)
                .unwrap();

            *cards.entry('J').or_default() = 0;
            *cards.get_mut(&max_chr).unwrap() += j_count;
            cards.values().map(|v| v.pow(2)).sum()
        };
        Ok(Self {
            cards,
            first_card,
            second_card,
            third_card,
            fourth_cards,
            fifth_cards,
            bid,
            cardinality,
        })
    }
}

fn get_char2(cards: &str, position: usize) -> u32 {
    let first_card = cards
        .chars()
        .nth(position)
        .map(|chr| match chr {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'J' => 1,
            'T' => 11,
            x => x.to_digit(10).unwrap(),
        })
        .unwrap();
    first_card
}
impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cardinality
            .cmp(&other.cardinality)
            .then_with(|| self.first_card.cmp(&other.first_card))
            .then_with(|| self.second_card.cmp(&other.second_card))
            .then_with(|| self.third_card.cmp(&other.third_card))
            .then_with(|| self.fourth_cards.cmp(&other.fourth_cards))
            .then_with(|| self.fifth_cards.cmp(&other.fifth_cards))
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
