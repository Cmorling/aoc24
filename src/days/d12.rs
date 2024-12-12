use std::char;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

use crate::day::Day;

#[derive(Default)]

pub struct D12 {
    grid: HashMap<IVec2, (char, bool)>,
    directions: Vec<IVec2>,
    result: i32,
}

impl D12 {
    fn floodfill(&mut self, start: IVec2, is_part_two: bool) -> Option<(u64, u64)> {
        let (start_val, start_visited) = self.grid.get(&start).expect("Start not in bound");

        if *start_visited {
            return None
        }

        let mut area = 0;
        let mut perimeter = 0;

        let mut seen_cells = HashSet::new();
        let mut que = VecDeque::new();
        let mut edges: HashMap<(IVec2, i32), Vec<i32>> = HashMap::new();

        que.push_back((start, *start_val));
        seen_cells.insert(start);

        while let Some((current_cell, current_val)) = que.pop_front() {
            area += 1;
            self.grid.insert(current_cell, (current_val, true));

            for direction in self.directions.iter() {
                let check_vec = current_cell + direction;

                if seen_cells.contains(&check_vec) {
                    continue;
                }

                if let Some((check_val, _)) = self.grid.get(&check_vec) {
                    if current_val == *check_val {
                        seen_cells.insert(check_vec);
                        que.push_back((check_vec, *check_val));
                        continue;
                    }
                }
                perimeter += 1;

                if is_part_two {
                    let masked_coord;
                    let masked_value;

                    if direction.x == 0 {
                        masked_coord = current_cell.y;
                        masked_value = current_cell.x;
                    } else {
                        masked_coord = current_cell.x;
                        masked_value = current_cell.y;
                    }
                    edges.entry((*direction, masked_coord)).or_default().push(masked_value);
                }
            }

        }

        if is_part_two {
            perimeter = 0;

            for (_, same_cardinal_direction) in edges.iter() {
                let mut sorted_cardinal = same_cardinal_direction.clone();
                sorted_cardinal.sort();

                perimeter += 1;

                let mut previous_val = None;

                for val in sorted_cardinal {
                    if let Some(p_val) = previous_val {
                        if p_val + 1 != val {
                            perimeter += 1;
                        }
                    }
                    previous_val = Some(val)
                }
            }
        }

        Some((area, perimeter))
    }

    fn solve(&mut self, is_part_two: bool) {
        let grid_clone = self.grid.clone();
        self.result = grid_clone
            .keys()
            .map(|k| {
                if let Some((area, perimeter)) = self.floodfill(*k, is_part_two) {
                    area * perimeter
                } else {
                    0
                }
            })
            .sum::<u64>() as i32;
    }
}

impl Day for D12 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        self.directions = vec![
            IVec2::X,
            IVec2::Y,
            IVec2::NEG_X,
            IVec2::NEG_Y,

        ];

        self.grid = reader
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut grid, (y, row)| {
                for (x, cell) in row.expect("Row not found").chars().enumerate() {
                    if cell == '\n' {
                        continue;
                    }
                    
                    let position = IVec2::new(x as i32, y as i32);

                    grid.insert(position, (cell, false));

                }
                grid
            });

        Ok(())
    }

    fn solve_part_one(&mut self) ->  std::io::Result<()>{
        self.solve(false);
        Ok(())
    }
    fn solve_part_two(&mut self) -> std::io::Result<()> {
        self.solve(true);

        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
