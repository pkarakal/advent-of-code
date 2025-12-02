use common::{Answer, Solution};
use std::collections::HashMap;

pub struct Day4;

impl Solution for Day4 {
    fn name(&self) -> String {
        "Day 4".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        parse(input)
            .iter()
            .map(|x| x.get_winning_numbers())
            .filter(|x| *x > 0)
            .map(|x| 2u32.pow((x - 1) as u32))
            .sum::<u32>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let cards = parse(input);

        let mut copies: HashMap<usize, usize> = HashMap::new();

        for card in cards.clone() {
            copies.insert(card.number, 1);
        }

        for (i, card) in cards.iter().enumerate() {
            let nums = card
                .scratch
                .iter()
                .filter(|x| card.winning.contains(x))
                .count();
            for j in 1..=nums {
                *copies.entry(i + j + 1).or_default() += copies[&(i + 1)];
            }
        }
        copies.values().sum::<usize>().into()
    }
}

#[derive(Default, Debug, Clone)]
struct Card {
    number: usize,
    winning: Vec<u32>,
    scratch: Vec<u32>,
}

impl Card {
    fn get_winning_numbers(&self) -> usize {
        self.scratch
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = vec![];
    for (index, input_line) in input.lines().enumerate() {
        let (_, line) = input_line.split_once(":").unwrap();
        let (winning_numbers, scratch_numbers) = line.trim().split_once("|").unwrap();
        let winning: Vec<u32> = winning_numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let scratch: Vec<u32> = scratch_numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        cards.push(Card {
            number: index + 1,
            winning,
            scratch,
        })
    }
    cards
}

#[cfg(test)]
mod test {
    use crate::day_4::Day4;
    use common::{Answer, Solution};

    const CASE_A: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        assert_eq!(Day4.part_one(CASE_A), Answer::U32(13))
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day4.part_two(CASE_A), Answer::USize(30))
    }
}
