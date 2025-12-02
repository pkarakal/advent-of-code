use common::{Answer, Solution};

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn is_possible(&self) -> bool {
        self.red <= VALID_GAME.red && self.green <= VALID_GAME.green && self.blue <= VALID_GAME.blue
    }

    fn get_min_count_to_possible(&self, right: &Self) -> Self {
        Self {
            red: self.red.max(right.red),
            green: self.green.max(right.green),
            blue: self.blue.max(right.blue),
        }
    }
}


const VALID_GAME: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

pub struct Day2;

impl Solution for Day2 {
    fn name(&self) -> String {
        "Day 2".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        parse_lines(input)
            .iter()
            .enumerate()
            .filter(|(_, games)| {
                games.iter().all(|game| game.is_possible())
            })
            .map(|i| i.0 + 1)
            .sum::<usize>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        parse_lines(input)
            .iter()
            .map(|games| {
                let mut min_count = CubeSet::default();
                for game in games {
                    min_count = min_count.get_min_count_to_possible(game);
                }
                min_count.red * min_count.green * min_count.blue
            })
            .sum::<u32>()
            .into()
    }
}

fn parse_lines(input: &str) -> Vec<Vec<CubeSet>> {
    input
        .lines()
        .map(|line| {
            let cubes = line.split_once(':').unwrap().1;
            let mut sets = vec![];
            for set in cubes.split(';') {
                let mut cube_set = CubeSet::default();
                for cube in set.trim().split(',') {
                    let mut iter = cube.split_whitespace();
                    let count = iter.next().unwrap().parse::<u32>().unwrap();
                    let color = iter.next().unwrap();
                    match color {
                        "red" => cube_set.red += count,
                        "green" => cube_set.green += count,
                        "blue" => cube_set.blue += count,
                        _ => unreachable!()
                    }
                }
                sets.push(cube_set)
            }
            sets
        }).collect::<Vec<Vec<CubeSet>>>()
}


#[cfg(test)]
mod test {
    use common::{Answer, Solution};
    use super::{Day2};

    const CASE_A: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    const CASE_B: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";


    #[test]
    fn test_part_one() {
        assert_eq!(Day2.part_one(CASE_A), Answer::USize(8))
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day2.part_two(CASE_B), Answer::U32(2286))
    }
}