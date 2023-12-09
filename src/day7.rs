use std::{collections::HashMap, str::FromStr};

macro_rules! run_part {
    ($idx:expr, $type:ty) => {
        let data_part: Vec<$type> = include_str!("../data/day_2023_7.data")
            .lines()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        println!("Part {}: {}", $idx, compute(&data_part));
    };
}

fn main() {
    run_part!(1, HandPart1);
    run_part!(2, HandPart2);
}

trait CamelHand {
    fn bid(&self) -> usize;
}

fn getcards_part1(cards: &str) -> Vec<u32> {
    cards
        .chars()
        .map(|chr| match chr {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'J' => 12,
            'T' => 11,
            x => x.to_digit(10).unwrap(),
        })
        .collect()
}

fn getcards_part2(cards: &str) -> Vec<u32> {
    cards
        .chars()
        .map(|chr| match chr {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'J' => 1,
            'T' => 11,
            x => x.to_digit(10).unwrap(),
        })
        .collect()
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

macro_rules! declare_hand_type {
    ($($handtype:ident,)+) => {
$(
   #[derive(Eq, PartialEq, Clone)]
    struct $handtype {
        cards: HashMap<char, usize>,
        hand: Vec<u32>,
        bid: usize,
        cardinality: usize,
    }

    impl Ord for $handtype {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.cardinality
                .cmp(&other.cardinality)
                .then_with(|| self.hand[0].cmp(&other.hand[0]))
                .then_with(|| self.hand[1].cmp(&other.hand[1]))
                .then_with(|| self.hand[2].cmp(&other.hand[2]))
                .then_with(|| self.hand[3].cmp(&other.hand[3]))
                .then_with(|| self.hand[4].cmp(&other.hand[4]))
        }
    }
    impl PartialOrd for $handtype {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl CamelHand for $handtype {
        fn bid(&self) -> usize {
            self.bid
        }}
)+};
}

declare_hand_type!(HandPart1, HandPart2,);

impl FromStr for HandPart1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').expect("Cards <-> bid failed");
        let bid = bid.parse().unwrap();
        let hand = getcards_part1(cards);

        let cards: HashMap<char, usize> = cards.chars().fold(HashMap::new(), |mut acc, chr| {
            *acc.entry(chr).or_default() += 1;
            acc
        });
        let cardinality = cards.values().map(|v| v.pow(2)).sum();
        Ok(Self {
            cards,
            hand,
            bid,
            cardinality,
        })
    }
}

impl FromStr for HandPart2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').expect("Cards <-> bid failed");
        let bid = bid.parse().unwrap();
        let hand = getcards_part2(cards);

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
            hand,
            bid,
            cardinality,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let raw: Vec<HandPart1> = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .lines()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        assert_eq!(compute(&raw), 6440)
    }

    #[test]
    fn test_day7_part2() {
        let raw: Vec<HandPart2> = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .lines()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        assert_eq!(compute(&raw), 5905)
    }
}
