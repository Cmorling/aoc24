use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

use glam::IVec2;

use crate::day::Day;

struct Cell {
    wall: bool,
    obstacle: bool,
}

#[derive(Clone, Copy, Debug)]

struct Cell2 {
    wall: bool,
    obstacle: u8,
}

#[derive(Default)]

pub struct D15 {
    grid: HashMap<IVec2, Cell>,
    grid2: HashMap<IVec2, Cell2>,
    moves: Vec<IVec2>,
    robot: IVec2,
    dimensions: IVec2,
    result: i32,
}

impl D15 {
    fn display(&self) {
        for y in 0..(self.dimensions.y) {
            for x in 0..(self.dimensions.x) {
                let pos = IVec2::new(x,y);

                if pos == self.robot {
                    print!("@");
                    continue;
                }

                if let Some(c) = self.grid.get(&pos) {
                    if c.obstacle {
                        print!("O");
                    } else if c.wall {

                        print!("#");
                    } else {
                        print!(".");
                    }
                }             
            }
            println!();
        }
    }

    fn display2(&self) {
        for y in 0..(self.dimensions.y) {
            for x in 0..(self.dimensions.x * 2) {
                let pos = IVec2::new(x,y);

                if pos == self.robot {
                    print!("@");
                    continue;
                }

                if let Some(c) = self.grid2.get(&pos) {
                    if c.obstacle == 1 {
                        print!("[");
                    } else if c.obstacle == 2 {
                        print!("]");
                    } else if c.wall {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }             
            }
            println!();
        }
    }

    fn move_obstacle (&mut self, pos: &IVec2, direction: &IVec2) -> bool {
        let next_sq = pos + direction;
        let cell = self.grid.get_mut(&next_sq).expect("Outside grid");

        if cell.wall {
            false
        } else if cell.obstacle {
            return self.move_obstacle(&next_sq, direction);
        } else {
            cell.obstacle = true;
            return true;
        }
    }

    fn walk(&mut self, direction: &IVec2) {
        let next_sq = self.robot + direction;
        let cell = self.grid.get(&next_sq).expect("Outside grid");
        if cell.wall {
            return;
        } else if cell.obstacle {
            if self.move_obstacle(&next_sq, direction) {
                let mut_cell = self.grid.get_mut(&next_sq).expect("Outside grid");
                mut_cell.obstacle = false; 
            } else {
                return;
            }
        } 
        self.robot = next_sq;
    }

    fn move_obstacle2_x (&mut self, pos: &IVec2, direction: &IVec2) -> bool {
        let next_sq = pos + direction;
        let current_cell = *self.grid2.get(pos).expect("Outside grid");

        let cell = *self.grid2.get(&next_sq).expect("Outside grid");
        
        let cell_mut;
        if cell.wall {
            false
        } else if cell.obstacle > 0 {
            let res = self.move_obstacle2_x(&next_sq, direction);
            if res {
                cell_mut = self.grid2.get_mut(&next_sq).expect("Outside grid");
                cell_mut.obstacle = current_cell.obstacle;
                true
            } else {
                false
            }
        } else {
            cell_mut = self.grid2.get_mut(&next_sq).expect("Outside grid");
            cell_mut.obstacle = current_cell.obstacle;
            return true;
        }
    }

    fn check_obstacle2_y (&mut self, pos: &IVec2, direction: &IVec2) -> bool {
        let next_sq = pos + direction;
        let cell = *self.grid2.get(&next_sq).expect("Outside grid");

        let current_cell = *self.grid2.get(pos).expect("Outside grid");
        let friend_location = 
            if current_cell.obstacle == 1 {
                pos + IVec2::X
            } else {
                pos + IVec2::NEG_X
            };

        let friend_next_sq = friend_location + direction;
        let cell_friend = *self.grid2.get(&friend_next_sq).expect("Outside grid");
        let checkers = [(cell, next_sq), (cell_friend, friend_next_sq)];

        checkers
            .iter()
            .all(|(check, loc)| {
                if check.wall {
                    false
                } else if check.obstacle > 0 {
                    self.check_obstacle2_y(loc, direction)
                } else {
                    return true;
                }
            })
    }

    fn move_obstacle2_y (&mut self, pos: &IVec2, direction: &IVec2) {
        let next_sq = pos + direction;
        let cell = *self.grid2.get(&next_sq).expect("Outside grid");

        let current_cell = *self.grid2.get(pos).expect("Outside grid");
        let current_cell_friend; 
        let friend_location;

        if current_cell.obstacle == 1 {
            friend_location = pos + IVec2::X; 
            current_cell_friend = *self.grid2.get(&friend_location).expect("Outside grid");
        } else {
            friend_location = pos + IVec2::NEG_X; 
            current_cell_friend = *self.grid2.get(&friend_location).expect("Outside grid");
        }

        let friend_next_sq = friend_location + direction;
        let cell_friend = *self.grid2.get(&friend_next_sq).expect("Outside grid");
        
        if current_cell.obstacle == cell.obstacle {
            self.move_obstacle2_y(&next_sq, direction);
        } else {
            let checkers = [(cell, next_sq), (cell_friend, friend_next_sq)];
            checkers
                .iter()
                .for_each(|(check, loc)| {
                    if check.obstacle > 0 {
                        self.move_obstacle2_y(loc, direction);
                    } 
                });
        }

        let cell_mut= self.grid2.get_mut(&next_sq).expect("Outside grid");
        cell_mut.obstacle = current_cell.obstacle;
        let cell_friend_mut= self.grid2.get_mut(&friend_next_sq).expect("Outside grid");
        cell_friend_mut.obstacle = current_cell_friend.obstacle;
        
        let current_cell_mut = self.grid2.get_mut(pos).expect("Outside grid");
        current_cell_mut.obstacle = 0;
        let current_cell_friend_mut = self.grid2.get_mut(&friend_location).expect("Outside grid");
        current_cell_friend_mut.obstacle = 0;
    }

    fn walk2(&mut self, direction: &IVec2) {
        let next_sq = self.robot + direction;
        let cell = self.grid2.get(&next_sq).expect("Outside grid");
        if cell.wall {
            return;
        } else if cell.obstacle > 0 {
            if direction == &IVec2::X || direction == &IVec2::NEG_X {
                if self.move_obstacle2_x(&next_sq, direction) {
                    let mut_cell = self.grid2.get_mut(&next_sq).expect("Outside grid");
                    mut_cell.obstacle = 0; 
                } else {
                    return;
                }
            } else if self.check_obstacle2_y(&next_sq, direction) {
                self.move_obstacle2_y(&next_sq, direction);
            } else {
                return;
            }
        } 
        self.robot = next_sq;
    }

    fn parse_part2(&mut self) {
        for y in 0..(self.dimensions.y) {
            for x in 0..(self.dimensions.x) {
                let pos = IVec2::new(x,y);
                let new_pos = IVec2::new(x*2, y);
                let new_pos_2 = new_pos + IVec2::X;

                if let Some(c) = self.grid.get(&pos) {
                    if c.obstacle {
                        self.grid2.insert(new_pos, Cell2 {wall: false, obstacle: 1});
                        self.grid2.insert(new_pos_2, Cell2 {wall: false, obstacle: 2});
                    } else if c.wall {
                        self.grid2.insert(new_pos, Cell2 {wall: true, obstacle: 0});
                        self.grid2.insert(new_pos_2, Cell2 {wall: true, obstacle: 0});
                    } else {
                        self.grid2.insert(new_pos, Cell2 {wall: false, obstacle: 0}); 
                        self.grid2.insert(new_pos_2, Cell2 {wall: false, obstacle: 0});
                    }
                }             
            }
        }
        self.robot = IVec2::new(self.robot.x * 2, self.robot.y);
    }

}

impl Day for D15 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut reader_lines = reader.lines().enumerate();
       
