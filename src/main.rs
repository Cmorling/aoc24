use aoc24::*;
use clap::Parser;
use std::time::Instant;

/// Solver for Advent of code 2024 written in rust (author cBang) 
///
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// Day to solve
    #[arg(short, long)]
    day: u32,

    /// Part to solve
    #[arg(short, long)]
    part: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();


    let day = args.day;
    let mut d;

    aoc_macros::init_day!("d", "src/days/", "day");

    let now = Instant::now();
    d.parse_input(&format!("src/input/day{}.txt", day))?;

    match args.part {
        1 => d.solve_part_one(),
        2 => d.solve_part_two(),
        _ => Ok(println!("Invalid part see cargo -h")),
    }?;

    let sol = d.get_solution();
    println!("Solution for Day {} Part {}: {}", args.day, args.part, sol);

    let done = Instant::now();
    println!("Executed in {:?}", done.duration_since(now));
    Ok(())
}
