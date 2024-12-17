use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;

use crate::day::Day;

#[derive(Default)]

struct Cscvm {
    a: u64,
    b: u64,
    c: u64,
    out: Vec<u64>,
}

impl Cscvm {
    fn get_cmb(&self, combo: &u64) -> u64 {
        if *combo == 7 {
           println!("Invalid program");
        }
        match combo {
            0..4 => *combo,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0,
        }
    }

    fn adv(&mut self, combo: &u64) {
        self.a >>= self.get_cmb(combo);
    }

    fn bxl(&mut self, literal: &u64) {
        self.b ^= literal;
    }

    fn bst(&mut self, combo: &u64) {
        self.b = self.get_cmb(combo) % 8;
    }

    fn jnz(&mut self, literal: &u64) -> Option<u64> {
        if self.a == 0 {
            return None;
        }
        Some(*literal)
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
    }

    fn out(&mut self, combo: &u64) {
        self.out.push(self.get_cmb(combo) % 8);
    }

    fn bdv(&mut self, combo: &u64) {
        self.b = self.a >> self.get_cmb(combo);
    }

    fn cdv(&mut self, combo: &u64) {
        self.c = self.a >> self.get_cmb(combo);
    }

}

#[derive(Default)]

pub struct D17 {
    cscvm: Cscvm,
    instructions: Vec<u64>,
    result: String,
}

impl D17 {
    fn subset_rev(lhs: &[u64], rhs :&[u64], depth: usize) -> bool {
        if lhs.len() != depth {
            return false;
        }
        let mut cl_lhs = lhs.to_owned();
        let mut cl_rhs = rhs.to_owned();
        cl_lhs.reverse();
        cl_rhs.reverse();

        let max_len = lhs.len();

        for i in 0..max_len {
            if cl_lhs[i] != cl_rhs[i] {
                return false;
            }
        }
        true
    }
    fn run_csvm(&mut self) {
        let mut ip = 0;
        while ip < self.instructions.len() {
            let op = self.instructions[ip];
            let arg = self.instructions[ip + 1];
       
            match op {
                0 => self.cscvm.adv(&arg),
                1 => self.cscvm.bxl(&arg),
                2 => self.cscvm.bst(&arg),
                3 => {
                    if let Some(new_ip) = self.cscvm.jnz(&arg){
                        ip = new_ip as usize;
                        continue;
                    }
                },
                4 => self.cscvm.bxc(),
                5 => self.cscvm.out(&arg),
                6 => self.cscvm.bdv(&arg),
                7 => self.cscvm.cdv(&arg),
                other => println!("Invalid instruction: {}", other),
            }
            ip += 2;
        }
    }

    fn walk(&mut self, init_a: u64, depth: usize) -> Option<u64> {
        let s_b = self.cscvm.b;
        let s_c = self.cscvm.c;
        
        let mut should_walk = false;
        let mut guess_a;

        for guess in 0..8 {
            guess_a = (init_a << 3) + guess;
            self.cscvm.a = guess_a;

            self.run_csvm();

            if D17::subset_rev(&self.cscvm.out, &self.instructions, depth) {
                if self.cscvm.out.len() == self.instructions.len() {
                    return Some(guess_a);
                }
                should_walk = true;
            }

            self.cscvm.out = Vec::new();
            self.cscvm.b = s_b;
            self.cscvm.c = s_c;

            if should_walk {
                if let Some(answer) = self.walk(guess_a, depth + 1) {
                    return Some(answer);
                }
                should_walk = false;
            }
        } 
        None
    }
}

impl Day for D17 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut reader_lines = reader.lines();
        
        let re = Regex::new(r"\d+").expect("error creating regex");

        if let Some(Ok(line)) = reader_lines.next() {
            if let Some(cap) = re.captures(&line) {
                self.cscvm.a = cap[0].parse::<u64>().expect("Could not parse");
            }
        }

        if let Some(Ok(line)) = reader_lines.next() {
            if let Some(cap) = re.captures(&line) {
                self.cscvm.b = cap[0].parse::<u64>().expect("Could not parse");
            }
        }

        if let Some(Ok(line)) = reader_lines.next() {
            if let Some(cap) = re.captures(&line) {
                self.cscvm.c = cap[0].parse::<u64>().expect("Could not parse");
            }
        }

        reader_lines.next();

        if let Some(Ok(line)) = reader_lines.next() {
            self.instructions = re
                .captures_iter(&line)
                .fold(Vec::new(), |mut inst, cap| {
                    inst.push(cap[0].parse::<u64>().expect("Could not parse"));
                    inst
                });
        }
        self.cscvm.out = Vec::new();
        Ok(())
    }

    fn solve_part_one(&mut self) -> std::io::Result<()>{
        self.run_csvm();
        self.result = self.cscvm.out
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",");

        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        self.result = self.walk(0, 1).expect("Did not find inital value").to_string();
        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.clone()
    }
} 
