use clap::Parser;
use common::Solution;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct AdventOfCode {
    /// Year to use
    #[arg(short, long, default_value_t = 2023)]
    year: u32,

    /// Day which problem to run
    #[arg(short, long, default_value_t = 1)]
    day: u32,

    /// Part specifies which part of the given day to run
    #[arg(short, long, default_value_t = 1)]
    part: u32
}



fn main() {

    let args = AdventOfCode::parse();

    let solutions = get_solutions_for_year(args.year);

    let solution = match solutions.get((args.day as usize) - 1) {
        Some(s) => s,
        None => {
            println!("No solution for day {} in year {}", args.day, args.year);
            return;
        }
    };

    let data = common::load_file(args.year, args.day).unwrap();

    let result = match args.part {
        1 => solution.part_one(&data),
        2 => solution.part_two(&data),
        _ => unimplemented!()
    };

    println!("Result = {}", result);
}

fn get_solutions_for_year<'a>(year: u32) -> Vec<&'a dyn Solution> {
    match year  {
        2023 => aoc2023::ALL.to_vec(),
        2024 => aoc2024::ALL.to_vec(),
        _ => unimplemented!()
    }
}
