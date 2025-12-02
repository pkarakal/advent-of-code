use common::{Answer, Solution};

pub struct Day4;

impl Solution for Day4 {
    fn name(&self) -> String {
        "Day 4".into()
    }
    fn part_one(&self, input: &str) -> Answer {
        let items = parse(input);

        let _rows = items.len();
        let _cols = items[0].len();
    
        let target = "XMAS".chars().collect::<Vec<char>>();
        let rows = items.len() as i64;
        let cols = items[0].len() as i64;

        let directions: [(i64, i64); 8] = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        

        let mut count = 0i64;
        for i in 0..rows {
            for j in 0..cols {
                for &(dx, dy) in &directions {
                    let mut found = true;
                    for (k, &item) in target.iter().enumerate() {
                        let x = i + dx * k as i64;
                        let y = j + dy * k as i64;
                        if x < 0
                            || y < 0
                            || x >= rows
                            || y >= cols
                            || items[x as usize][y as usize] != item
                        {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }

        count.into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let items = parse(input);
        let rows = items.len() as i64;
        let cols = items[0].len() as i64;

        let mut count = 0usize;

        // Look for 'A' in the center and check if it forms an X-MAS
        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                if items[i as usize][j as usize] == 'A' {
                    // Check if this 'A' is the center of an X-MAS pattern
                    if is_xmas_pattern(&items, i, j) {
                        count += 1;
                    }
                }
            }
        }

        count.into()
    }
}

fn is_xmas_pattern(grid: &[Vec<char>], row: i64, col: i64) -> bool {
    // Get the four corners around 'A'
    // NW, NE, SE, SW
    let nw = grid[(row - 1) as usize][(col - 1) as usize];
    let ne = grid[(row - 1) as usize][(col + 1) as usize];
    let se = grid[(row + 1) as usize][(col + 1) as usize];
    let sw = grid[(row + 1) as usize][(col - 1) as usize];

    // There are 4 valid X-MAS patterns:
    // M.S   M.M   S.M   S.S
    // .A.   .A.   .A.   .A.
    // M.S   S.S   S.M   M.M

    // Pattern 1: M.S / M.S
    if nw == 'M' && ne == 'S' && se == 'S' && sw == 'M' {
        return true;
    }

    // Pattern 2: M.M / S.S
    if nw == 'M' && ne == 'M' && se == 'S' && sw == 'S' {
        return true;
    }

    // Pattern 3: S.M / S.M
    if nw == 'S' && ne == 'M' && se == 'M' && sw == 'S' {
        return true;
    }

    // Pattern 4: S.S / M.M
    if nw == 'S' && ne == 'S' && se == 'M' && sw == 'M' {
        return true;
    }

    false
}


fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}


#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_4::Day4;

    const CASE_A: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    const CASE_B: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";
    #[test]
    fn test_part_one() {
        assert_eq!(Day4.part_one(CASE_A), 18i64.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day4.part_two(CASE_B), 9usize.into())
    }
}