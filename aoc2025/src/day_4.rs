use common::{Answer, Solution};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::PartialEq;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum GridItem {
    Empty,
    Roll,
}

impl GridItem {
    fn from_char(c: char) -> GridItem {
        match c {
            '.' => GridItem::Empty,
            '@' => GridItem::Roll,
            _ => unreachable!(),
        }
    }
}

pub struct Day4;

impl Solution for Day4 {
    fn name(&self) -> String {
        "Day 4".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let rolls = parse(input);

        rolls
            .iter()
            .filter(|&&(row, col)| count_adjacent(&rolls, row, col) < 4)
            .count()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let rolls = parse(input);

        (0..)
            .scan(rolls, |rolls, _| {
                let accessible: HashSet<_> = rolls
                    .par_iter()
                    .filter(|&&(row, col)| count_adjacent(rolls, row, col) < 4)
                    .copied()
                    .collect();

                if accessible.is_empty() {
                    None
                } else {
                    let count = accessible.len();
                    *rolls = &*rolls - &accessible;
                    Some(count)
                }
            })
            .sum::<usize>()
            .into()
    }
}

fn count_adjacent(grid: &HashSet<(usize, usize)>, row: usize, col: usize) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions
        .iter()
        .filter_map(|(dr, dc)| {
            let new_row = row.checked_add_signed(*dr)?;
            let new_col = col.checked_add_signed(*dc)?;
            Some((new_row, new_col))
        })
        .filter(|pos| grid.contains(pos))
        .count()
}

fn parse(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| (col, GridItem::from_char(c)))
                .filter(|(_, c)| *c == GridItem::Roll)
                .map(move |(col, _)| (row, col))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day_4::Day4;
    use common::Solution;

    const CASE_A: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day4.part_one(CASE_A), 13usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day4.part_two(CASE_A), 43usize.into())
    }
}
