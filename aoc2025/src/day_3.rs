use common::{Answer, Solution};

pub struct Day3;

impl Solution for Day3 {
    fn name(&self) -> String {
        "Day 3".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        // my naive solution before part 2
        // parse(input)
        //     .into_iter()
        //     .filter(|item| item.len() > 2)
        //     .map(|c| {
        //         c[..c.len() - 1]
        //             .iter()
        //             .rev()
        //             .fold((c[c.len() - 1], 0), |(max_right, best), &digit| {
        //                 let candidate = 10 * digit + max_right;
        //                 let new_best = best.max(candidate);
        //                 let new_max_right = max_right.max(digit);
        //
        //                 (new_max_right, new_best)
        //             })
        //             .1
        //     })
        //     .sum::<u32>()
        //     .into()

        parse(input)
            .into_iter()
            .map(|c| {
                top_k_digits(c, 2)
                    .iter()
                    .fold(0u64, |acc, &d| acc * 10 + d as u64)
            })
            .sum::<u64>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        parse(input)
            .into_iter()
            .map(|c| {
                top_k_digits(c, 12)
                    .iter()
                    .fold(0u64, |acc, &d| acc * 10 + d as u64)
            })
            .sum::<u64>()
            .into()
    }
}

fn top_k_digits(digits: Vec<u32>, k: usize) -> Vec<u32> {
    let n = digits.len();
    let mut to_remove = n.saturating_sub(k);

    digits
        .iter()
        .fold(Vec::with_capacity(k), |mut stack, &digit| {
            while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
                stack.pop();
                to_remove -= 1;
            }
            if stack.len() < k {
                stack.push(digit);
            } else {
                to_remove -= 1;
            }
            stack
        })
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::day_3::Day3;
    use common::Solution;

    const CASE_A: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day3.part_one(CASE_A), 357u64.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day3.part_two(CASE_A), 3121910778619u64.into())
    }
}
