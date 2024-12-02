use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

use crate::day::Day;

fn _valid_check(v: Vec<i32>) -> bool {
    v
    .iter()
    .tuple_windows()
    .all(|(&n, &m)| !(n.abs_diff(m) > 3)) && 
    (
        v
        .iter()
        .tuple_windows()
        .all(|(&n, &m)| n > m) || 
        v
        .iter()
        .tuple_windows()
        .all(|(&n, &m)| m > n)
    )
}

#[derive(Default)]

pub struct D2p1 {
    reports: Vec<Vec<i32>>,
    result: i32,
}

impl Day for D2p1 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let v: Vec<i32> = line
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            self.reports.push(v);
        }

        Ok(())
    }

    fn solve(&mut self) ->  std::io::Result<()>{
        self.result = self.reports.iter()
            .filter(|v| {
                let orig = v.to_vec();
                _valid_check(orig)
            }).count() as i32;
        Ok(())
    }

    fn get_solution(&mut self) -> String {
        self.result.to_string()
    }
}

#[derive(Default)]

pub struct D2p2 {
    reports: Vec<Vec<i32>>,
    result: i32,
}

impl Day for D2p2 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let v: Vec<i32> = line
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            self.reports.push(v);
        }

        Ok(())
    }
    
    fn solve(&mut self) ->  std::io::Result<()>{
        self.result = self.reports
            .iter()
            .filter(|v| {
                let mut res = false;

                let orig = v.to_vec();
                res |= _valid_check(orig);

                (0..v.len())
                    .for_each(|i| {
                        let mut copy = v.to_vec();
                        copy.remove(i);
                        res |= _valid_check(copy);
                    });
               res
            })
            .count() as i32;
        Ok(())
    }

    fn get_solution(&mut self) -> String {
        self.result.to_string()
    }
}
