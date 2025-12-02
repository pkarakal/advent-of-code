use std::collections::{HashMap, HashSet};
use common::{Answer, Solution};

pub struct Day3;

impl Solution for Day3 {
    fn name(&self) -> String {
        "Day 3".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        parse(input)
            .gears
            .iter()
            .filter(|g| g.part_number)
            .map(|x| x.value)
            .sum::<u32>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        parse(input)
            .ratios
            .iter()
            .filter(|(_, c)| c.len() == 2)
            .map(|(_, x)| x[0] * x[1])
            .sum::<u32>()
            .into()
    }
}

#[derive(Default, Debug)]
struct Gear {
    value: u32,
    part_number: bool,
}

#[derive(Default, Debug, Eq, Hash, PartialEq)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Debug)]
struct ParsedInput {
    gears: Vec<Gear>,
    ratios: HashMap<Coordinates, Vec<u32>>
}

fn parse(input: &str) -> ParsedInput {
    // vector of characters per line
    let chars: Vec<Vec<char>> = input
        .lines()
        .map(|c| c.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut symbols: HashSet<Coordinates> = HashSet::new();

    for (y_index, line) in chars.iter().enumerate() {
        for (x_index, c) in line.iter().enumerate() {
            if !"1234567890.".contains(*c) {
                symbols.insert(Coordinates::new(x_index, y_index));
            }
        }
    }

    let mut gears: Vec<Gear> = vec![];
    let mut ratios = HashMap::new();
    for (y_index, line) in chars.iter().enumerate() {
        let mut pos = None;

        let mut check = |pos, x| {
            if let Some(pos) = pos {
                let [start, end] = [pos, x - 1];

                let value = line[start..=end]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let mut part_number = false;
                for nx in (start as isize - 1)..=(end as isize + 1) {
                    for ny in (y_index as isize - 1)..=(y_index as isize + 1) {
                        if nx < 0 || ny < 0 {
                            continue;
                        }

                        let [nx, ny] = [nx as usize, ny as usize];
                        let pos = Coordinates::new(nx, ny);
                        part_number |= symbols.contains(&pos);

                        if symbols.contains(&pos) && chars[ny][nx] == '*' {
                            ratios.entry(pos).or_insert(Vec::new()).push(value);
                        }
                    }
                }

                gears.push(Gear { value, part_number });
            }
        };



        for (x_index, c) in line.iter().enumerate() {
            if c.is_numeric() {
                if pos.is_none(){
                    pos = Some(x_index)
                }
            } else {
                check(pos, x_index);
                pos = None
            }
        }

        check(pos, line.len());
    }


    ParsedInput{ gears, ratios}
}



#[cfg(test)]
mod test {
    use common::{Answer, Solution};
    use crate::day_3::Day3;

    const CASE_A: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_one(){
        assert_eq!(Day3.part_one(CASE_A), Answer::U32(4361))
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day3.part_two(CASE_A), Answer::U32(467835))
    }
}