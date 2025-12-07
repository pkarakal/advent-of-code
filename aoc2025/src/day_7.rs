use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug)]
enum Manifold {
    Empty,
    Start,
    Splitter,
    Beam,
}

impl Manifold {
    fn from_char(c: char) -> Manifold {
        match c {
            '.' => Manifold::Empty,
            'S' => Manifold::Start,
            '^' => Manifold::Splitter,
            '|' => Manifold::Beam,
            _ => unreachable!(),
        }
    }
}

pub struct Day7;

impl Solution for Day7 {
    fn name(&self) -> String {
        "Day 7".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let items = parse(input);

        let (starting_row, starting_col) = find_starting_position(&items);

        items[starting_row + 1..]
            .iter()
            .fold((HashSet::from([starting_col]), 0), |(cols, total), line| {
                let (next_cols, splits) = process_row(line, cols);
                (next_cols, total + splits)
            })
            .1
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let grid = parse(input);

        let (starting_row, starting_col) = find_starting_position(&grid);

        grid[starting_row + 1..]
            .iter()
            .fold(
                HashMap::from([(starting_col, 1)]),
                |position_counts, line| {
                    let width = line.len();

                    position_counts
                        .into_iter()
                        .filter(|&(col, _)| col < width)
                        .flat_map(|(col, count)| match line[col] {
                            Manifold::Splitter => vec![(col - 1, count), (col + 1, count)],
                            _ => vec![(col, count)],
                        })
                        .into_group_map()
                        .into_iter()
                        .map(|(col, counts)| (col, counts.into_iter().sum()))
                        .collect()
                },
            )
            .values()
            .sum::<usize>()
            .into()
    }
}

fn process_row(line: &[Manifold], active_columns: HashSet<usize>) -> (HashSet<usize>, usize) {
    let width = line.len();

    let splits_count = active_columns
        .iter()
        .filter(|&&col| col < width && line[col] == Manifold::Splitter)
        .count();

    let next_columns = active_columns
        .into_iter()
        .filter(|&col| col < width)
        .flat_map(|col| match line[col] {
            Manifold::Splitter => vec![col - 1, col + 1],
            _ => vec![col],
        })
        .collect();

    (next_columns, splits_count)
}

fn find_starting_position(grid: &[Vec<Manifold>]) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|m| *m == Manifold::Start)
                .map(|pos| (i, pos))
        })
        .unwrap()
}

fn parse(input: &str) -> Vec<Vec<Manifold>> {
    input
        .lines()
        .map(|line| line.chars().map(Manifold::from_char).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day_7::Day7;
    use common::Solution;

    const CASE_A: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day7.part_one(CASE_A), 21usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day7.part_two(CASE_A), 40usize.into())
    }
}
