use std::{fs, io};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Answer {
    U32(u32),
    U64(u64),
    USize(usize)
}


impl From<u32> for Answer {
    fn from(n: u32) -> Self {
        Self::U32(n as u32)
    }
}

impl From<usize> for Answer {
    fn from(n: usize) -> Self {
        Self::USize(n as usize)
    }
}

impl From<u64> for Answer {
    fn from(n: u64) -> Self {
        Self::U64(n as u64)
    }
}

impl Display for Answer{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::USize(u) => write!(f, "{u}"),
            Answer::U32(n) => write!(f, "{n}"),
            Answer::U64(n) => write!(f, "{n}")
        }
    }
}


pub trait Solution {
    fn name(&self) -> String;
    fn part_one(&self, input: &str) -> Answer;
    fn part_two(&self, input: &str) -> Answer;
}

pub fn parse_file(file: &Path) -> io::Result<String>{
    return fs::read_to_string(file);
}

pub fn load_file(year: u32, day: u32) -> io::Result<String> {
    let file_name = format!("data/{}/{:02}.txt", year, day);
    let file = Path::new(&file_name);
    return parse_file(file);
}
