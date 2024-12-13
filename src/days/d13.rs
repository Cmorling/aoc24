use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::Sub;

use glam::{DVec2, DMat2};
use regex::Regex;
use rug::ops::AddFrom;
use rug::Float;

use crate::day::Day;

const RUG_PRECISION: u32 = 49;

pub struct D13 {
    result: Box<Float>,
    precision: Box<Float>,
    machines: Vec<(DMat2, DVec2)>,
}

impl D13 {
    fn solve(&mut self, add: f64) {
         for (system, target) in self.machines.iter() {
            let target_x = Float::with_val(RUG_PRECISION, add + target.x);
            let target_y = Float::with_val(RUG_PRECISION, add + target.y);

            let det = system.determinant();

            let inv_det = Float::with_val(RUG_PRECISION, 1) / det;

            let rug_system = [[Float::with_val(RUG_PRECISION, system.x_axis.x), Float::with_val(RUG_PRECISION, system.x_axis.y)],
                [Float::with_val(RUG_PRECISION, system.y_axis.x), Float::with_val(RUG_PRECISION, system.y_axis.y)]];

            let inv_rug_system = [
                [rug_system[1][1].clone() * &inv_det, -rug_system[0][1].clone() * &inv_det],
                [-rug_system[1][0].clone() * &inv_det, rug_system[0][0].clone() * &inv_det],
            ];

            let answer_x = inv_rug_system[0][0].clone() * target_x.clone() + inv_rug_system[1][0].clone() * target_y.clone();
            let answer_y = inv_rug_system[0][1].clone() * target_x + inv_rug_system[1][1].clone() * target_y;

            let res_x = answer_x.clone().round();
            let res_y = answer_y.clone().round();

            let prec_x = res_x.clone().sub(answer_x).abs();
            let prec_y = res_y.clone().sub(answer_y).abs();

            if prec_x > *self.precision || prec_y > *self.precision {
                continue;
            }

            let ptr_res = self.result.as_mut();
            ptr_res.add_from(res_y);
            ptr_res.add_from(res_x * Float::with_val(RUG_PRECISION, 3));
        }

    }
}

impl Default for D13 {
    fn default() -> Self {
        Self {
            result: Box::new(Float::new(RUG_PRECISION)),
            precision: Box::new(Float::with_val(RUG_PRECISION, 0.001)),
            machines: Vec::new(),
        }
    }
}

impl Day for D13 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let re_a = Regex::new(r"^Button A: X\+(?<x>\d+), Y\+(?<y>\d+)$").unwrap();
        let re_b = Regex::new(r"^Button B: X\+(?<x>\d+), Y\+(?<y>\d+)$").unwrap();
        let re_prize = Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();
        
        let mut a_vec = DVec2::new(0.0, 0.0);
        let mut b_vec = DVec2::new(0.0, 0.0);
        for line in reader.lines() {
            let current_line = line.expect("Error fetching line");

            if let Some(caps) = re_a.captures(&current_line) {
                let x_f = &caps["x"].parse::<f64>().expect("Could not convert to float");
                let y_f = &caps["y"].parse::<f64>().expect("Could not convert to float");
                a_vec = DVec2::new(*x_f, *y_f);

            } else if let Some(caps) = re_b.captures(&current_line) {
                let x_f = &caps["x"].parse::<f64>().expect("Could not convert to float");
                let y_f = &caps["y"].parse::<f64>().expect("Could not convert to float");
                b_vec = DVec2::new(*x_f, *y_f);

            } else if let Some(caps) = re_prize.captures(&current_line) {
                let x_f = &caps["x"].parse::<f64>().expect("Could not convert to float");
                let y_f = &caps["y"].parse::<f64>().expect("Could not convert to float");
                let target = DVec2::new(*x_f, *y_f);
                
                self.machines.push((DMat2::from_cols(a_vec, b_vec), target));
            }
        }

        Ok(())
    }

    fn solve_part_one(&mut self) ->  std::io::Result<()>{
        self.solve(0.0);
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        self.solve(1e13);
        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
