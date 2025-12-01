use common::{Answer, Solution};
use regex::Regex;

pub struct Day3;

impl Solution for Day3 {
    fn name(&self) -> String {
        return "Day 3".into();
    }
    fn part_one(&self, input: &str) -> Answer {
        let items = parse_a(input);
        items.iter().fold(0, |sum, x| {
            sum + (x.iter().fold(1, |mul, i| mul*i))
        }).into()
    }
    fn part_two(&self, input: &str) -> Answer {
        let items = parse_b(input);
        items.iter().fold(0, |sum, x| {
            sum + (x.iter().fold(1, |mul, i| mul*i))
        }).into()
    }
}

fn parse_a(input: &str) -> Vec<Vec<usize>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    input.lines().map(str::trim).flat_map(|line| {
        re.captures_iter(line)
        .filter_map(|capture| {
            let mut items = vec![];
            let x = capture.get(1).unwrap().as_str().parse::<usize>().ok()?;
            let y = capture.get(2).unwrap().as_str().parse::<usize>().ok()?;
            items.push(x);
            items.push(y);
            Some(items)
        })
    }).collect()
}

struct Parser {
    chars: Vec<char>,
    idx: usize
}

impl Parser {
    pub fn new(input: &str) -> Self {
        return Parser {
            chars: input.chars().collect(),
            idx: 0
        }
    }

    pub fn expect(&mut self, str: &str) -> bool {
        let valid = self.idx + str.len() < self.chars.len()
            && self.chars[self.idx..self.idx + str.len()]
                .iter()
                .zip(str.chars())
                .all(|(&a, b)| a == b);

        if valid {
            self.idx += str.len();
        }

        valid
    }

    pub fn number(&mut self) -> Option<usize> {
        let mut working = String::new();
        while self.chars[self.idx].is_ascii_digit() && self.idx < self.chars.len() {
            working.push(self.chars[self.idx]);
            self.idx += 1;
        }
        working.parse::<usize>().ok()
    }

    pub fn advance(&mut self, count: usize) {
        self.idx += count;
    }

    pub fn is_eof(&self) -> bool {
        self.idx >= self.chars.len()
    }

}

fn parse_b(input: &str) -> Vec<Vec<usize>> {
    let mut out = vec![];

    let mut parser = Parser::new(input);
    let mut active = true;

    while !parser.is_eof() {
        active |= parser.expect("do()");
        active &= !parser.expect("don't()");

        if parser.expect("mul(") {
            let Some(a) = parser.number() else {continue};
            if !parser.expect(",") {
                continue;
            }
            let Some(b) = parser.number() else {continue};
            if !parser.expect(")") {
                continue;
            }
            if active {
                out.push(vec![a,b]);
            }
        } else {
            parser.advance(1);
        }
    }
    out

}


#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_3::Day3;

    const CASE_A: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
    const CASE_B: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
    #[test]
    fn test_part_one() {
        assert_eq!(Day3.part_one(CASE_A), 161usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day3.part_two(CASE_B), 48usize.into())
    }
}