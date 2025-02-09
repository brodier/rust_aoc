use std::{collections::HashMap};
use std::io::{stdin, Read};

use regex::Regex;

use super::load_puzzle;

const WIDTH:usize = 101;
const HEIGHT:usize = 103;

#[derive(Debug)]
struct Robot {
    pos:(usize,usize),
}

enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

struct Puzzle {
    map:Vec<String>,
    robot:(usize,usize),
    path:Vec<String>,
}

impl Puzzle {
    fn build(input:String) -> Puzzle {
        let mut map = Vec::new();
        let mut path = Vec::new();
        let mut robot:(usize,usize) = (0,0);
        let mut load_map = true;
        let mut line_counter = 0;
        for line in input.lines() {
            if line.len() == 0 {
                load_map = false;
                continue;
            }
            if load_map {
                if let Some(x_pos) = line.find("@") {
                    robot.0 = x_pos;
                    robot.1 = line_counter;
                }
                map.push(line.to_string());
            } else {
                path.push(line.to_string());
            }
            line_counter += 1;
        }
        Puzzle{map,robot,path}
    }
    
    fn apply_path_to_robot(&mut self) {

    }

    fn compute_gps(&self) -> usize {
        0
    } 
}

pub fn day15(step:usize) -> usize {
    let puzzle_input = load_puzzle(15);
    let mut puzzle = Puzzle::build(puzzle_input);

    puzzle.apply_path_to_robot();
    puzzle.compute_gps()
}