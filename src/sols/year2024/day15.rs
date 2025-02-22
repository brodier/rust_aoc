use std::error::Error;
use std::{collections::HashMap};
use std::io::{stdin, stdout, Write, Read};
use regex::Regex;

use super::load_puzzle;

const WIDTH:usize = 101;
const HEIGHT:usize = 103;

#[derive(Debug)]
struct Robot {
    pos:(usize,usize),
}

#[derive(Debug)]
enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

#[derive(Debug)]
enum PuzzleError {
    OUT_OF_BOARD,
    WALL
}

struct Puzzle {
    map:Vec<String>,
    robot:(usize,usize),
    path:Vec<Dir>,
}

impl Puzzle {
    fn build(input:String, mode:usize) -> Puzzle {
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

                let line = if mode == 2 {
                    line.replace("#", "##").replace("O", "[]").replace(".", "..").replace("@", "@.")
                } else {
                    line.to_string()
                };

                if let Some(x_pos) = line.find("@") {
                    robot.0 = x_pos;
                    robot.1 = line_counter;
                }
                map.push(line);
            } else {
                for c in line.chars() {
                    path.push(Dir::build(c));
                }
            }
            line_counter += 1;
        }
        Puzzle{map,robot,path}
    }
    
    fn compute_gps(&self) -> usize {
        let mut result = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map.first().unwrap().len() {
                if self.map[y].as_bytes()[x] == b'O' || self.map[y].as_bytes()[x] == b'[' {
                    result += 100 * y + x;
                }
            }
        }
        result
    } 

    fn apply_path_to_robot(&mut self) {
        let board_size = (self.map.first().unwrap().len(), self.map.len());
        for step in self.path.iter().as_ref() {
            let mut block_to_check = Vec::new();
            let mut block_to_move = Vec::new();
            let mut checked_pos = Vec::new();
            let mut curr_pos = self.robot;
            let mut bc:usize = 0;
            let mut step_result = Err(PuzzleError::WALL);
            'check: while let Ok(next_pos) = step.get_next(curr_pos, board_size) {
                if next_pos == curr_pos {
                    panic!("Should never pass here");
                }
                let c =  self.map[next_pos.1].as_bytes()[next_pos.0] as char;
                checked_pos.push(curr_pos);
                curr_pos = next_pos;
                if c=='.' {
                    if block_to_check.len() == 0 {
                        step_result = Ok(bc);
                        break;
                    }
                    curr_pos = block_to_check.pop().unwrap();
                } else if c=='O' {
                    bc+=1
                } else if c=='[' || c == ']' {
                    match step {
                        Dir::LEFT | Dir::RIGHT => {
                            curr_pos = step.get_next(curr_pos, board_size).unwrap();
                            if c=='[' {
                                block_to_move.push((curr_pos.0-1,curr_pos.1));
                            } else {
                                block_to_move.push(curr_pos);
                            }
                        },
                        Dir::UP | Dir::DOWN => {
                            if c==']' {
                                curr_pos = (curr_pos.0-1,curr_pos.1)
                            }
                            let pos_to_check = (curr_pos.0+1,curr_pos.1);
                            block_to_check.push(pos_to_check);
                            block_to_move.push(curr_pos);
                        }
                    }
                } else if c == '#' {
                    step_result = Err(PuzzleError::WALL);
                    break;
                }
                while checked_pos.contains(&curr_pos) {
                    if block_to_check.len() == 0 {
                        step_result = Ok(bc);
                        break 'check;
                    } 
                    curr_pos = block_to_check.pop().unwrap();
                }
            }
            if step_result.is_err() {
                continue;
            }
            // moving block first
            if block_to_move.len() > 0 {
                block_to_move.sort_unstable_by(|b1,b2|
                    match step {
                        Dir::RIGHT => {
                            b1.0.cmp(&b2.0)
                        },
                        Dir::LEFT => {
                            b2.0.cmp(&b1.0)
                        },
                        Dir::DOWN => {
                            b1.1.cmp(&b2.1)
                        },
                        Dir::UP => {
                            b2.1.cmp(&b1.1)
                        },
                    });
            }
            while block_to_move.len() > 0 {
                let b = block_to_move.pop().unwrap();
                match step {
                    Dir::UP | Dir::DOWN => {
                        self.map[b.1].replace_range(b.0..b.0+2, "..");        
                    },
                    Dir::LEFT|Dir::RIGHT => {}
                }
                let b= step.get_next(b, board_size).unwrap();
                self.map[b.1].replace_range(b.0..b.0+2, "[]");
            }
            // Positionning on first bloc 'O' to move
            curr_pos = step.get_next(self.robot, board_size).unwrap();
            if bc > 0 {
                while bc > 0 {
                    curr_pos = step.get_next(curr_pos, board_size).unwrap();
                    bc -=1;
                }
                self.map[curr_pos.1].replace_range(curr_pos.0..curr_pos.0+1, "O");
            } 
            curr_pos = self.robot;
            self.map[curr_pos.1].replace_range(curr_pos.0..curr_pos.0+1, ".");
            curr_pos = step.get_next(curr_pos, board_size).unwrap();
            self.map[curr_pos.1].replace_range(curr_pos.0..curr_pos.0+1, "@");
            self.robot = curr_pos;
        }
    }

}

impl Dir {
    fn build(c:char) -> Dir {
        match c {
            '<' => Dir::LEFT,
            '^' => Dir::UP,
            '>' => Dir::RIGHT,
            'v' => Dir::DOWN,
            _ => panic!("Invalid char for building Dir")
        }
    }

    fn get_next(&self, pos:(usize,usize), board_size:(usize,usize)) -> Result<(usize,usize), PuzzleError> {
        match self {
            Dir::UP => if pos.1 > 0 {
                 Ok((pos.0, pos.1 - 1))
            } else {
                Err(PuzzleError::OUT_OF_BOARD)
            },
            Dir::DOWN => if pos.1 + 1 < board_size.1 {
                Ok((pos.0,pos.1+1))
            } else  {
                Err(PuzzleError::OUT_OF_BOARD)
            },
            Dir::LEFT => if pos.0 > 0 {
                Ok((pos.0-1,pos.1))
            } else {
                Err(PuzzleError::OUT_OF_BOARD)
            },
            Dir::RIGHT => if pos.0 +1 < board_size.0 {
                Ok((pos.0+1,pos.1))
            } else {
                Err(PuzzleError::OUT_OF_BOARD)
            }
        }
    }
}

fn _press_any_key_to_continue() {
    println!("Press any key to continue...");
    let _ = stdout().flush(); // Ensure the message is printed before waiting for input
    let _ = stdin().read(&mut [0u8]).unwrap(); // Wait for a single byte input
}

pub fn day15(step:usize) -> usize {
    let puzzle_input = load_puzzle(15);
    let mut puzzle = Puzzle::build(puzzle_input, step);
    puzzle.apply_path_to_robot();
    puzzle.compute_gps()
}