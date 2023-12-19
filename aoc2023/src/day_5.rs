use common::{Answer, Solution};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Day5;

impl Solution for Day5 {
    fn name(&self) -> String {
        return "Day 5".into();
    }

    fn part_one(&self, input: &str) -> Answer {
        // println!("{input}");
        let RangeResult  {seeds, mappings}  = parse(input);

        let min = seeds.par_iter().map(|s| find_location(&mappings, *s)).min().unwrap();

        return min.into();
    }

    // TODO: optimize part two and fix off-by-one bug
    fn part_two(&self, input: &str) -> Answer {
        let RangeResult {seeds, mappings} = parse(input);
        let seed_ranges =
            seeds
                .chunks_exact(2)
                .flat_map(|chunk| {
                    chunk[0]..chunk[0]+chunk[1]
                })
                .collect::<Vec<_>>();

        seed_ranges.par_iter()
            .map(|s| find_location(&mappings, *s))
            .min()
            .unwrap()
            .into()
    }
}


fn parse(input: &str) -> RangeResult {
    let mut parse_result = RangeResult::default();
    let mut input_categories = input.split("\n\n");

    let seeds = input_categories
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    parse_result.seeds = seeds;

    let mut maps = vec![];

    let ranges = input_categories
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.split(":").last().unwrap())
        .collect::<Vec<&str>>();


    for range in ranges {
        let mut map_ranges: Vec<Conversion> = vec![];

        for line in range.lines().filter(|x| !x.is_empty()) {
            let parts = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u64>>();


            let mut iter_parts = parts.into_iter();

            map_ranges.push(Conversion {
                destination: iter_parts.next().unwrap(),
                source: iter_parts.next().unwrap(),
                length: iter_parts.next().unwrap(),
            });
        }
        maps.push(CategoryMap{ ranges: map_ranges })
    }
    parse_result.mappings = maps;

    parse_result
}


#[derive(Default, Debug)]
struct RangeResult {
    seeds: Vec<u64>,
    mappings: Vec<CategoryMap>,
}


#[derive(Default, Debug)]
struct CategoryMap {
    ranges: Vec<Conversion>
}

impl CategoryMap {
    fn transform(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|x| x.convert(value))
            .unwrap_or(value)
    }
}

// Step through the maps and until we find the location
fn find_location(maps: &Vec<CategoryMap>, location: u64) -> u64 {
    maps.iter().fold(location, |loc, map| map.transform(loc))
}

#[derive(Default, Debug)]
struct Conversion {
    destination: u64,
    source: u64,
    length: u64,
}

impl Conversion{
    fn convert(&self, location: u64) -> Option<u64> {
        // Check if location is within range
        let lower_bound = self.source;
        let upper_bound = self.source + self.length;
        let bounds = lower_bound..=upper_bound;

        if !bounds.contains(&location) {
            return None;
        }

        Some(self.destination + location - self.source)
    }
}


#[cfg(test)]
mod test {
    use common::Solution;
    use crate::day_5::Day5;

    const CASE_A: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part_one() {
        assert_eq!(Day5.part_one(CASE_A), 35u64.into())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day5.part_two(CASE_A), 46u64.into())
    }
}

