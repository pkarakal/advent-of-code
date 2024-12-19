use std::ops::{Add, Mul};
use common::{Answer, Solution};
use crate::day_10::Tile::StartingPosition;

#[derive(Debug, Default)]
struct Maze {
    start_position: Point,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        return Point {
            x,
            y,
        };
    }

    fn rotate_clockwise(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    fn rotate_counter_clockwise(&self) -> Self {
        Point::new(-self.y, self.x)
    }

    fn distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    fn next_point(&self, current_position: &Point) -> Point {
        match self {
            Directions::Up => Point::new(current_position.x, current_position.y -1),
            Directions::Down => Point::new(current_position.x, current_position.y +1),
            Directions::Left => Point::new(current_position-1, current_position.y),
            Directions::Right => Point::new(current_position + 1, current_position.y)
        }
    }
}

#[derive(Debug)]
enum Tile {
    HorizontalPipe,
    VerticalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthEastBend,
    SouthWestBend,
    Ground,
    StartingPosition,
}

impl Tile {

}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::NorthEastBend,
            'J' => Tile::NorthWestBend,
            '7' => Tile::SouthWestBend,
            'F' => Tile::SouthEastBend,
            '.' => Tile::Ground,
            'S' => Tile::StartingPosition,
            _ => unreachable!()
        }
    }
}

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> String {
        return "Day 10".into();
    }

    fn part_one(&self, input: &str) -> Answer {
        let maze = parse(input);

        return 0u64.into();
    }

    fn part_two(&self, input: &str) -> Answer {
        todo!()
    }
}

fn parse(input: &str) -> Maze {
    let tiles = input
        .lines()
        .map(|x| x.chars().map(|c| c.into()).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    let start_point = tiles
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter()
            .enumerate()
            .find_map(|(x, c)| return match c{
                StartingPosition => Some(Point::new(x as i64, y as i64)),
                _ => None
            }))
        .unwrap();

    return Maze{
        start_position: start_point,
        tiles
    };
}

#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_10::Day10;

    const CASE_A: &str = ".....
.F-7.
.|.|.
.L-J.
.....";

    #[test]
    fn test_part_one() {
        assert_eq!(Day10.part_one(CASE_A), 4usize.into())
    }
}

