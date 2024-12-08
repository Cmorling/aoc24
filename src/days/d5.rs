use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use std::ops::ControlFlow;

use crate::day::Day;
use crate::util::{has_common_element, find_first_common_index};

#[derive(Default)]

pub struct D5 {
    pages: Vec<Vec<i32>>,
    page_rules: HashMap<i32, Vec<i32>>,
    result: i32,
}

impl D5 {
    fn correct_vector(&self, vec_a: &[i32]) -> Option<Vec<i32>> {
        match vec_a
            .iter()
            .enumerate()
            .try_for_each(|(ind, val)| {
                match self.page_rules.get(val) {
                    Some(r) => {
                        match find_first_common_index(&vec_a[0..ind], r) {
                            Some((i, _)) => {
                                let mut new = vec_a.to_vec();
                                new.remove(ind);
                                new.insert(i, *val);

                                match self.correct_vector(&new) {
                                    Some(n) => ControlFlow::Break(n),
                                    None => ControlFlow::Break(new)
                                }
                            },
                            None => ControlFlow::Continue(())
                        }
                        },
                    None => ControlFlow::Continue(())
                }
            }) 
        {
            ControlFlow::Break(n) => Some(n),
            ControlFlow::Continue(()) => None
        }
    }
}

impl Day for D5 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        (self.pages, self.page_rules) = reader
            .lines()
            .fold(
            (Vec::new(), HashMap::<i32, Vec<i32>>::new()), 
            |(mut p, mut pr), line| {
                let l = line.unwrap();     

                if l.contains("|") {
                    let Some((before, after)) = l
                        .split("|")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_tuple() else {todo!()};

                    pr.entry(before).or_insert_with(Vec::new).push(after);
                } else if l.contains(","){
                    p.push(l
                        .split(",")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect()
                        );
                } 
                (p, pr)
            });

        Ok(())
    }

    fn solve_part_one(&mut self) -> std::io::Result<()>{
        self.result = self.pages
            .iter()
            .map(|v| {
                match v
                    .iter()
                    .enumerate()
                    .try_for_each(|(ind, val)| {
                        match self.page_rules.get(val) {
                            Some(r) => {
                                match has_common_element(r, &v[0..ind]) {
                                    true => ControlFlow::Break(()),
                                    false => ControlFlow::Continue(())
                                }},
                            None => ControlFlow::Continue(())
                        }
                    }) 
                {
                    ControlFlow::Continue(()) => *v.get((v.len() - 1)/2).unwrap(),
                    ControlFlow::Break(()) => 0,
                }
            })
            .sum();
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()>{
        self.result = self.pages
            .iter()
            .map(|v| {
                match self.correct_vector(v) {
                    Some(corrected) => *corrected.get((corrected.len() - 1)/2).unwrap(),
                    None => 0
                }
            })
            .sum();
        Ok(())
    }


    fn get_solution(&self) -> String {
        self.result.to_string()
    }
}
