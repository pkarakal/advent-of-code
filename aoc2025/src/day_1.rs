use common::{Answer, Solution};

enum Sign {
    Plus = 1,
    Minus = -1,
}

struct Direction {
    sign: Sign,
    distance: u16
}


pub struct Day1;

impl Solution for Day1 {
    fn name(&self) -> String {
        return "Day 1".into();
    }
    fn part_one(&self, input: &str) -> Answer {
        let directions = parse(input);

        directions.into_iter().fold((50u16, 0u16), |(pos, item), d| {
            // convert Sign into a ±1 multiplier
            let delta = match d.sign {
                Sign::Plus  =>  d.distance as i32,
                Sign::Minus => -(d.distance as i32),
            };

            let new_pos = ((pos as i32 + delta).rem_euclid(100)) as u16;

            let new_item = if new_pos == 0 {
                item + 1
            } else {
                item
            };

            (new_pos, new_item)
        }).1.into()
    }
    fn part_two(&self, input: &str) -> Answer {
        let directions = parse(input);

        directions.into_iter().fold((50u16, 0u16), |(pos, count), d|{
            // convert Sign into a ±1 multiplier
            let delta = match d.sign {
                Sign::Plus  =>  d.distance as i32,
                Sign::Minus => -(d.distance as i32),
            };

            let raw = pos as i32 + delta;
            let new_pos = raw.rem_euclid(100) as u16;

            let times_at_zero = match raw {
                // Landed exactly on 0 after rotation
                0 => 1,
                // Wrapped around going right: count how many complete circles (each passes through 0)
                n if n >= 100 => n.div_euclid(100),
                // Wrapped around going left: count complete circles plus one crossing if we didn't start at 0
                n if n < 0 => (n.abs().div_euclid(100)) + (pos != 0) as i32,
                // range [1-99]
                _ => 0,
            };


            (new_pos, count + times_at_zero as u16)
        }).1.into()
    }
}

fn parse(input: &str) -> Vec<Direction> {
    input.lines().map(str::trim).map(|line|{
        let (left, right) = line.split_at(1);
        let sign = match left {
            "L" => Sign::Minus,
            "R" => Sign::Plus,
            _ => unreachable!()
        };
        Direction{sign, distance: right.parse::<u16>().unwrap()}
    }).collect()
}

#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_1::Day1;

    const CASE_A: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    const CASE_B: &str = "L1000";
    const CASE_C: &str = "L950";
    #[test]
    fn test_part_one() {
        assert_eq!(Day1.part_one(CASE_A), 3u16.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day1.part_two(CASE_A), 6u16.into());
        assert_eq!(Day1.part_two(CASE_B), 10u16.into());
        assert_eq!(Day1.part_two(CASE_C), 10u16.into());
    }
}