        let mut max_y = 0;
        let mut max_x = 0;

        while let Some((y, Ok(line))) = reader_lines.next() {
            if line.is_empty() {
                break;
            }
            max_x = 0;
            for (x, c) in line.chars().enumerate() {
                let pos = IVec2::new(x as i32, y as i32);
                let mut cell_insert = Cell {wall: false, obstacle: false};

                match c {
                    '#' => cell_insert.wall = true,
                    'O' => cell_insert.obstacle = true,
                    '@' => self.robot = pos, 
                    '.' => {},
                    _ => break,
                }
                self.grid.insert(pos, cell_insert);
                max_x += 1
            }
            max_y += 1;
        }

        self.dimensions = IVec2::new(max_x, max_y);

        while let Some((_, Ok(line))) = reader_lines.next() {
            for dir in line.chars() {
                if dir == '\n' {
                    break;
                }
                let add_vector = match dir {
                        '>' => IVec2::X,
                        '<' => IVec2::NEG_X,
                        '^' => IVec2::NEG_Y,
                        'v' => IVec2::Y,
                        _ => IVec2::new(0,0),
                };
                self.moves.push(add_vector);
            }
        }

        Ok(())
    }

    fn solve_part_one(&mut self) -> std::io::Result<()>{
        let moves_cpy = self.moves.clone();

        for dir in moves_cpy {
            self.walk(&dir);
            // self.display();
        }

        for y in 0..(self.dimensions.y) {
            for x in 0..(self.dimensions.x) {
                let pos = IVec2::new(x,y);

                if let Some(c) = self.grid.get(&pos) {
                    if c.obstacle {
                        self.result += (100 * y) + (x);
                    } 
                }             
            }
        }
        // self.display();
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        self.parse_part2();
        // self.display2();

        let moves_cpy = self.moves.clone();

        for dir in moves_cpy {
            self.walk2(&dir);
            // self.display2();
        }

        for y in 0..(self.dimensions.y) {
            for x in 0..(self.dimensions.x * 2) {
                let pos = IVec2::new(x,y);

                if let Some(c) = self.grid2.get(&pos) {
                    if c.obstacle == 1 {
                        self.result += (100 * y) + (x);
                    } 
                }             
            }
        }
        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
