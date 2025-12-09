use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

struct Circuit {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Circuit {
    fn new(n: usize) -> Self {
        Circuit {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn get_union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut seen_roots = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            seen_roots.insert(root, self.size[root]);
        }
        seen_roots.into_values().collect()
    }
}

pub struct Day8;

impl Solution for Day8 {
    fn name(&self) -> String {
        "Day 8".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let grid = parse(input);
        let mut pairwise_distance = calculate_pairwise_distances(&grid);

        // Actually sort the distances
        pairwise_distance.sort_by_key(|e| e.2);

        let mut circuit = Circuit::new(grid.len());

        // Determine number of connections based on input size
        // Example has 20 points -> 10 connections
        // Full input likely has many more -> 1000 connections
        let num_connections = if grid.len() == 20 { 10 } else { 1000 };

        // Process the shortest pairs
        for &(i, j, _dist) in pairwise_distance.iter().take(num_connections) {
            circuit.get_union(i, j);
        }

        circuit
            .get_circuit_sizes()
            .iter()
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .product::<usize>()
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let grid = parse(input);
        let mut pairwise_distance = calculate_pairwise_distances(&grid);

        // Sort the distances
        pairwise_distance.sort_by_key(|e| e.2);

        let mut circuit = Circuit::new(grid.len());

        let (i, j) = pairwise_distance
            .iter()
            .filter_map(|&(i, j, _dist)| circuit.get_union(i, j).then_some((i, j)))
            .take(grid.len() - 1)
            .last()
            .unwrap();

        ((grid[i].x * grid[j].x) as usize).into()
    }
}

fn calculate_pairwise_distances(grid: &[Point]) -> Vec<(usize, usize, i64)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, _)| {
            ((i + 1)..grid.len()).map(move |j| {
                let dist_sq = grid[i].distance_squared(&grid[j]);
                (i, j, dist_sq)
            })
        })
        .collect()
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(",").collect_vec();
            assert_eq!(parts.len(), 3);
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day_8::{parse, Day8, Point};
    use common::Solution;

    const CASE_A: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_parse() {
        let expected_a = Point {
            x: 162,
            y: 817,
            z: 812,
        };

        let got = parse(CASE_A);
        assert_eq!(got.len(), 20);
        assert_eq!(expected_a, got[0]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Day8.part_one(CASE_A), 40usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day8.part_two(CASE_A), 25272usize.into())
    }
}
