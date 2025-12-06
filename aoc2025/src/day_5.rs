use common::{Answer, Solution};

#[derive(PartialEq, Eq, Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

pub struct Day5;

impl Solution for Day5 {
    fn name(&self) -> String {
        "Day 5".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let (fresh, ingredients) = parse(input);

        ingredients
            .iter()
            .filter(|&&i| fresh.iter().any(|j| i >= j.start && i <= j.end))
            .count()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let (fresh, _) = parse(input);

        let mut ranges = fresh;

        ranges.sort_by_key(|r| r.start);

        ranges
            .iter()
            .fold(Vec::new(), |mut acc: Vec<Range>, range| {
                if let Some(last) = acc.last_mut() {
                    if range.start <= last.end + 1 {
                        last.end = last.end.max(range.end);
                    } else {
                        acc.push(range.clone());
                    }
                } else {
                    acc.push(range.clone());
                }
                acc
            })
            .iter()
            .map(|range| range.end.saturating_sub(range.start).saturating_add(1))
            .sum::<usize>()
            .into()
    }
}

fn parse(input: &str) -> (Vec<Range>, Vec<usize>) {
    let mut lines = input.lines();

    let fresh: Vec<Range> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .flat_map(|line| {
            line.split(',')
                .map(str::trim)
                .map(|s| {
                    let mut parts = s.split('-');
                    let start = parts.next().unwrap().parse::<usize>().unwrap();
                    let end = parts.next().unwrap().parse::<usize>().unwrap();
                    Range { start, end }
                })
                .collect::<Vec<Range>>()
        })
        .collect();

    let ingredients = lines
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    (fresh, ingredients)
}

#[cfg(test)]
mod test {
    use crate::day_5::Day5;
    use common::Solution;

    const CASE_A: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day5.part_one(CASE_A), 3usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day5.part_two(CASE_A), 14usize.into())
    }
}
