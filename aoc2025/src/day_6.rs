use common::{Answer, Solution};

#[derive(PartialEq, Debug)]
enum Sign {
    Multiplication,
    Addition,
}

impl Sign {
    fn from_char(c: char) -> Sign {
        match c {
            '+' => Sign::Addition,
            '*' => Sign::Multiplication,
            _ => unreachable!(),
        }
    }
}

pub struct Day6;

impl Solution for Day6 {
    fn name(&self) -> String {
        "Day 6".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let (items, signs) = parse(input);

        let items_transposed = transpose(items);
        solve(items_transposed, signs)
    }

    fn part_two(&self, input: &str) -> Answer {
        let (items, signs) = parse_b(input);

        solve(items, signs)
    }
}

fn solve(items: Vec<Vec<usize>>, signs: Vec<Sign>) -> Answer {
    items
        .iter()
        .zip(signs.iter())
        .map(|(item, sign)| match sign {
            Sign::Addition => item.iter().sum::<usize>(),
            Sign::Multiplication => item.iter().product::<usize>(),
        })
        .sum::<usize>()
        .into()
}

fn transpose(v: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i]).collect())
        .collect()
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Sign>) {
    let items = input
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let signs = input
        .lines()
        .rev()
        .take(1)
        .flat_map(|line| {
            line.split_whitespace()
                .map(|c| Sign::from_char(c.chars().next().unwrap()))
        })
        .collect::<Vec<Sign>>();

    (items, signs)
}

fn parse_b(input: &str) -> (Vec<Vec<usize>>, Vec<Sign>) {
    let lines: Vec<&str> = input.lines().collect();
    let number_lines = &lines[..lines.len() - 1];
    let max_len = number_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Collect all columns right-to-left as strings
    let columns: Vec<String> = (0..max_len)
        .rev()
        .map(|col_idx| {
            number_lines
                .iter()
                .filter_map(|line| line.chars().nth(col_idx))
                .collect()
        })
        .collect();

    let items: Vec<Vec<usize>> = columns
        .split(|col| col.trim().is_empty())
        .filter(|group| !group.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|col| col.chars().filter(|&c| c != ' ').collect::<String>())
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let signs: Vec<Sign> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .rev()
        .map(|c| Sign::from_char(c.chars().next().unwrap()))
        .collect();

    (items, signs)
}

#[cfg(test)]
mod test {
    use crate::day_6::{parse_b, Day6};
    use common::Solution;
    use itertools::assert_equal;

    const CASE_A: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day6.part_one(CASE_A), 4277556usize.into())
    }

    #[test]
    fn test_parse_b() {
        let expected = vec![
            vec![4, 431, 623],
            vec![175, 581, 32],
            vec![8, 248, 369],
            vec![356, 24, 1],
        ];
        let (got, _) = parse_b(CASE_A);
        assert_equal(expected, got);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day6.part_two(CASE_A), 3263827usize.into())
    }
}
