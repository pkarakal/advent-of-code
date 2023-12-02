use common::{Answer, Solution};

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub struct Day1;

impl Solution for Day1 {
    fn name(&self) -> String {
        return "Day 1".into();
    }
    fn part_one(&self, input: &str) -> Answer {
        let mut sum: u32 = 0;
        for line in input.lines() {
            let mut digits = line.chars().filter_map(|i| i.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            sum += first * 10 + last
        }
        return sum.into();
    }

    fn part_two(&self, input: &str) -> Answer {
        let mut sum = 0;

        for line in input.lines() {
            let (first, last) = extract_digits(line);
            sum += first * 10 + last;
        }

        return sum.into();
    }
}

fn extract_digits(line: &str) -> (u32, u32) {
    let mut first = None;
    let mut last = 0;

    let mut extract = |c: u32| {
        first = first.or(Some(c));
        last = c
    };

    let bytes = line.as_bytes();

    for i in 0..bytes.len() {
        let c = bytes[i];
        if c.is_ascii_digit() {
            extract((c - b'0') as u32)
        } else {
            for (idx, digit) in DIGITS.iter().enumerate() {
                if bytes[i..].starts_with(digit.as_bytes()) {
                    extract((idx as u32) + 1);
                }
            }
        }
    }
    return (first.unwrap(), last);
}


#[cfg(test)]
mod test {
    use common::{Answer, Solution};
    use super::{Day1};

    const CASE_A: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const CASE_B: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn case_a() {
        assert_eq!(Day1.part_one(CASE_A), Answer::U32(142));
    }

    #[test]
    fn case_b() {
        assert_eq!(Day1.part_two(CASE_B), Answer::U32(281))
    }
}
