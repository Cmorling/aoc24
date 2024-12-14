use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;
use std::collections::HashSet;

use glam::IVec2;
use regex::Regex;

use crate::day::Day;

const X: i32 = 101;
const Y: i32 = 103;
const STEPS: i32 = 100;

#[derive(Default)]

pub struct D14 {
    robots: Vec<(IVec2, IVec2)>,
    result: u32,
}

impl D14 {
    fn step(state: Vec<(IVec2, IVec2)>, steps: i32) -> Vec<(IVec2, IVec2)>{
        state 
            .iter()
            .map(|(start, v)| {
                let big_position = start + (v * steps);
                let mut final_position_x = big_position.x % X;
                let mut final_position_y = big_position.y % Y;

                if final_position_x < 0 {
                    final_position_x += X;
                }
                if final_position_y < 0 {
                    final_position_y += Y;
                }

                (IVec2::new(final_position_x, final_position_y), *v)
            })
            .collect::<Vec<(IVec2, IVec2)>>()
    }

    fn display(&self, state: &[(IVec2, IVec2)]) -> bool {

        let mut d = HashSet::with_capacity(state.len());
        for (pos, _) in state.iter() {
            d.insert(pos);
        }
        let directions = [
            IVec2::X,
            IVec2::Y,
            IVec2::NEG_X,
            IVec2::NEG_Y,
            IVec2::new(1,1),
            IVec2::new(-1,1),
            IVec2::new(-1,-1),
            IVec2::new(1,-1),
        ];

        let mut should_continue = false;
        for (pos, _) in state.iter() {
            should_continue |= directions.iter().all(|dir| d.contains(&(pos + dir)));
            if should_continue {
                break;
            }
        }

        if !should_continue {
            return false;
        }

        for y in 0..Y {
            for x in 0..X {
                if d.contains(&IVec2::new(x,y)) {
                    print!("x");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        true
    }
}

impl Day for D14 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut reader_lines = reader.lines();
        
        let re = Regex::new(r"^p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)$").expect("Could not parse regex");

        while let Some(Ok(line)) = reader_lines.next() {
            if let Some(caps) = re.captures(&line) {
                let px = caps["px"].parse::<i32>().expect("Parse error");
                let py = caps["py"].parse::<i32>().expect("Parse error");
                let vx = caps["vx"].parse::<i32>().expect("Parse error");
                let vy = caps["vy"].parse::<i32>().expect("Parse error");

                self.robots.push((
                    IVec2::new(px, py),
                    IVec2::new(vx, vy),
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&mut self) -> std::io::Result<()>{
        print!("Parsed: {:?}", self.robots);
        let final_positions = D14::step(self.robots.clone(), STEPS);

        let mut quadrants = vec![0; 4];
        let mid_x = (X-1) / 2;
        let mid_y = (Y-1) / 2;

        for (pos, _) in final_positions.iter() {
            let mut insert_pos;
            match mid_x.cmp(&pos.x) {
                Ordering::Less => insert_pos = 0,
                Ordering::Equal => continue,
                Ordering::Greater => insert_pos = 2,
            }

            match mid_y.cmp(&pos.y) {
                Ordering::Less => insert_pos += 0,
                Ordering::Equal => continue,
                Ordering::Greater => insert_pos += 1,
            }

            quadrants[insert_pos] += 1;
        }
        print!("Quadrants: {:?}", quadrants);
        self.result = quadrants
            .iter()
            .product::<u32>();
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        let mut next_position = self.robots.clone();

        for i in 0..10000 {
            next_position = D14::step(next_position.clone(), 1);
            let ret = self.display(&next_position);
            if ret {
                println!("Finished iteration {} (state above this)", i + 1);
            }
        } 

        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
