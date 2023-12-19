use std::collections::HashMap;
use common::{Answer, lcm, Solution};

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(n: char) -> Self {
        match n {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!()
        }
    }
}

#[derive(Default, Debug)]
struct InstructionMap<'a> {
    instructions: Vec<Instruction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> InstructionMap<'a> {
    fn get(&'a self, pos: &'a str, instruction: &Instruction) -> &'a str {
        let (left, right) = self.nodes.get(pos).unwrap();
        match instruction {
            Instruction::Left => left,
            Instruction::Right => right
        }
    }
}

pub struct Day8;

impl Solution for Day8 {
    fn name(&self) -> String {
        return "Day 8".into();
    }

    fn part_one(&self, input: &str) -> Answer {
        let map = parse(input);

        let mut i = 0;
        let mut pos = "AAA";

        loop {
            pos = map.get(pos, &map.instructions[i % map.instructions.len()]);
            i += 1;

            if pos == "ZZZ" {
                break;
            }
        }

        return i.into();
    }

    fn part_two(&self, input: &str) -> Answer {
        let map = parse(input);


        let start_locations = map.nodes
            .keys()
            .filter(|x| x.ends_with('A'))
            .cloned()
            .collect::<Vec<&str>>();

        let mut cycles = Vec::new();

        for start in start_locations {
            let mut pos = start;
            let mut cycle_len = 0;
            let mut i = 0;
            loop {
                pos = map.get(pos, &map.instructions[i % map.instructions.len()]);
                i += 1;

                cycle_len += 1;
                if pos.ends_with('Z') {
                    cycles.push(cycle_len);
                    break;
                }
            }
        }

        cycles.into_iter().fold(1, |acc: usize, s:usize | lcm(acc, s)).into()
    }
}

fn parse(input: &str) -> InstructionMap {
    let (instructions, node_list) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .chars()
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Instruction>>();

    let mut nodes = HashMap::new();

    for item in node_list.lines().into_iter() {
        let (node_name, children_nodes) = item.split_once(" = ").unwrap();
        let (left, right) = children_nodes
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        nodes.insert(node_name.trim(), (left, right));
    }

    return InstructionMap {
        instructions,
        nodes,
    };
}

#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_8::Day8;

    const CASE_A: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const CASE_B: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day8.part_one(CASE_A), 6usize.into());
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day8.part_two(CASE_B), 6usize.into());
    }
}