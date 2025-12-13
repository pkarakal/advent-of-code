use common::{Answer, Solution};

#[derive(Debug, PartialEq)]
enum LightIndicatorStatus {
    On,
    Off,
}

impl From<char> for LightIndicatorStatus {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::On,
            '.' => Self::Off,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Machine {
    desired_lights: Vec<LightIndicatorStatus>,
    buttons: Vec<Vec<i64>>,
    voltage: Vec<i64>,
}

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> String {
        "Day 10".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        let machines = parse(input);
        machines.iter().filter_map(solve_a).sum::<usize>().into()
    }

    fn part_two(&self, input: &str) -> Answer {
        parse(input).iter().map(solve_b).sum::<usize>().into()
    }
}

fn build_augmented_matrix(machine: &Machine) -> Vec<Vec<u8>> {
    (0..machine.desired_lights.len())
        .map(|light_idx| {
            machine
                .buttons
                .iter()
                .map(|button| button.contains(&(light_idx as i64)) as u8)
                .chain(std::iter::once(match machine.desired_lights[light_idx] {
                    LightIndicatorStatus::On => 1,
                    LightIndicatorStatus::Off => 0,
                }))
                .collect()
        })
        .collect()
}

fn gaussian_eliminate(
    matrix: Vec<Vec<u8>>,
    num_buttons: usize,
) -> (Vec<Vec<u8>>, Vec<Option<usize>>) {
    let num_lights = matrix.len();
    let mut matrix = matrix;
    let mut pivot_cols = vec![None; num_lights];

    (0..num_buttons).fold(0, |pivot_row, col| {
        if pivot_row >= num_lights {
            return pivot_row;
        }

        (pivot_row..num_lights)
            .find(|&r| matrix[r][col] == 1)
            .map(|row| {
                matrix.swap(pivot_row, row);

                let pivot = matrix[pivot_row].clone();
                matrix
                    .iter_mut()
                    .enumerate()
                    .filter(|(r, _)| *r != pivot_row)
                    .filter(|(_, row)| row[col] == 1)
                    .for_each(|(_, row)| {
                        (col..=num_buttons).for_each(|c| row[c] ^= pivot[c]);
                    });

                pivot_cols[pivot_row] = Some(col);
                pivot_row + 1
            })
            .unwrap_or(pivot_row)
    });

    (matrix, pivot_cols)
}

fn is_inconsistent(matrix: &[Vec<u8>], num_buttons: usize) -> bool {
    matrix
        .iter()
        .any(|row| row[..num_buttons].iter().all(|&x| x == 0) && row[num_buttons] == 1)
}

fn extract_particular_solution(
    matrix: &[Vec<u8>],
    pivot_cols: &[Option<usize>],
    num_buttons: usize,
) -> Vec<u8> {
    let mut solution = vec![0u8; num_buttons];

    pivot_cols
        .iter()
        .enumerate()
        .filter_map(|(r, &col_opt)| col_opt.map(|col| (col, matrix[r][num_buttons])))
        .for_each(|(col, value)| solution[col] = value);

    solution
}

fn compute_nullspace(
    matrix: &[Vec<u8>],
    pivot_cols: &[Option<usize>],
    num_buttons: usize,
) -> Vec<Vec<u8>> {
    let pivot_columns: Vec<usize> = pivot_cols.iter().filter_map(|&c| c).collect();
    let is_pivot: Vec<bool> = (0..num_buttons)
        .map(|c| pivot_columns.contains(&c))
        .collect();

    (0..num_buttons)
        .filter(|&c| !is_pivot[c])
        .map(|free_col| {
            let mut nullspace_vec = vec![0u8; num_buttons];
            nullspace_vec[free_col] = 1;

            pivot_cols
                .iter()
                .enumerate()
                .rev()
                .filter_map(|(r, &col_opt)| col_opt.map(|col| (r, col)))
                .filter(|(r, _)| matrix[*r][free_col] == 1)
                .for_each(|(_, col)| nullspace_vec[col] ^= 1);

            nullspace_vec
        })
        .collect()
}

fn find_minimum_solution(particular: Vec<u8>, nullspace: Vec<Vec<u8>>) -> usize {
    let num_combinations = 1 << nullspace.len();

    (0..num_combinations)
        .map(|mask| {
            let mut candidate = particular.clone();

            nullspace
                .iter()
                .enumerate()
                .filter(|(i, _)| (mask >> i) & 1 == 1)
                .for_each(|(_, null_vec)| {
                    candidate
                        .iter_mut()
                        .zip(null_vec.iter())
                        .for_each(|(c, &n)| *c ^= n);
                });

            candidate.iter().filter(|&&x| x == 1).count()
        })
        .min()
        .unwrap_or(0)
}

