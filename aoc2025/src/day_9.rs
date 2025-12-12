use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

pub struct Day9;

impl Solution for Day9 {
    fn name(&self) -> String {
        "Day 9".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        let tiles = parse(input);

        tiles
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| tiles[i + 1..].iter().map(move |p2| (p1, p2)))
            .filter(|(p1, p2)| p1.x != p2.x && p1.y != p2.y)
            .map(|(p1, p2)| ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1))
            .max()
            .unwrap_or(0)
            .into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let tiles = parse(input);

        let checker = BoundaryChecker::new(&tiles);

        let mut heap: BinaryHeap<_> = tiles
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| {
                tiles[i + 1..].iter().filter_map(move |p2| {
                    if p1.x != p2.x && p1.y != p2.y {
                        let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                        Some(RectCandidate {
                            area,
                            p1: *p1,
                            p2: *p2,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect();

        while let Some(candidate) = heap.pop() {
            if checker.is_valid_rectangle(candidate.p1, candidate.p2) {
                return candidate.area.into();
            }
        }

        0i64.into()
    }

    // this is my naive solution, which is too slow for the input and that's why I opted
    // to sample the edges instead of checking all the points
    //
    // fn part_two(&self, input: &str) -> Answer {
    //     let tiles = parse(input);
    //     tiles
    //         .iter()
    //         .enumerate()
    //         .flat_map(|(i, p1)| tiles[i + 1..].iter().map(move |p2| (p1, p2)))
    //         .filter(|(p1, p2)| p1.x != p2.x && p1.y != p2.y)
    //         .collect::<Vec<_>>()
    //         .par_iter()
    //         .filter_map(|(p1, p2)| {
    //             let min_x = p1.x.min(p2.x);
    //             let max_x = p1.x.max(p2.x);
    //             let min_y = p1.y.min(p2.y);
    //             let max_y = p1.y.max(p2.y);
    //
    //             let width = max_x - min_x;
    //             let height = max_y - min_y;
    //
    //             let should_sample = width > 100 || height > 100;
    //
    //             if !should_sample {
    //                 // Small rectangle - check full perimeter
    //                 let top_bottom_valid = (min_x..=max_x).all(|x| {
    //                     is_inside_or_on_polygon(&tiles, x, min_y)
    //                         && is_inside_or_on_polygon(&tiles, x, max_y)
    //                 });
    //
    //                 if !top_bottom_valid {
    //                     return None;
    //                 }
    //
    //                 let left_right_valid = ((min_y + 1)..max_y).all(|y| {
    //                     is_inside_or_on_polygon(&tiles, min_x, y)
    //                         && is_inside_or_on_polygon(&tiles, max_x, y)
    //                 });
    //
    //                 if left_right_valid {
    //                     Some((width + 1) * (height + 1))
    //                 } else {
    //                     None
    //                 }
    //             } else {
    //                 // Large rectangle - use sampling
    //                 let x_step = (width / 100).max(1);
    //                 let y_step = (height / 100).max(1);
    //
    //                 // Check corners first (fast rejection)
    //                 if !is_inside_or_on_polygon(&tiles, min_x, min_y)
    //                     || !is_inside_or_on_polygon(&tiles, max_x, min_y)
    //                     || !is_inside_or_on_polygon(&tiles, min_x, max_y)
    //                     || !is_inside_or_on_polygon(&tiles, max_x, max_y)
    //                 {
    //                     return None;
    //                 }
    //
    //                 // Sample edges
    //                 let edges_valid = (min_x..=max_x).step_by(x_step as usize).all(|x| {
    //                     is_inside_or_on_polygon(&tiles, x, min_y)
    //                         && is_inside_or_on_polygon(&tiles, x, max_y)
    //                 }) && (min_y..=max_y).step_by(y_step as usize).all(|y| {
    //                     is_inside_or_on_polygon(&tiles, min_x, y)
    //                         && is_inside_or_on_polygon(&tiles, max_x, y)
    //                 });
    //
    //                 if edges_valid {
    //                     Some((width + 1) * (height + 1))
    //                 } else {
    //                     None
    //                 }
    //             }
    //         })
    //         .max()
    //         .unwrap_or(0)
    //         .into()
    // }
}

#[derive(Eq, PartialEq)]
struct RectCandidate {
    area: i64,
    p1: Point,
    p2: Point,
}

impl Ord for RectCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

impl PartialOrd for RectCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct BoundaryChecker {
    polygon: Vec<Point>,
    horizontal_edges: Vec<(i64, i64, i64)>, // (y, x_min, x_max)
    vertical_edges: Vec<(i64, i64, i64)>,   // (x, y_min, y_max)
}

impl BoundaryChecker {
    fn new(polygon: &[Point]) -> Self {
        let mut horizontal_edges = Vec::new();
        let mut vertical_edges = Vec::new();

        for i in 0..polygon.len() {
            let p1 = &polygon[i];
            let p2 = &polygon[(i + 1) % polygon.len()];

            if p1.y == p2.y {
                let (x_min, x_max) = if p1.x < p2.x {
                    (p1.x, p2.x)
                } else {
                    (p2.x, p1.x)
                };
                horizontal_edges.push((p1.y, x_min, x_max));
            } else if p1.x == p2.x {
                let (y_min, y_max) = if p1.y < p2.y {
                    (p1.y, p2.y)
                } else {
                    (p2.y, p1.y)
                };
                vertical_edges.push((p1.x, y_min, y_max));
            }
        }

        Self {
            polygon: polygon.to_vec(),
            horizontal_edges,
            vertical_edges,
        }
    }

    fn is_valid_rectangle(&self, p1: Point, p2: Point) -> bool {
        let (x_min, x_max) = if p1.x < p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };
        let (y_min, y_max) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };

        let width = x_max - x_min;
        let height = y_max - y_min;

        self.check_rectangle(x_min, x_max, y_min, y_max, width, height)
    }

    fn check_rectangle(
        &self,
        x_min: i64,
        x_max: i64,
        y_min: i64,
        y_max: i64,
        width: i64,
        height: i64,
    ) -> bool {
        if !self.is_inside_or_on_boundary(x_min, y_min)
            || !self.is_inside_or_on_boundary(x_max, y_min)
            || !self.is_inside_or_on_boundary(x_min, y_max)
            || !self.is_inside_or_on_boundary(x_max, y_max)
        {
            return false;
        }

        let x_step = (width / 100).max(1);
        let y_step = (height / 100).max(1);

        let mut x = x_min;
        while x <= x_max {
            if !self.is_inside_or_on_boundary(x, y_min) || !self.is_inside_or_on_boundary(x, y_max)
            {
                return false;
            }
            x += x_step;
        }

        let mut y = y_min + y_step;
        while y < y_max {
            if !self.is_inside_or_on_boundary(x_min, y) || !self.is_inside_or_on_boundary(x_max, y)
            {
                return false;
            }
            y += y_step;
        }

        true
    }

    fn is_inside_or_on_boundary(&self, x: i64, y: i64) -> bool {
        for &(edge_y, x_min, x_max) in &self.horizontal_edges {
            if edge_y == y && x >= x_min && x <= x_max {
                return true;
            }
        }

        for &(edge_x, y_min, y_max) in &self.vertical_edges {
            if edge_x == x && y >= y_min && y <= y_max {
                return true;
            }
        }

        self.is_inside_polygon(x, y)
    }

    fn is_inside_polygon(&self, x: i64, y: i64) -> bool {
        let mut inside = false;

        for i in 0..self.polygon.len() {
            let p1 = &self.polygon[i];
            let p2 = &self.polygon[(i + 1) % self.polygon.len()];

            if (p1.y <= y) != (p2.y <= y) && x < (p2.x - p1.x) * (y - p1.y) / (p2.y - p1.y) + p1.x {
                inside = !inside;
            }
        }

        inside
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(",").collect_vec();
            assert_eq!(parts.len(), 2);
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        })
        .collect()
}

// // Check if point is inside the polygon OR on its boundary
// fn is_inside_or_on_polygon(polygon: &[Point], x: i64, y: i64) -> bool {
//     // First check if point is on an edge
//     for i in 0..polygon.len() {
//         let p1 = &polygon[i];
//         let p2 = &polygon[(i + 1) % polygon.len()];
//
//         // Check if point is on this edge
//         // For horizontal edge
//         if p1.y == p2.y && p1.y == y {
//             let min_x = p1.x.min(p2.x);
//             let max_x = p1.x.max(p2.x);
//             if x >= min_x && x <= max_x {
//                 return true;
//             }
//         }
//
//         // For vertical edge
//         if p1.x == p2.x && p1.x == x {
//             let min_y = p1.y.min(p2.y);
//             let max_y = p1.y.max(p2.y);
//             if y >= min_y && y <= max_y {
//                 return true;
//             }
//         }
//     }
//
//     // If not on boundary, check if inside using ray casting
//     is_inside_polygon(polygon, x, y)
// }
//
// fn is_inside_polygon(polygon: &[Point], x: i64, y: i64) -> bool {
//     (0..polygon.len())
//         .filter(|&i| {
//             let p1 = &polygon[i];
//             let p2 = &polygon[(i + 1) % polygon.len()];
//
//             (p1.y <= y) != (p2.y <= y) && x < (p2.x - p1.x) * (y - p1.y) / (p2.y - p1.y) + p1.x
//         })
//         .count()
//         % 2
//         == 1
// }

#[cfg(test)]
mod test {
    use crate::day_9::{parse, Day9, Point};
    use common::Solution;

    const CASE_A: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_parse() {
        let expected_a = Point { x: 7, y: 1 };

        let got = parse(CASE_A);
        assert_eq!(got.len(), 8);
        assert_eq!(expected_a, got[0]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Day9.part_one(CASE_A), 50i64.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day9.part_two(CASE_A), 24i64.into())
    }
}
