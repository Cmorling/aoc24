use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::day::Day;

#[derive(Default)]

pub struct D1p1 {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
    result: i32
}

impl Day for D1p1 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let v: Vec<&str> = line.split("   ").collect();
            self.vec1.push(v[0].parse::<i32>().unwrap());
            self.vec2.push(v[1].parse::<i32>().unwrap());
        }

        Ok(())
    }

    fn solve(&mut self) ->  std::io::Result<()>{
        self.vec1.sort();
        self.vec2.sort();
        self.result = 0;

        for (ind, n) in self.vec1.iter().enumerate() {
            self.result += (n - self.vec2[ind]).abs();
        }

        Ok(())
    }

    fn get_solution(self) -> String {
        self.result.to_string()
    }
} 
#[derive(Default)]

pub struct D1p2 {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
    result: i32 
}

impl Day for D1p2 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let v: Vec<&str> = line.split("   ").collect();
            self.vec1.push(v[0].parse::<i32>().unwrap());
            self.vec2.push(v[1].parse::<i32>().unwrap());
        }

        Ok(())
    }

    fn solve(&mut self) -> std::io::Result<()> {
        self.result = 0;
        for n in self.vec1.iter() {
            let count = self.vec2.iter().filter(|&i| i == n).count();
            self.result += (count as i32) * n;
        }

        Ok(())
    }

    fn get_solution(self) -> String {
        self.result.to_string()
    }
}
