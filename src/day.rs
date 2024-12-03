pub trait Day: Default{

    fn new() -> Self {
        Default::default()
    }

    fn parse_input(&mut self, path: &str) -> std::io::Result<()>; 
    fn solve(&mut self) -> std::io::Result<()>;
    fn get_solution(self) -> String; 
}
