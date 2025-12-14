use common::{Answer, Solution};
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, Vec<String>>;
type Memo = HashMap<(String, State), usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Neither,
    Dac,
    Fft,
    Both,
}

impl State {
    fn visit_dac(&self) -> State {
        match self {
            State::Neither | State::Dac => State::Dac,
            State::Both | State::Fft => State::Both,
        }
    }

    fn visit_fft(&self) -> State {
        match self {
            State::Neither | State::Fft => State::Fft,
            State::Both | State::Dac => State::Both,
        }
    }

    fn is_complete(&self) -> bool {
        matches!(self, State::Both)
    }
}

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> String {
        "Day 11".into()
    }

    fn part_one(&self, input: &str) -> Answer {
        let graph = parse(input);
        let mut memo = HashMap::new();
        let mut visiting = HashSet::new();

        count_paths("you", &graph, &mut memo, &mut visiting).into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let graph = parse(input);

        let mut visited = HashSet::new();
        let mut memo = HashMap::new();

        dfs("svr", &graph, &mut visited, State::Neither, &mut memo).into()
    }
}

fn count_paths(
    node: &str,
    graph: &Graph,
    memo: &mut HashMap<String, usize>,
    visiting: &mut HashSet<String>,
) -> usize {
    if node == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    if !visiting.insert(node.to_string()) {
        return 0;
    }

    let paths = graph[node]
        .iter()
        .map(|neighbor| count_paths(neighbor, graph, memo, visiting))
        .sum();

    visiting.remove(node);
    memo.insert(node.to_string(), paths);

    paths
}

fn dfs(
    node: &str,
    graph: &Graph,
    visited: &mut HashSet<String>,
    state: State,
    memo: &mut Memo,
) -> usize {
    let state = match node {
        "dac" => state.visit_dac(),
        "fft" => state.visit_fft(),
        _ => state,
    };

    if node == "out" {
        return if state.is_complete() { 1 } else { 0 };
    }

    let key = (node.to_string(), state);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if !visited.insert(node.to_string()) {
        return 0;
    }

    let total = graph
        .get(node)
        .into_iter()
        .flat_map(|neighbor| neighbor.iter())
        .fold(0, |acc, next| acc + dfs(next, graph, visited, state, memo));

    visited.remove(node);
    memo.insert(key, total);

    total
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph::new();

    input.lines().for_each(|line| {
        let (node, parts) = line.split_once(": ").unwrap();

        let neighbors = parts
            .split_whitespace()
            .map(|part| part.to_string())
            .collect::<Vec<_>>();

        graph.insert(node.to_string(), neighbors);
    });
    graph
}

#[cfg(test)]
mod test {
    use crate::day_11::{parse, Day11};
    use common::Solution;
    use std::collections::HashMap;

    const CASE_A: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const CASE_B: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_parse() {
        let mut expected = HashMap::new();

        expected.insert(
            "aaa".to_string(),
            vec!["you".to_string(), "hhh".to_string()],
        );
        expected.insert(
            "you".to_string(),
            vec!["bbb".to_string(), "ccc".to_string()],
        );
        expected.insert(
            "bbb".to_string(),
            vec!["ddd".to_string(), "eee".to_string()],
        );
        expected.insert(
            "ccc".to_string(),
            vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()],
        );
        expected.insert("ddd".to_string(), vec!["ggg".to_string()]);
        expected.insert("eee".to_string(), vec!["out".to_string()]);
        expected.insert("fff".to_string(), vec!["out".to_string()]);
        expected.insert("ggg".to_string(), vec!["out".to_string()]);
        expected.insert(
            "hhh".to_string(),
            vec!["ccc".to_string(), "fff".to_string(), "iii".to_string()],
        );
        expected.insert("iii".to_string(), vec!["out".to_string()]);

        let got = parse(CASE_A);
        assert_eq!(got, expected);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Day11.part_one(CASE_A), 5usize.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day11.part_two(CASE_B), 2usize.into())
    }
}
