use common::{Answer, Solution};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

pub struct Day2;

impl Solution for Day2 {
    fn name(&self) -> String {
        "Day 2".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let ranges = parse(input);

        // 0u64.into()
        ranges
            .into_iter()
            .flat_map(|range| {
                (range.start..=range.end).filter(|&i| {
                    let n_str = i.to_string();
                    if n_str.len() % 2 != 0 {
                        return false;
                    }
                    let mid = n_str.len() / 2;
                    n_str[..mid] == n_str[mid..]
                })
            })
            .sum::<usize>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let ranges = parse(input);

        ranges
            .into_par_iter()
            .flat_map(|range| {
                (range.start..=range.end).into_par_iter().filter(|&i| {
                    let s = i.to_string();
                    let n_bytes = s.as_bytes();
                    let len = n_bytes.len();
                    let mid = len / 2;

                    (1..=mid).any(|pattern_len| {
                        len % pattern_len == 0
                            && n_bytes
                                .chunks_exact(pattern_len)
                                .all(|chunk| chunk == &n_bytes[0..pattern_len])
                    })
                })
            })
            .sum::<usize>()
            .into()
    }
}

fn parse(input: &str) -> Vec<Range> {
    input
        .lines()
        .map(str::trim)
        .flat_map(|line| {
            line.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut parts = s.split('-');
                    let start = parts.next().unwrap().parse::<usize>().unwrap();
                    let end = parts.next().unwrap().parse::<usize>().unwrap();
                    Range { start, end }
                })
                .collect::<Vec<Range>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day_2::{parse, Day2, Range};
    use common::Solution;

    const CASE_A: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parse() {
        let expected = vec![
            Range { start: 11, end: 22 },
            Range {
                start: 95,
                end: 115,
            },
            Range {
                start: 998,
                end: 1012,
            },
            Range {
                start: 1188511880,
                end: 1188511890,
            },
            Range {
                start: 222220,
                end: 222224,
            },
            Range {
                start: 1698522,
                end: 1698528,
            },
            Range {
                start: 446443,
                end: 446449,
            },
            Range {
                start: 38593856,
                end: 38593862,
            },
            Range {
                start: 565653,
                end: 565659,
            },
            Range {
                start: 824824821,
                end: 824824827,
            },
            Range {
                start: 2121212118,
                end: 2121212124,
            },
        ];
        let got = parse(CASE_A);
        assert_eq!(got, expected);
        assert_eq!(got[0], Range { start: 11, end: 22 });
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Day2.part_one(CASE_A), 1227775554usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day2.part_two(CASE_A), 4174379265usize.into())
    }
}
