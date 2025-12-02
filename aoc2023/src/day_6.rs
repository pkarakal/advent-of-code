use itertools::Itertools;
use common::{Answer, Solution};

const SPEED: u64 = 1;

#[derive(Default, Debug)]
struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn get_winning_ways(&self) -> usize {
        (0..self.time)
            .filter(|x| (self.time - x) * SPEED*x > self.distance)
            .count()
    }
}

pub struct Day6;

impl Solution for Day6{
    fn name(&self) -> String {
        "Day 6".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        parse_one(input)
            .iter()
            .map(|x| x.get_winning_ways())
            .product::<usize>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        parse_two(input)
            .get_winning_ways()
            .into()
    }
}

fn parse_one(input: &str) -> Vec<Race> {
    let (times, distances) = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .skip(1)
                .map(|item| item.parse::<u64>().unwrap())
        })
        .next_tuple()
        .unwrap();

    times.zip(distances)
        .map(|(time,distance)| {
          Race{time, distance}
        })
        .collect::<Vec<Race>>()
}


fn parse_two(input: &str) -> Race {
    let (time, distance) = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .next_tuple()
        .unwrap();
    Race {time, distance}
}


#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_6::Day6;

    const CASE_A: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day6.part_one(CASE_A), 288usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day6.part_two(CASE_A), 71503usize.into())
    }
}
