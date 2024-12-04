use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::day::Day;
use crate::util::{two_d_window, matrix_transpose, matrix_count_row_match};

#[derive(Default)]

pub struct D4p1 {
    matrix: Vec<Vec<char>>,
    result: i32,
}

impl Day for D4p1 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.matrix = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();

        Ok(())
    }

    fn solve(&mut self) -> std::io::Result<()>{
        
        let windows = two_d_window(&self.matrix, (4, 4));
        let dim_y = self.matrix.len();
        let dim_x = self.matrix[0].len();

        self.result = windows
            .iter()
            .enumerate()
            .map(|(i, window)| {
                let is_x_edge = (i + 1) % (dim_x - 4 + 1) == 0;
                let is_y_edge = i > (windows.len() - (dim_y - 4 - 1));

                let mut matches = 0;
                let pattern = vec!['X', 'M', 'A', 'S'];
                let mut b_pattern = pattern.to_vec(); 
                b_pattern.reverse();

                let mut tests = Vec::new();

                //horizontal
                tests.push(window[0].clone());
                if is_y_edge {
                    tests.push(window[1].clone());
                    tests.push(window[2].clone());
                    tests.push(window[3].clone());
                }
                // vertical
                let t_window = matrix_transpose(&window);
                tests.push(t_window[0].clone());
                if is_x_edge {
                    tests.push(t_window[1].clone());
                    tests.push(t_window[2].clone());
                    tests.push(t_window[3].clone());
                }

                // diagonal
                let mut diag1 = Vec::new();
                let mut diag2 = Vec::new();

                diag1.push(window[0][0]);
                diag1.push(window[1][1]);
                diag1.push(window[2][2]);
                diag1.push(window[3][3]);
        
                diag2.push(window[3][0]);
                diag2.push(window[2][1]);
                diag2.push(window[1][2]);
                diag2.push(window[0][3]);
               
                tests.push(diag1.clone());
                tests.push(diag2.clone());
                matches += matrix_count_row_match(&tests, &pattern);
                matches += matrix_count_row_match(&tests, &b_pattern);
        
                matches as i32
            })
            .sum();

        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
}

#[derive(Default)]

pub struct D4p2 {
    matrix: Vec<Vec<char>>,
    result: i32,
}

impl Day for D4p2 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.matrix = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();

        Ok(())
    }

    fn solve(&mut self) -> std::io::Result<()>{
        
        let windows = two_d_window(&self.matrix, (3, 3));
        self.result = windows
            .iter()
            .map(|window| {
                let mut matches = 0;
                let pattern = vec!['M', 'A', 'S'];
                let mut b_pattern = pattern.to_vec(); 
                b_pattern.reverse();

                let mut tests = Vec::new();

                let mut diag1 = Vec::new();
                let mut diag2 = Vec::new();

                diag1.push(window[0][0]);
                diag1.push(window[1][1]);
                diag1.push(window[2][2]);
        
                diag2.push(window[2][0]);
                diag2.push(window[1][1]);
                diag2.push(window[0][2]);
                
                tests.push(diag1.clone());
                tests.push(diag2.clone());

                matches += matrix_count_row_match(&tests, &pattern);
                matches += matrix_count_row_match(&tests, &b_pattern);
                
                if matches == 2 {
                    return 1
                }
                0
            })
            .sum();
        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
}


