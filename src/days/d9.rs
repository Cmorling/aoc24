use std::fs::File;
use std::io::{BufReader, BufRead};


use crate::day::Day;


#[derive(Default)]

pub struct D9 {
    disk: Vec<Option<usize>>,
    fragment_count: usize,
    result: u64,
    max_id: usize
}

impl D9 {
    fn get_next_none(vec_a: &[Option<usize>]) -> Option<usize> {
        for (ind, opt) in vec_a.iter().enumerate() {
            if opt.is_none() {
                return Some(ind);
            }
        }
        None
    }

    fn chksum(vec_a: &[Option<usize>]) -> u64 {
        vec_a
            .iter()
            .enumerate()
            .map(|(ind, value)| {
                match value {
                    Some(val) => val * ind,
                    None => 0,
                }
            })
            .sum::<usize>() as u64
    }

    fn get_next_none_group(vec_a: &[Option<usize>], size: &usize) -> Option<usize> {
        let mut check_count = 0;
        for (ind, opt) in vec_a.iter().enumerate() {
            if check_count == *size {
                return Some(ind - check_count);
            }

            match opt {
                Some(_) => check_count = 0,
                None => check_count += 1,
            }
        }
        None
    }

    fn get_next_some_group(vec_a: &[Option<usize>], search_param: &usize) -> Option<(usize, usize)> {
        let mut seen = 0;
        for (ind, opt) in vec_a.iter().enumerate() {
           if let Some(val) = opt {
               if val == search_param {
                   seen += 1;
                   continue;
               }            
           } 
           if seen > 0 {
                return Some((ind - seen, seen));
           }
        }

        if seen > 0 {
            return Some((vec_a.len() - seen, seen));
        }

        None
    }
}

impl Day for D9 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut disk = String::new();
        reader.read_line(&mut disk)?;
        
        let mut is_none = false;
        (self.disk, self.fragment_count, self.max_id)= disk
            .chars()
            .fold((Vec::new(), 0, 0), |(mut d, mut fc, mut disk_id), c| {
                let current_adder;

                match is_none {
                    true => current_adder = None,
                    false => {
                        current_adder = Some(disk_id);
                        disk_id += 1;
                        fc += c.to_digit(10).expect("Could not convert character to digit") as usize;
                    },
                } 
                if c != '\n' {
                    for _ in 0..(c.to_digit(10).expect("Could not convert character to digit")) {
                        d.push(current_adder);
                    }
                } 
                
                is_none = !is_none;
                (d, fc, disk_id)
            });
        Ok(())
    }

    fn solve_part_one(&mut self) ->  std::io::Result<()>{
        let mut fixed = self.disk.clone();
        let fixed_len = fixed.len();

        let mut next_none = D9::get_next_none(&fixed).expect("None not found");
        
        for (ind, to_be_moved) in self.disk.iter().rev().enumerate() {
            match to_be_moved {
                Some(val) => {
                    fixed[next_none] = Some(*val);
                    fixed[fixed_len - ind - 1] = None;
                    next_none = next_none + D9::get_next_none(&fixed[next_none..]).expect("None not found");
                },
                None => continue
            } 

            if fixed_len - ind - 1 == self.fragment_count {
                break
            } 
        }

        self.result = D9::chksum(&fixed);
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        let mut fixed = self.disk.clone();
        let fixed_len = fixed.len();

        if fixed[fixed_len - 1].is_some() {
            self.max_id -= 1;
        }
        for i in (0..(self.max_id + 1)).rev() {
            let (some_idx, some_len) = D9::get_next_some_group(&fixed, &i).expect("Could not find some group");
            let none_idx_opt = D9::get_next_none_group(&fixed, &some_len);
            

            let none_idx = match none_idx_opt {
                Some(idx) => idx,
                None => continue,
            };

            if none_idx > some_idx {
                continue;
            }

            for j in 0..(some_len) {
                fixed[none_idx + j] = Some(i);
                fixed[some_idx + j] = None;
            }

        }
        self.result = D9::chksum(&fixed);

        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }

    fn new() -> Self {
        Default::default()
    }
} 
