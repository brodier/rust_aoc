use std::{iter::Peekable, str::Lines};

use regex::Regex;

const TOKEN_A_COST:usize = 3;
const TOKEN_B_COST:usize = 1;

#[derive(Debug)]
struct ClawMachine {
    button_a:(i64,i64),
    button_b:(i64,i64),
    prize:(i64,i64)
}


type Puzzle = Vec<ClawMachine>;

fn load_line(pattern:&Regex, line:&str) -> (i64,i64) {
    pattern.captures_iter(line)
    .map(|caps| {
        let (_, [x, y]) = caps.extract();
        (x.parse().unwrap(), y.parse().unwrap())
    }).last().unwrap()
}

impl ClawMachine {
    fn build(lines_itt: &mut Peekable<Lines<'_>>) -> ClawMachine {
        if lines_itt.peek().unwrap().is_empty() {
            lines_itt.next();
        }
        let ba_pattern:Regex = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").expect("Invalid regex");
        let bb_pattern:Regex = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").expect("Invalid regex");
        let prize_pattern:Regex = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").expect("Invalid regex");
        ClawMachine{
            button_a:load_line(&ba_pattern, lines_itt.next().unwrap()), 
            button_b:load_line(&bb_pattern, lines_itt.next().unwrap()), 
            prize:load_line(&prize_pattern, lines_itt.next().unwrap())
        }
    }
    fn as_step_2(&self) -> ClawMachine {
        let step_2_offset:i64 = 10_000_000_000_000;
        ClawMachine{button_a:self.button_a, button_b: self.button_b, prize: (self.prize.0 + step_2_offset, self.prize.1 + step_2_offset)}
    }
}

fn load(puzzle_input:String, step2:bool) -> Puzzle {
    let mut puzzle = Vec::new();
    let mut lines_itt = puzzle_input.lines().into_iter().peekable();
    while lines_itt.peek().is_some() {
        if step2 {
            puzzle.push(ClawMachine::build(&mut lines_itt).as_step_2());
        } else {
            puzzle.push(ClawMachine::build(&mut lines_itt));
        }
        
    }
    puzzle
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

pub fn solve(step:usize, puzzle_input:String) -> String {
    let puzzle = load(puzzle_input, step == 2);
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