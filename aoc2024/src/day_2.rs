use common::{Answer, Solution};
use itertools::Itertools;

pub struct Day2;

impl Solution for Day2 {
    fn name(&self) -> String {
        "Day 2".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let items = parse(input);
        let res = items
            .iter()
            .filter(|x| {
                let sig = (x[0] - x[1]).signum();
                x.iter()
                    .tuple_windows()
                    .map(|(a, b)| a - b)
                    .all(|y| { 1..=3 }.contains(&y.abs()) && y.signum() == sig)
            })
            .count() as i64;
        res.into()
    }
    fn part_two(&self, input: &str) -> Answer {
        let items = parse(input);
        items
            .iter()
            .filter(|x| tolerates_one_failure(x, None))
            .count()
            .into()
    }
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn tolerates_one_failure(input: &Vec<i64>, skip: Option<usize>) -> bool {
    let values_iter = input
        .iter()
        .enumerate()
        .filter(|(idx, _)| skip.is_none() || Some(*idx) != skip)
        .map(|(_, x)| *x);

    let mut diffs = values_iter.tuple_windows().map(|(x, y)| x - y).peekable();

    let sig = diffs.peek().unwrap().signum();

    let first_invalid = diffs.position(|x| !(1..=3).contains(&x.abs()) || x.signum() != sig);

    match first_invalid {
        Some(x) if skip.is_none() => {
            tolerates_one_failure(input, Some(x + 1))
                || tolerates_one_failure(input, Some(x.saturating_sub(1)))
                || tolerates_one_failure(input, Some(x))
        }
        None => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::day_2::Day2;
    use common::Solution;

    const CASE_A: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day2.part_one(CASE_A), 2i64.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day2.part_two(CASE_A), 4usize.into())
    }
}
