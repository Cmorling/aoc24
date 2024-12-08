use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::{BufReader, BufRead};
use glam::IVec2;

use crate::day::Day;

#[derive(Default, Debug)]

struct Guard {
    position: IVec2,
    direction: IVec2,
    placed_obstacles: HashSet<IVec2>,
}


impl Guard {
    fn new() -> Self {
        Default::default()
    }

    fn init(&mut self, pos: IVec2, dir: IVec2) {
        self.position = pos;
        self.direction = dir;
    }
   
    fn advance(&mut self, grid: &mut HashMap<IVec2, Square>, is_root: bool, is_p2: bool) -> Option<bool> {

        match grid.get_mut(&self.position) {
            Some(sq) => sq.visited = true,
            None => return None,
        }
        
        let mut empty = false;
        while !empty {
            match grid.get(&(self.position + self.direction)) {
                Some(sq) => match sq.obstacle {
                    true => {
                        self.direction = self.rotate(&self.direction.clone());
                    },
                    false => {
                        empty = true;
                    }
                },
                None => {
                    return None;
                },
            }
        }

        if is_root && is_p2 { 
            let try_place = self.position + self.direction;

            if let Some(sq) = grid.get(&try_place) {
                if !sq.visited {
                    let mut shadow = Guard::new();
                    shadow.init(self.position, self.direction);

                    let mut grid_cpy = grid.clone();
                    grid_cpy.entry(try_place).or_insert(Square{visited: false, obstacle: false}).obstacle = true;

                    let mut visited_states: HashSet<(IVec2, IVec2)> = HashSet::new();
                    let mut should_exit = false;

                    while !should_exit {
                        let (current_position, currrent_direction) = (shadow.position, shadow.direction);
                        if !visited_states.insert((current_position, currrent_direction)) {
                            self.placed_obstacles.insert(try_place);
                            should_exit = true;
                        }
                        let res = shadow.advance(&mut grid_cpy, false, true);
                        if res.is_none() {
                            should_exit = true;
                        }
                    }
                }    
            }
             
        }

        self.position += self.direction;
        
        Some(false) 
    }

    fn rotate(&self, v: &IVec2) -> IVec2 {
        IVec2::new(-v.y, v.x)
    }
}

#[derive(Debug, Copy, Clone)]
struct Square {
    visited: bool,
    obstacle: bool,
}

#[derive(Default, Debug)]

pub struct D6 {
    grid: HashMap<IVec2, Square>,
    guard: Guard,
    result: i32,
}

impl Day for D6 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        (self.grid, self.guard) = reader
            .lines()
            .enumerate()
            .fold(
                (HashMap::new(), Guard::new()),
                |(mut grid, mut guard), (y, line)| {
                    line
                        .unwrap()
                        .chars()
                        .enumerate()
                        .for_each(|(x, c)| {
                             match c {
                                '.' => {grid.insert(IVec2::new(x as i32, y as i32), Square{visited: false, obstacle: false});},
                                '#' => {grid.insert(IVec2::new(x as i32, y as i32), Square{visited: false, obstacle: true});},
                                '^' => {
                                    guard.init(IVec2::new(x as i32, y as i32), IVec2::new(0, -1));
                                    grid.insert(IVec2::new(x as i32, y as i32), Square{visited: true, obstacle: false});
                                },
                                _ => (),
                            };
                        });
                    (grid, guard)
                });
        Ok(())
    }

    fn solve_part_one(&mut self) -> std::io::Result<()>{
        while self.guard.advance(&mut self.grid, true, false).is_some() {}
        self.result = self.grid
            .iter()
            .filter(|(_,v)| v.visited)
            .count() as i32;
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()>{
        self.guard.placed_obstacles.insert(self.guard.position);

        while self.guard.advance(&mut self.grid, true, true).is_some() {}

        self.result = (self.guard.placed_obstacles.len() - 1) as i32; 
        Ok(())
    }


    fn get_solution(&self) -> String {
        self.result.to_string()
    }
}