fn solve_a(machine: &Machine) -> Option<usize> {
    let num_buttons = machine.buttons.len();
    let matrix = build_augmented_matrix(machine);
    let (reduced_matrix, pivot_cols) = gaussian_eliminate(matrix, num_buttons);

    if is_inconsistent(&reduced_matrix, num_buttons) {
        return None;
    }

    let particular = extract_particular_solution(&reduced_matrix, &pivot_cols, num_buttons);
    let nullspace = compute_nullspace(&reduced_matrix, &pivot_cols, num_buttons);

    Some(find_minimum_solution(particular, nullspace))
}

fn solve_b(machine: &Machine) -> usize {
    let binary_buttons = get_binary_buttons(&machine.buttons);
    let subset_xors: Vec<_> = subsets(&binary_buttons)
        .iter()
        .map(|subset| (subset.clone(), subset.iter().fold(0, |a, &b| a ^ b)))
        .collect();
    fewest_joltage_presses_recur(&subset_xors, &machine.voltage).unwrap()
}

fn fewest_joltage_presses_recur(
    subset_xors: &[(Vec<u32>, u32)],
    joltages: &[i64],
) -> Option<usize> {
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }

    let binary_joltages = get_binary_joltages(joltages);
    let mut best: Option<usize> = None;

    for (subset, xor) in subset_xors {
        if *xor != binary_joltages {
            continue;
        }

        let new_joltages = get_new_joltages(joltages, subset);
        if new_joltages.iter().any(|&j| j < 0) {
            continue;
        }

        if let Some(count) = fewest_joltage_presses_recur(subset_xors, &new_joltages) {
            let total = subset.len() + 2 * count;
            best = Some(match best {
                Some(b) => b.min(total),
                None => total,
            });
        }
    }

    best
}
fn get_binary_buttons(buttons: &[Vec<i64>]) -> Vec<u32> {
    buttons
        .iter()
        .map(|b| b.iter().map(|&n| 1u32 << n).sum())
        .collect()
}

fn get_binary_joltages(joltages: &[i64]) -> u32 {
    joltages
        .iter()
        .enumerate()
        .map(|(i, &j)| ((j % 2) as u32) << i)
        .sum()
}

fn get_new_joltages(joltages: &[i64], subset: &[u32]) -> Vec<i64> {
    let mut new = Vec::with_capacity(joltages.len());
    for (i, &j) in joltages.iter().enumerate() {
        let presses_affecting_i = subset.iter().filter(|&&b| (b & (1 << i)) != 0).count() as i64;
        new.push((j - presses_affecting_i) / 2);
    }
    new
}

// Generate all subsets of a slice
fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut result = vec![Vec::new()];
    for &item in set {
        let mut new_subsets = result.clone();
        for subset in &mut new_subsets {
            subset.push(item);
        }
        result.extend(new_subsets);
    }
    result
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");

            let voltage = parts
                .clone()
                .last()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let desired_lights = parts
                .next()
                .unwrap()
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .map(LightIndicatorStatus::from)
                .collect::<Vec<_>>();

            let keys = parts
                .filter(|p| p.starts_with('(') && p.ends_with(')'))
                .map(|p| {
                    p.trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<usize>().unwrap() as i64)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            Machine {
                desired_lights,
                buttons: keys,
                voltage,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day_10::LightIndicatorStatus::{Off, On};
    use crate::day_10::{parse, Day10, Machine};
    use common::Solution;

    const CASE_A: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_parse() {
        let expected = vec![
            Machine {
                desired_lights: vec![Off, On, On, Off],
                buttons: vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                voltage: vec![3, 5, 4, 7],
            },
            Machine {
                desired_lights: vec![Off, Off, Off, On, Off],
                buttons: vec![
                    vec![0, 2, 3, 4],
                    vec![2, 3],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4],
                ],
                voltage: vec![7, 5, 12, 7, 2],
            },
            Machine {
                desired_lights: vec![Off, On, On, On, Off, On],
                buttons: vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2],
                ],
                voltage: vec![10, 11, 11, 5, 10, 5],
            },
        ];

        let got = parse(CASE_A);

        assert_eq!(expected.len(), got.len());
        assert_eq!(expected, got);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Day10.part_one(CASE_A), 7usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day10.part_two(CASE_A), 33usize.into())
    }
}
