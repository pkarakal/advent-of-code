use std::ops::{Add, Mul};
use std::collections::HashSet;
use common::{Answer, Solution};
use crate::day_10::Tile::StartingPosition;

#[derive(Debug, Default)]
struct Maze {
    start_position: Point,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn get(&self, point: &Point) -> Option<&Tile> {
        if point.y < 0 || point.x < 0 {
            return None;
        }
        self.tiles.get(point.y as usize)?.get(point.x as usize)
    }

    fn width(&self) -> usize {
        self.tiles.first().map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    fn next_point(&self, current_position: &Point) -> Point {
        *current_position + self.delta()
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
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
    fn connections(&self) -> Vec<Direction> {
        match self {
            Tile::VerticalPipe => vec![Direction::Up, Direction::Down],
            Tile::HorizontalPipe => vec![Direction::Left, Direction::Right],
            Tile::NorthEastBend => vec![Direction::Up, Direction::Right],
            Tile::NorthWestBend => vec![Direction::Up, Direction::Left],
            Tile::SouthWestBend => vec![Direction::Down, Direction::Left],
            Tile::SouthEastBend => vec![Direction::Down, Direction::Right],
            Tile::StartingPosition => vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            Tile::Ground => vec![],
        }
    }

    fn connects_to(&self, direction: Direction) -> bool {
        self.connections().contains(&direction)
    }

    fn next_direction(&self, from_direction: Direction) -> Option<Direction> {
        let connections = self.connections();
        if connections.len() != 2 {
            return None;
        }

        let opposite = from_direction.opposite();
        connections.into_iter().find(|&d| d != opposite)
    }
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

fn find_loop(maze: &Maze) -> Vec<Point> {
    let start = maze.start_position;

    // Find the first valid direction from start
    let first_dir = [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
        .iter()
        .find(|&&dir| {
            let next = dir.next_point(&start);
            if let Some(tile) = maze.get(&next) {
                // Check if the tile is not ground and connects back to us
                match tile {
                    Tile::Ground => false,
                    _ => tile.connects_to(dir.opposite())
                }
            } else {
                false
            }
        })
        .copied()
        .expect("No valid starting direction found");

    let mut path = vec![start];
    let mut current = first_dir.next_point(&start);
    let mut from_dir = first_dir;

    while current != start {
        path.push(current);

        let tile = maze.get(&current).expect("Invalid position in loop");
        let next_dir = tile.next_direction(from_dir).expect("No valid next direction");

        from_dir = next_dir;
        current = next_dir.next_point(&current);
    }

    path
}

fn determine_start_tile(maze: &Maze) -> Tile {
    let start = maze.start_position;
    let mut connections = Vec::new();

    for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
        let next = dir.next_point(&start);
        if let Some(tile) = maze.get(&next) {
            match tile {
                Tile::Ground => continue,
                _ => {
                    if tile.connects_to(dir.opposite()) {
                        connections.push(dir);
                    }
                }
            }
        }
    }

    connections.sort_by_key(|d| match d {
        Direction::Up => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    });

    match (connections.get(0), connections.get(1)) {
        (Some(&Direction::Up), Some(&Direction::Down)) => Tile::VerticalPipe,
        (Some(&Direction::Left), Some(&Direction::Right)) => Tile::HorizontalPipe,
        (Some(&Direction::Up), Some(&Direction::Right)) => Tile::NorthEastBend,
        (Some(&Direction::Up), Some(&Direction::Left)) => Tile::NorthWestBend,
        (Some(&Direction::Down), Some(&Direction::Left)) => Tile::SouthWestBend,
        (Some(&Direction::Down), Some(&Direction::Right)) => Tile::SouthEastBend,
        _ => Tile::StartingPosition,
    }
}

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> String {
        "Day 10".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        let maze = parse(input);
        let loop_path = find_loop(&maze);
        (loop_path.len() / 2).into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let mut maze = parse(input);
        let loop_path = find_loop(&maze);
        let loop_set: HashSet<Point> = loop_path.iter().copied().collect();

        // Replace starting position with actual tile
        let start_tile = determine_start_tile(&maze);
        maze.tiles[maze.start_position.y as usize][maze.start_position.x as usize] = start_tile;

        let mut count: usize = 0;

        // Use scanline algorithm
        for y in 0..maze.height() {
            let mut inside = false;
            let mut last_corner: Option<Direction> = None;

            for x in 0..maze.width() {
                let point = Point::new(x as i64, y as i64);

                if loop_set.contains(&point) {
                    let tile = maze.get(&point).unwrap();

                    match tile {
                        Tile::VerticalPipe => {
                            inside = !inside;
                        }
                        Tile::NorthEastBend => {
                            last_corner = Some(Direction::Up);
                        }
                        Tile::SouthEastBend => {
                            last_corner = Some(Direction::Down);
                        }
                        Tile::NorthWestBend => {
                            if last_corner == Some(Direction::Down) {
                                inside = !inside;
                            }
                            last_corner = None;
                        }
                        Tile::SouthWestBend => {
                            if last_corner == Some(Direction::Up) {
                                inside = !inside;
                            }
                            last_corner = None;
                        }
                        _ => {}
                    }
                } else if inside {
                    count += 1;
                }
            }
        }

        count.into()
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
            .find_map(|(x, c)| match c {
                StartingPosition => Some(Point::new(x as i64, y as i64)),
                _ => None
            }))
        .unwrap();

    Maze {
        start_position: start_point,
        tiles
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_10::Day10;

    const CASE_A: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const CASE_B: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const CASE_C: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const CASE_D: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test_part_one() {
        assert_eq!(Day10.part_one(CASE_A), 4usize.into());
        assert_eq!(Day10.part_one(CASE_B), 8usize.into());
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day10.part_two(CASE_C), 4usize.into());
        assert_eq!(Day10.part_two(CASE_D), 8usize.into());
    }
}
