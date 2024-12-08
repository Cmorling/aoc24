use std::fs::File;
use std::io::{BufReader, BufRead};

use itertools::Itertools;

use crate::day::Day;

#[derive(Debug)]
struct CalibrationLine {
    target: u64,
    comps: Vec<u64>
}

#[derive(Default)]

pub struct D7 {
    result: u64,
    targets: Vec<CalibrationLine>
}

impl D7 {
    fn concat(&self, v1: &u64, v2: &u64) -> Option<u64> {
       let v1_s = v1.to_string();
       let v2_s = v2.to_string();
       
       let res = format!("{v1_s}{v2_s}");

       match res.parse::<u64>() {
           Ok(r) => Some(r),
           Err(_) => None,
       }
    }
}

impl Day for D7 {

    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        self.targets = reader
            .lines()
            .fold(Vec::new(), |mut v, l| {
                let line = l.unwrap();
                let mut parts_iter = line.split(": ");
                let target = parts_iter.next().unwrap().parse::<u64>().unwrap(); 
                let comps = parts_iter.next().unwrap();
                let comps_collected: Vec<u64> = comps
                    .split(" ")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                v.push(CalibrationLine {target, comps: comps_collected});
                v
            });
        Ok(())
    }

    fn solve_part_one(&mut self) ->  std::io::Result<()>{
        let ops = [0, 1];
        
        self.result = self.targets
            .iter()
            .filter_map(|calib| {
                let repeated = vec![ops.iter().cloned(); calib.comps.len()];
                let ops_perm = repeated.into_iter().multi_cartesian_product();
                ops_perm
                    .fold(None, |mut acc, op| {
                        let res2 = calib.comps
                            .iter()
                            .enumerate()
                            .fold(0, |mut acc2:u64, (ind, value)| {
                                match op[ind] {
                                    0 => {
                                        match acc2.checked_mul(*value) {
                                            Some(prod) => acc2 = prod,
                                            None => acc2 += 0,
                                        }
                                    },
                                    1 => acc2 += value,
                                    _ => acc2 += 0,
                                }
                                acc2
                            });
                        if res2 == calib.target  {
                            acc = Some(calib.target);
                        }
                        acc
                    })
            })
            .sum();
        
        Ok(())
    }
    fn solve_part_two(&mut self) ->  std::io::Result<()>{
        let ops = [0, 1, 2];
        let mut ran = 0; 

        self.result = self.targets
            .iter()
            .filter_map(|calib| {
                let repeated = vec![ops.iter().cloned(); calib.comps.len()];
                let ops_perm = repeated.into_iter().multi_cartesian_product();
                ran += 1;
                ops_perm
                    .fold(None, |mut acc, op| {
                        let res2 = calib.comps
                            .iter()
                            .enumerate()
                            .fold(0, |mut acc2:u64, (ind, value)| {
                                match op[ind] {
                                    0 => {
                                        match acc2.checked_mul(*value) {
                                            Some(prod) => acc2 = prod,
                                            None => acc2 += 0,
                                        }
                                    },
                                    1 => acc2 += value,
                                    2 => {
                                        match self.concat(&acc2, value) {
                                            Some(r) => acc2 = r,
                                            None => acc2 += 0,
                                        }
                                    },
                                    _ => acc2 += 0,
                                }
                                acc2
                            });
                        if res2 == calib.target  {
                            acc = Some(calib.target);
                        }
                        acc
                    })
            })
            .sum();
        
        Ok(())
    }


    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 

