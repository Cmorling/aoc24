use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

use glam::IVec2;
use itertools::Itertools;

use crate::day::Day;

#[derive(PartialEq)]

enum CoordOccupation {
    Empty,
    Antinode,
}

#[derive(Default)]

pub struct D8 {
    grid: HashMap<IVec2, CoordOccupation>,
    antenna_map: HashMap<char, Vec<IVec2>>,
    result: i32
}

impl D8 {
    fn place_antennas(vec_a: &IVec2, vec_b: &IVec2) -> Vec<IVec2> {
        let diff = vec_a - vec_b;
        vec![vec_a + diff, vec_b - diff]
    }

    fn place_antennas_part_two(&self, vec_a: &IVec2, vec_b: &IVec2) -> Vec<IVec2> {
        let diff = vec_a - vec_b;
        let mut res = Vec::new();
        let mut current_vec_a = *vec_a;
        let mut current_vec_b = *vec_b;

        while self.grid.contains_key(&(current_vec_a + diff)) {
            res.push(current_vec_a+diff);
            current_vec_a += diff;
        };

        while self.grid.contains_key(&(current_vec_b - diff)) {
            res.push(current_vec_b - diff);
            current_vec_b -= diff;
        };
        res.push(*vec_a);
        res.push(*vec_b);
        res
    }
}

impl Day for D8 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        (self.grid, self.antenna_map) = reader
            .lines()
            .enumerate()
            .fold(
                (HashMap::new(), HashMap::new()),
                |(mut grid, mut antenna_map), (y, line)| {
                    line
                        .unwrap()
                        .chars()
                        .enumerate()
                        .for_each(|(x, c)| {
                            let current_cord = IVec2::new(x as i32, y as i32);
                            match c {
                                '.' => {
                                    grid.insert(current_cord, CoordOccupation::Empty);
                                },
                                ant => {
                                    grid.insert(current_cord, CoordOccupation::Empty);
                                    // grid.insert(current_cord.clone(), CoordOccupation::Antenna);
                                    antenna_map.entry(ant).or_insert(Vec::new()).push(current_cord);
                                },
                            };
                        });
                    (grid, antenna_map)
                });

        Ok(())
    }

    fn solve_part_one(&mut self) ->  std::io::Result<()>{
        self.antenna_map
            .iter()
            .for_each(|(_, v)| {
                let pairs = v.iter().combinations(2);

                let hash_placed = pairs
                    .fold(Vec::new(), |mut placed, p| {
                        let want_to_place = D8::place_antennas(p[0], p[1]);
                        placed.push(want_to_place[0]);
                        placed.push(want_to_place[1]);
                        placed
                    });

                hash_placed
                    .iter()
                    .for_each(|location| {

                        if let Some(coord) = self.grid.get_mut(location) {
                            if coord == &CoordOccupation::Empty {
                                *coord = CoordOccupation::Antinode;
                            }
                        }
                    })
            });
        self.result = self.grid.values().map(|v| {
                if let CoordOccupation::Antinode = v {
                    1
                } else {
                   0 
                }
            })
            .sum::<u32>() as i32;

        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        let antenna_map_clone = self.antenna_map.clone();
        antenna_map_clone
            .iter()
            .for_each(|(_, v)| {
                let pairs = v.iter().combinations(2);

                let hash_placed = pairs
                    .fold(Vec::new(), |mut placed, p| {
                        let want_to_place = self.place_antennas_part_two(p[0], p[1]);
                        want_to_place
                            .iter()
                            .for_each(|pos| placed.push(*pos));

                        placed
                    });

                hash_placed
                    .iter()
                    .for_each(|location| {
                        if let Some(coord) = self.grid.get_mut(location) {
                            if coord == &CoordOccupation::Empty {
                                *coord = CoordOccupation::Antinode;
                            }
                        }
                    })
            });
        self.result = self.grid.values().map(|v| {
                if let CoordOccupation::Antinode = v {
                    1
                } else {
                   0 
                }
            })
            .sum::<u32>() as i32;


        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
