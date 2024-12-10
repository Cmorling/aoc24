use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

use glam::IVec2;

use crate::day::Day;

#[derive(Default)]

pub struct D10 {
    zeroes: Vec<IVec2>,
    grid: HashMap<IVec2, (u32, Vec<IVec2>)>,
    reached: HashSet<IVec2>,
    result: i32
}

impl D10 {
    fn dfs(&mut self, start: &IVec2, is_part_one: bool) -> u32 {
        let (current_val, search_directions) = self.grid.get(start).expect("Could not find vector in hashmap").clone();

        if current_val == 9 {
            if is_part_one {
                if !self.reached.contains(start) {
                    self.reached.insert(*start);
                    return 1;
                } else {
                    return 0;
                }
            } else {
                return 1;
            }
        }

        let mut ret = 0;
        for direction in search_directions {
            match self.grid.get(&direction) {
                Some((check_val, _)) => {
                    if current_val + 1 == *check_val {
                        ret += self.dfs(&direction, is_part_one); 
                    }
                },
                None => continue,
            }
        }
        ret
    }
}

impl Day for D10 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let unit_vectors = [IVec2::X,
            IVec2::Y,
            IVec2::NEG_X,
            IVec2::NEG_Y];

        (self.zeroes, self.grid) = reader
            .lines()
            .enumerate()
            .fold((Vec::new(), HashMap::new()), |(mut zeroes, mut grid), (y, row)| {
                for (x, cell) in row.expect("Row not found").chars().enumerate() {
                    if cell == '\n' {
                        continue;
                    }
                    let current_digit = cell.to_digit(10).expect("Could not convert char to digit");
                    let position = IVec2::new(x as i32, y as i32);
                    let mut insert_vec = Vec::new();
                    
                    for unit_vec in unit_vectors.iter() {
                        insert_vec.push(position + unit_vec);
                    }

                    grid.insert(position, (current_digit, insert_vec));

                    if current_digit == 0 {
                        zeroes.push(position);
                    }
                }

                (zeroes, grid)
            });


        Ok(())
    }

    fn solve_part_one(&mut self) ->  std::io::Result<()>{
        let zeroes = self.zeroes.clone();

        self.result = zeroes 
            .iter()
            .map(|pos| {
                self.reached = HashSet::new();
                self.dfs(pos, true)
            })
            .sum::<u32>() as i32;

        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        let zeroes = self.zeroes.clone();

        self.result = zeroes 
            .iter()
            .map(|pos| {
                self.dfs(pos, false)
            })
            .sum::<u32>() as i32;


        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
