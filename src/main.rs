use aoc24::*;

fn main() -> std::io::Result<()>{
    // specify which day to solve
    let mut d = D5p2::new();

    d.parse_input("src/input/day5.txt")?;
    d.solve()?;

    let sol = d.get_solution();
    println!("{}", sol);
    Ok(())
}
