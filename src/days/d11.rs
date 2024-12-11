use std::fs::File;
use std::io::{BufReader, BufRead};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::day::Day;

#[derive(Debug)]
struct Node {
    value: u64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: u64) -> Self {
        Node {
            value,
            children: Vec::new(),
        }
    }
    fn get_even_numbers(&self) -> Option<(u64, u64)> {
        let string = self.value.to_string();
        if string.len() % 2 == 0 {
            let half = string.len() / 2;
            let rhs = &string[0..half];
            let lhs = &string[half..];
            return Some((rhs.parse::<u64>().expect("Could not parse"), lhs.parse::<u64>().expect("Could not parse")));
        }
        None
    }

    fn calc_children(&self) -> Vec<u64> {
        if self.value == 0 {
            vec![1]
        } else if let Some((rhs, lhs)) = self.get_even_numbers() {
            return vec![rhs, lhs];
        } else {
            return vec![self.value * 2024];
        }
    }

    fn link_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }
}

#[derive(Default)]

pub struct D11 {
    start_rocks: Vec<u64>,
    cached_nodes: HashMap<u64, Rc<RefCell<Node>>>,
    cached_paths: HashMap<(u8, u64), u64>,
    result: u64
}

impl D11 {
    fn dfs(&mut self, current_node: Rc<RefCell<Node>>, blinks: u8) -> u64 {
        if blinks == 0 {
            return 1;
        }

        let mut imut_node = current_node.borrow();

        if imut_node.children.is_empty() {
            drop(imut_node);
            let mut mut_node = current_node.borrow_mut();

            let constructed_values= mut_node.calc_children();
            for value in constructed_values.iter() {
                if let Some(already_constructed) = self.cached_nodes.get(value) {

                   mut_node.link_child(Rc::clone(already_constructed)); 

                } else {

                    let new_node = Rc::new(RefCell::new(Node::new(*value)));
                    self.cached_nodes.insert(*value, Rc::clone(&new_node));
                    mut_node.link_child(Rc::clone(&new_node)); 

                }
            }
            drop(mut_node);
            imut_node = current_node.borrow();
        } 

        if let Some(path) = self.cached_paths.get(&(blinks, imut_node.value)) {
            *path
            
        } else {
            let mut res = 0;
            for child in imut_node.children.iter() {
                res += self.dfs(Rc::clone(child), blinks - 1);
            }

            self.cached_paths.insert((blinks, imut_node.value), res);
            res
        }
    }
    fn solve(&mut self, iterations: u8) {
        let c_start_rocks = self.start_rocks.clone();
        self.result = c_start_rocks
            .iter()
            .map(|rock| {
                match self.cached_nodes.get(rock) {
                    Some(ref_node) => {
                        self.dfs(Rc::clone(ref_node), iterations) 
                    },
                    None => {
                        let new_node = Rc::new(RefCell::new(Node::new(*rock)));
                        self.cached_nodes.insert(*rock, Rc::clone(&new_node));
                        self.dfs(Rc::clone(&new_node), iterations)
                    },
                }
            })
            .sum();
    }
}

impl Day for D11 {
    fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut start_state = String::new();
        reader.read_line(&mut start_state)?;
        
        self.start_rocks = start_state
            .split_whitespace()
            .fold(Vec::new(), |mut rocks, rock| {
                rocks.push(rock.parse::<u64>().expect("Could not convert roc"));
                rocks
            });
        Ok(())
    }

    
    fn solve_part_one(&mut self) ->  std::io::Result<()>{
        self.solve(25);
        Ok(())
    }

    fn solve_part_two(&mut self) -> std::io::Result<()> {
        self.solve(75);
        Ok(())
    }

    fn get_solution(&self) -> String {
        self.result.to_string()
    }
} 
