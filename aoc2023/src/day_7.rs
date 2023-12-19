use std::cmp::Ordering;
use itertools::Itertools;
use common::{Answer, Solution};
use crate::day_7::CardType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

const CARD_SEQUENCE_A: [&str; 13] = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];
const CARD_SEQUENCE_B: [&str; 13] = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"];

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum CardType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}


#[derive(Default, Debug)]
struct CamelCard {
    cards: Vec<u8>,
    bid: u32,
}

impl CamelCard {
    fn compare(&self, other: &CamelCard) -> Ordering {
        for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                return a.cmp(&b);
            }
        }
        return Ordering::Equal;
    }

    fn classify(&self) -> CardType {
        let mut card_counts = [0; 13];
        for &card in &self.cards {
            card_counts[13 - card as usize] += 1;
        }

        if card_counts.iter().any(|&c| c == 5) {
            CardType::FiveOfAKind
        } else if card_counts.iter().any(|&c| c == 4) {
            FourOfAKind
        } else if card_counts.iter().any(|&c| c == 3) && card_counts.iter().any(|&c| c == 2) {
            FullHouse
        } else if card_counts.iter().any(|&c| c == 3) {
            ThreeOfAKind
        } else if card_counts.iter().filter(|&&c| c == 2).count() == 2 {
            TwoPair
        } else if card_counts.iter().any(|&c| c == 2) {
            OnePair
        } else {
            HighCard
        }
    }

    fn optimize(&self) -> CardType {
        let mut card_counts = [0; 13];
        for &card in &self.cards {
            card_counts[13 - card as usize] += 1;
        }
        let jacks = card_counts[12];

        let counts = card_counts[0..12]
            .iter()
            .copied()
            .filter(|x| *x != 0)
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        return if counts.len() <= 1 || counts[0] + jacks == 5 {
            FiveOfAKind
        } else if counts[0] + jacks == 4 {
            FourOfAKind
        } else if (counts[0] == 3 && counts[1] + jacks == 2) || (counts[0] + jacks == 3 && counts[1] == 2) {
            FullHouse
        } else if counts[0] + jacks == 3 {
            ThreeOfAKind
        } else if (counts[0] + jacks == 2 && counts[1] == 2) || (counts[0] + jacks == 2 && counts[1] == 2) {
            TwoPair
        } else if counts[0] + jacks == 2 {
            OnePair
        } else {
            HighCard
        };
    }
}

pub struct Day7;

impl Solution for Day7 {
    fn name(&self) -> String {
        return "Day 7".into();
    }

    fn part_one(&self, input: &str) -> Answer {
        let mut items = parse(input, CARD_SEQUENCE_A.iter().join(""));
        items
            .sort_by(|a, b| {
                a.classify()
                    .cmp(&b.classify())
                    .then_with(|| b.compare(a))
            });
        items
            .iter()
            .rev()
            .enumerate()
            .map(|(i, card)| card.bid as usize * (i + 1))
            .sum::<usize>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let mut items = parse(input, CARD_SEQUENCE_B.iter().join(""));
        items
            .sort_by(|a, b| {
                a.optimize()
                    .cmp(&b.optimize())
                    .then_with(|| b.compare(a))
            });
        items
            .iter()
            .rev()
            .enumerate()
            .map(|(i, card)| card.bid as usize * (i + 1))
            .sum::<usize>()
            .into()
    }
}

fn parse(input: &str, card_sequence: String) -> Vec<CamelCard> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let mut input_iter = line.split_whitespace().into_iter();

        let cards = input_iter.next().unwrap();
        let bid = input_iter.last().unwrap();

        let cards = cards.as_bytes()
            .iter()
            .map(|&c| 13 - card_sequence.find(c as char).unwrap() as u8)
            .collect();
        let bid = bid.parse().unwrap();

        hands.push(CamelCard { cards, bid });
    }

    return hands;
}

#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_7::Day7;

    const CASE_A: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day7.part_one(CASE_A), 6440usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day7.part_two(CASE_A), 5905usize.into())
    }
}