use std::io::{Error, ErrorKind};

use aoc24::*;

use clap::Parser;
use criterion::Criterion;

/// Solver for Advent of code 2024 written in rust (author cBang)

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Day to solve
    #[arg(short)]
    day: u32,

    /// Part to solve
    #[arg(short)]
    part: u32,

    /// Benchmark solution with criterion
    #[arg(short)]
    benchmark: bool,
}

fn do_solving(d: &mut DayEnum, day: &u32, part: &u32) -> std::io::Result<()> {
    d.parse_input(&format!("src/input/day{}.txt", day))?;

    match part {
        1 => d.solve_part_one(),
        2 => d.solve_part_two(),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Expected -d [1 2]")),
    }?;
    Ok(())
}

fn bench_mark_wrapper(d: &mut DayEnum, day: &u32, part: &u32) -> std::io::Result<()> {
    let mut criterion = Criterion::default();
    criterion.bench_function(&format!("Day {} part {}", day, part), |b| {
        b.iter(|| do_solving(d, day, part))
    });
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let day = args.day;
    let mut d;

    aoc_macros::init_day!("d", "src/days/", "day");

    match args.benchmark {
        true => bench_mark_wrapper(&mut d, &day, &args.part)?,
        false => do_solving(&mut d, &day, &args.part)?,
    };

    let sol = d.get_solution();
    println!("Solution for Day {} Part {}: {}", args.day, args.part, sol);

    Ok(())
}
