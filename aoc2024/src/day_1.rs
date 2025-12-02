use common::{Answer, Solution};

pub struct Day1;

impl Solution for Day1 {
    fn name(&self) -> String {
        "Day 1".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let (mut left, mut right) = parse(input);
        left.sort();
        right.sort();

        let item = left.iter().enumerate().fold(0, |sum, (idx, val)| {
            sum + (val - right.get(idx).unwrap()).abs()
        });
        item.into()
    }
    fn part_two(&self, input: &str) -> Answer {
        let (left, right) = parse(input);

        let item = left.iter().fold(0, |sum, val| {
            let occurrences = right.iter().filter(|x| x == &val).count() as i64;

            let score = val * occurrences;
            sum + score
        });

        item.into()
    }
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(str::trim)
        .fold((vec![], vec![]), |mut values, line| {
            let distances: Vec<_> = line
                .split_whitespace()
                .map(|string| string.parse().unwrap())
                .collect();

            values.0.push(distances[0]);
            values.1.push(distances[1]);

            values
        })
}

#[cfg(test)]
mod test {
    use crate::day_1::Day1;
    use common::Solution;

    const CASE_A: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";
    #[test]
    fn test_part_one() {
        assert_eq!(Day1.part_one(CASE_A), 11i64.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day1.part_two(CASE_A), 31i64.into())
    }
}
