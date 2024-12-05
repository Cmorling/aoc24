use std::time::Instant;
use aoc24::*;

fn main() -> std::io::Result<()>{
    let now = Instant::now();

    // specify which day to solve
    let mut d = D5p2::new();

    d.parse_input("src/input/day5.txt")?;
    d.solve()?;

    let sol = d.get_solution();
    println!("{}", sol);

    let done = Instant::now();
    println!("Executed in {:?}", done.duration_since(now));
    Ok(())
}
