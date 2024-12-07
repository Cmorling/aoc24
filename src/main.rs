use aoc24::*;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let now = Instant::now();

    // specify which day to solve
    let mut d = D7p2::new();

    d.parse_input("src/input/day7.txt")?;
    let a_fop = Instant::now();
    d.solve()?;

    let sol = d.get_solution();
    println!("{}", sol);

    let done = Instant::now();
    println!("Executed in {:?}", done.duration_since(now));
    println!("Excluding Input parsing {:?}", a_fop.duration_since(now));
    Ok(())
}
