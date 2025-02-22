use std::{collections::HashMap};
use std::io::{stdin, Read};

use regex::Regex;

use super::load_puzzle;

const WIDTH:usize = 101;
const HEIGHT:usize = 103;

#[derive(Debug)]
struct Robot {
    pos:(usize,usize),
    speed:(usize,usize),
}


type Puzzle = Vec<Robot>;

impl Robot {
    fn build(line:&str) -> Robot {
        let in_re = Regex::new(r"[-+]?\d+").unwrap();
        let mut in_vec:Vec<i64> = Vec::new();
        for cap in in_re.captures_iter(line) {
            in_vec.push(cap[0].parse().unwrap());
        }
        Robot{
            pos:(in_vec[0] as usize,in_vec[1] as usize),
            speed:(((in_vec[2] + WIDTH as i64) % WIDTH as i64) as usize, ((in_vec[3] + HEIGHT as i64) % HEIGHT as i64) as usize), 
        }
    }

    fn apply_move(&mut self, nb_iter:usize) {
        self.pos.0 = (self.pos.0 + self.speed.0 * nb_iter) % WIDTH;
        self.pos.1 = (self.pos.1 + self.speed.1 * nb_iter) % HEIGHT;
    }

    fn get_cadran(&self) -> Option<usize> {
        if self.pos.0 == WIDTH / 2 || self.pos.1 == HEIGHT / 2 {
            return None;
        }
        let mut result: usize = 0;
        if self.pos.0 > WIDTH / 2 {
            result += 1;
        }
        if self.pos.1 > HEIGHT / 2 {
            result += 2;
        }
        return Some(result);
    }

}

fn display(puzzle: &Vec<Robot>) {
    let mut screen = [[' ' as u8;WIDTH];HEIGHT];
    for robot in puzzle {
        screen[robot.pos.1][robot.pos.0] = '#' as u8;
    }
    for data in screen {
        let line = std::str::from_utf8(data.as_slice()).unwrap();
        println!("{}", line);
    }
}

fn load(puzzle_input:String, _step2:bool) -> Puzzle {
    let mut puzzle = Vec::new();
    for line in puzzle_input.lines() {
        puzzle.push(Robot::build(line));
    }
    puzzle
}

fn solve(puzzle:&mut Vec<Robot>) -> usize {
    let mut result_map = HashMap::new();
    for robot in puzzle {
        robot.apply_move(100);
        if let Some(cadran) = robot.get_cadran() {
            result_map.entry(cadran).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    let mut result = 1;
    for i in result_map.values() {
        result *= i;
    }
    result as usize
}

pub fn day14(step:usize) -> usize {
    let puzzle_input = load_puzzle(14);
    let mut puzzle = load(puzzle_input, step == 2);

    for robot in puzzle.iter_mut() {
        robot.apply_move(8168);
    }
    display(&puzzle);

    // solve_step_by_step(&mut puzzle)
    return 0
}