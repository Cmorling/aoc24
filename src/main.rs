use aoc24::*;

fn main() -> std::io::Result<()>{
    // specify which day to solver
    let mut d = D1p2::new();

    d.parse_input("src/input/day1.txt")?;
    d.solve()?;

    let sol = d.get_solution();
    println!("{}", sol);
    Ok(())
}
