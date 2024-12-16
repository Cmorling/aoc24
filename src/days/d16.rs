use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

use glam::IVec2;

use crate::day::Day;

#[derive(Eq, PartialEq)]
struct Path {
    cost: u32,
    position: IVec2,
    direction: IVec2,
    visited: HashSet<IVec2>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Path {
    fn new(cost: u32, position: IVec2, direction: IVec2) -> Self {
        Path {
            cost,
            position,
            direction,
            visited: HashSet::new(),
        }
    } 

    fn set_visited(&mut self, hs: HashSet<IVec2>) {
        self.visited = hs;
    }
}

#[derive(Default)]

pub struct D16 {
    grid: HashMap<IVec2, bool>,
    start: IVec2,
    end: IVec2,
    result: u32,
}

impl D16 {
    fn get_directions(last: &IVec2) -> Vec<IVec2> {
        let directions = [IVec2::X,
            IVec2::Y,
            IVec2::NEG_X,
            IVec2::NEG_Y];
        directions
            .iter()
            .filter(|d| *d != &(-last)).copied()
            .collect()
    }

    fn dijkstra(&self) -> Option<u32> {
        let mut paths = BinaryHeap::new();
        let mut cache: HashMap<(IVec2, IVec2), u32> = HashMap::new();

        paths.push(Path::new(0, self.start, IVec2::X));

        while let Some(current_path) = paths.pop() {
            if current_path.position == self.end {
                return Some(current_path.cost);
            }

            let available_directions = D16::get_directions(&current_path.direction);

            for d in available_directions.iter() {
                let check_pos = current_path.position + d;
                if let Some(is_wall) = self.grid.get(&check_pos) {
                    if *is_wall {
                        continue;
                    }

                    let mut cost = current_path.cost + 1;
                    if *d != current_path.direction {
                        cost += 1000;
                    }

                    if let Some(cached_cost) = cache.get(&(check_pos, *d)) {
                        if *cached_cost <= cost {
                            continue;
                        }
                    }

                    cache.insert((check_pos, *d), cost);
                    paths.push(Path::new(cost, check_pos, *d));
                }
            }
        }
        None
    }

    fn dijkstra_visited(&self) -> Vec<HashSet<IVec2>> {
        let mut paths = BinaryHeap::new();
        let mut cache: HashMap<(IVec2, IVec2), u32> = HashMap::new();
        let mut cost_end: Option<u32> = None;
        let mut wins = Vec::new();

        let mut start_path = Path::new(0, self.start, IVec2::X);
        let mut start_hs = HashSet::new();

        start_hs.insert(self.start);
        start_path.set_visited(start_hs);

        paths.push(start_path);

        while let Some(current_path) = paths.pop() {

            if let Some(lowest_cost) = cost_end {
                if lowest_cost < current_path.cost {
                        return wins;
                }
            }

            if current_path.position == self.end {
                cost_end = Some(current_path.cost);
                wins.push(current_path.visited);    
                continue;
            }

            let available_directions = D16::get_directions(&current_path.direction);

            for d in available_directions.iter() {
                let check_pos = current_path.position + d;
                if let Some(is_wall) = self.grid.get(&check_pos) {
                    if *is_wall {
                        continue;
                    }

                    let mut cost = current_path.cost + 1;
                    if *d != current_path.direction {
                        cost += 1000;
                    }

                    if let Some(cached_cost) = cache.get(&(check_pos, *d)) {
                        if *cached_cost < cost {
                            continue;
                        }
                    }
                    let mut new_path = Path::new(cost, check_pos, *d);
                    let mut hs = current_path.visited.clone();
                    hs.insert(check_pos);
                    new_path.set_visited(hs); 

                    cache.insert((new_path.position, new_path.direction), new_path.cost);
                    paths.push(new_path);
                }
            }
        }
        wins
    }
}

impl Day for D16 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut reader_lines = reader.lines().enumerate();
       
        while let Some((y, Ok(line))) = reader_lines.next() {

            for (x, c) in line.chars().enumerate() {
                let pos = IVec2::new(x as i32, y as i32);
                let mut is_wall = false;

                match c {
                    '#' => is_wall = true,
                    'S' => self.start = pos, 
                    'E' => self.end = pos,
                    '.' => is_wall = false,
                    _ => break,
                }
                self.grid.insert(pos, is_wall);
            }
        }

        Ok(())
    }

    fn solve_part_one(&mut self) -> std::io::Result<()>{
        self.result = self.dijkstra().expect("Did no find path");
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        let all_visited = self.dijkstra_visited(); 
        self.result = all_visited
            .into_iter()
            .flat_map(|set| set.into_iter())
            .collect::<HashSet<IVec2>>()
            .len() as u32;
        
        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
