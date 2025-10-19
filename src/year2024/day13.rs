use std::{iter::Peekable, str::Lines};

use crate::utils::common::parse_i64;

const TOKEN_A_COST:usize = 3;
const TOKEN_B_COST:usize = 1;

#[derive(Debug,Clone, Copy)]
pub struct ClawMachine {
    button_a:(i64,i64),
    button_b:(i64,i64),
    prize:(i64,i64)
}


impl ClawMachine {
    fn build(lines_itt: &mut Peekable<Lines<'_>>) -> ClawMachine {
        if lines_itt.peek().unwrap().is_empty() {
            lines_itt.next();
        }
        let ba = parse_i64(lines_itt.next().unwrap());
        let bb = parse_i64(lines_itt.next().unwrap());
        let prize = parse_i64(lines_itt.next().unwrap());
        ClawMachine{
            button_a:(ba[0],ba[1]), 
            button_b:(bb[0],bb[1]), 
            prize:(prize[0],prize[1]),
        }
    }
    fn fix_conversion(&mut self) {
        const STEP_2_OFFSET:i64 = 10_000_000_000_000;
        self.prize.0 += STEP_2_OFFSET;
        self.prize.1 += STEP_2_OFFSET
    }
}

pub fn get_max(button:(usize,usize), prize_dist:(usize,usize)) -> Option<usize> {
    let mut max = 101;
    let max_0 = prize_dist.0 / button.0;
    if max_0 < max {
        max = max_0;
    }
    let max_1 = prize_dist.1 / button.1;
    if max_1 < max {
        max = max_1;
    }    
    if max == 101 {
        Some(100)
    } else {
        Some(max)
    }
}

fn search(mc:&ClawMachine) -> Option<(i64,i64)> {
    let calc1 = mc.button_b.1*mc.button_a.0 - mc.button_a.1*mc.button_b.0;
    if calc1 == 0 {
        return None;
    } 
    let nb_a = (mc.button_b.1*mc.prize.0-mc.prize.1*mc.button_b.0)/calc1;
    let nb_b = (mc.prize.1 - nb_a * mc.button_a.1) / mc.button_b.1;
    if nb_a * mc.button_a.0 + nb_b * mc.button_b.0 == mc.prize.0 && nb_a * mc.button_a.1 + nb_b * mc.button_b.1 == mc.prize.1 {
        Some((nb_a,nb_b))
    } else {
        None
    }
}

pub fn solve(puzzle:&Vec<ClawMachine>) -> String {
    let mut result = 0;
    for p in puzzle.iter() {
        let search = search(p);
        if let Some((a ,b )) = search {
            //println!("{:?} => {} {}", p, a, b);
            result += TOKEN_A_COST * a as usize + TOKEN_B_COST * b as usize;
        } else {
            //println!("No sol for : {:?}", p);
        }
    }
    result.to_string()
}


pub fn parse(input:String) -> Vec<ClawMachine> {
    let mut puzzle = Vec::new();
    let mut lines_itt = input.lines().into_iter().peekable();
    while lines_itt.peek().is_some() {
        puzzle.push(ClawMachine::build(&mut lines_itt));        
    }
    puzzle
}

pub fn part1(input:&Vec<ClawMachine>) -> String {
    solve(input)
}

pub fn part2(input:&Vec<ClawMachine>) -> String {
    let puzzle = input.iter().map(|c| { 
        let mut cc = c.clone(); 
        cc.fix_conversion(); 
        cc}).collect();
    solve(&puzzle) 
}