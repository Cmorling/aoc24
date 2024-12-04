use aoc24::*;

fn main() -> std::io::Result<()>{
    // specify which day to solve
    let mut d = D4p1::new();

    d.parse_input("src/input/day4.txt")?;
    d.solve()?;

    let sol = d.get_solution();
    println!("{}", sol);
    Ok(())
}
