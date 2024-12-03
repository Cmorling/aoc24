use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;
use crate::day::Day;

#[derive(Default)]

pub struct D3p1 {
    inp: String,
    result: i32,
}

impl Day for D3p1 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.inp = reader
            .lines()
            .collect::<Result<Vec<String>, _>>()?
            .join("");

        Ok(())
    }

    fn solve(&mut self) -> std::io::Result<()>{
        let re = Regex::new(r"mul\((?<n>\d+),(?<m>\d+)\)").unwrap();

        self.result = re
            .captures_iter(&self.inp)
            .map(|caps| {
                caps.name("n").unwrap().as_str().parse::<i32>().unwrap() * 
                caps.name("m").unwrap().as_str().parse::<i32>().unwrap()
            })
            .sum();

        Ok(())
    }

    fn get_solution(self) -> String {
        self.result.to_string()
    }
}

#[derive(Default)]

pub struct D3p2 {
    inp: String,
    result: i32,
}

impl Day for D3p2 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.inp = reader
            .lines()
            .collect::<Result<Vec<String>, _>>()?
            .join("");

        Ok(())
    }
    
    fn solve(&mut self) -> std::io::Result<()>{
        let re = Regex::new(r"(?<mul>mul\((?<n>\d+),(?<m>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
        let mut is_doing = true;

        self.result = re
            .captures_iter(&self.inp)
            .map(|caps| {
                if caps.name("mul").is_some() && is_doing {
                    return caps.name("n").unwrap().as_str().parse::<i32>().unwrap() * 
                    caps.name("m").unwrap().as_str().parse::<i32>().unwrap();

                } else if caps.name("do").is_some() {
                    is_doing = true;
                } else if caps.name("dont").is_some() {
                    is_doing = false;
                }             
                0
            })
            .sum();
        Ok(())
    }

    fn get_solution(self) -> String {
        self.result.to_string()
    }
}
