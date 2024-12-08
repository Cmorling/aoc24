pub trait Day: Default {
    fn new() -> Self {
        Default::default()
    }

    fn parse_input(&mut self, path: &str) -> std::io::Result<()>;
    fn solve_part_one(&mut self) -> std::io::Result<()>;
    fn solve_part_two(&mut self) -> std::io::Result<()>;
    fn get_solution(&self) -> String;
}
