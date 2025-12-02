use std::fmt::{Display, Formatter};
use std::path::Path;
use std::{fs, io};

#[derive(Debug, PartialEq)]
pub enum Answer {
    I64(i64),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
}

impl From<u16> for Answer {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<i64> for Answer {
    fn from(n: i64) -> Self {
        Self::I64(n)
    }
}

impl From<u32> for Answer {
    fn from(n: u32) -> Self {
        Self::U32(n)
    }
}

impl From<usize> for Answer {
    fn from(n: usize) -> Self {
        Self::USize(n)
    }
}

impl From<u64> for Answer {
    fn from(n: u64) -> Self {
        Self::U64(n)
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::USize(u) => write!(f, "{u}"),
            Answer::U32(n) => write!(f, "{n}"),
            Answer::U64(n) => write!(f, "{n}"),
            Answer::I64(n) => write!(f, "{n}"),
            Answer::U16(n) => write!(f, "{n}"),
        }
    }
}

pub trait Solution {
    fn name(&self) -> String;
    fn part_one(&self, input: &str) -> Answer;
    fn part_two(&self, input: &str) -> Answer;
}

pub fn parse_file(file: &Path) -> io::Result<String> {
    fs::read_to_string(file)
}

pub fn load_file(year: u32, day: u32) -> io::Result<String> {
    let file_name = format!("data/{}/{:02}.txt", year, day);
    let file = Path::new(&file_name);
    parse_file(file)
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        // Rerun the function with b and the remainder of a / b
        _ => gcd(b, a % b),
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
