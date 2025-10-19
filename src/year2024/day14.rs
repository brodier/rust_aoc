use std::collections::HashMap;
use crate::utils::common::parse_i64;


const WIDTH:usize = 101;
const HEIGHT:usize = 103;

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pos:(usize,usize),
    speed:(usize,usize),
}

pub type Puzzle = Vec<Robot>;

impl Robot {
    fn build(line:&str) -> Robot {
        let in_vec = parse_i64(line);
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

fn _display(puzzle: &Vec<Robot>) {
    let mut screen = [[' ' as u8;WIDTH];HEIGHT];
    for robot in puzzle {
        screen[robot.pos.1][robot.pos.0] = '#' as u8;
    }
    for data in screen {
        let line = std::str::from_utf8(data.as_slice()).unwrap();
        println!("{}", line);
    }
}

pub fn parse(input:String) -> Puzzle {
    let mut puzzle = Vec::new();
    for line in input.lines() {
        puzzle.push(Robot::build(line));
    }
    puzzle
}

pub fn part1(input:&Puzzle) -> String {
    let mut result_map = HashMap::new();
    for mut robot in input.iter().map(|c| c.clone()) {
        robot.apply_move(100);
        if let Some(cadran) = robot.get_cadran() {
            result_map.entry(cadran).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    let mut result = 1;
    for i in result_map.values() {
        result *= i;
    }
    result.to_string()
}

pub fn part2(input:&Puzzle) -> String {
    let mut puzzle:Vec<Robot> = input.iter().map(|c| c.clone()).collect();
    // looking for box surrounding Christmas Tree 31x33
    let mut possible_align_rows:Vec<usize> = Vec::new();
    let mut possible_align_cols:Vec<usize> = Vec::new();
    for i in 0..103 {
        let mut rows_count:[usize;103] = [0;103];
        let mut cols_count:[usize;101] = [0;101];
        for r in puzzle.iter_mut() {
            cols_count[r.pos.0] += 1;
            rows_count[r.pos.1] += 1;
            r.apply_move(1);
        }
        // println!("rows count ({}) : {:?}", i, rows_count);
        // println!("cols count ({}) : {:?}", i, cols_count);
        if cols_count.as_slice().iter().filter(|v| **v>=33 ).count() > 1 {
            possible_align_cols.push(i);
        }
        if rows_count.as_slice().iter().filter(|v| **v>=31 ).count() > 1 {
            possible_align_rows.push(i);
        }
    }
    let mut min  = 101 * 103;
    for coltime in possible_align_cols  {
        for rowtime in possible_align_rows.iter() {
            let result = (5253 * coltime + 5151 * rowtime) % 10403;
            if result < min {
                min = result;
            }
        }
    }
    min.to_string()
}