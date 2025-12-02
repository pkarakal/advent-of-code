use common::{Answer, Solution};

struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn derive(&self) -> Vec<Vec<i64>> {
        let mut derived = vec![self.values.clone()];

        while !derived.last().unwrap().iter().all(|&x| x == 0) {
            let last = derived.last().unwrap();

            let items = last
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect::<Vec<_>>();
            derived.push(items)
        }
        derived
    }

    fn predict(&self) -> i64 {
        self.derive().iter().filter_map(|v| v.last()).sum()
    }

    fn reverse(&mut self) -> &Self {
        self.values.reverse();
        self
    }

    fn extrapolate(&mut self) -> i64 {
        self.reverse().predict()
    }
}

pub struct Day9;

impl Solution for Day9 {
    fn name(&self) -> String {
        "Day 9".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        let items: Vec<Sequence> = parse(input);

        items
            .iter()
            .map(Sequence::predict)
            .sum::<i64>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let mut items: Vec<Sequence> = parse(input);

        items.
            iter_mut()
            .map(Sequence::extrapolate)
            .sum::<i64>()
            .into()
    }
}

fn parse(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|x| Sequence {
            values: x
                .split_whitespace()
                .map(|i|
                    i
                        .parse()
                        .unwrap()
                )
                .collect::<Vec<i64>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_9::Day9;

    const CASE_A: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day9.part_one(CASE_A), 114i64.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day9.part_two(CASE_A), 2i64.into())
    }
}
