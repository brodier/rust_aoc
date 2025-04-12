use std::collections::HashMap;

use crate::utils::grid::Dir;

const EMPTY:u8 = '.' as u8;

pub struct Puzzle {
    map:Vec<String>,
    start:(usize,usize),
    end:(usize,usize),
    board_size:(usize,usize),
    path:Vec<(usize,usize)>,
}

type Step = ((usize,usize),Dir);

impl Puzzle {
    pub fn build(input:String) -> Puzzle {
        let mut map = Vec::new();
        let mut start = (0,0);
        let mut end=(0,0);
        let mut line_number = 0;
        for line in input.lines() {
            if let Some(x_pos) = line.find("S") {
                start.0 = x_pos;
                start.1 = line_number;
            }
            if let Some(x_pos) = line.find("E") {
                end.0 = x_pos;
                end.1 = line_number;
            }
            map.push(line.to_string());
            line_number +=1;
        }
        map[end.1] = map[end.1].replace("E", ".");
        let board_size = (map.first().unwrap().len(), map.len());
        let mut puzzle = Puzzle{map,start,end, board_size, path:Vec::new()};
        puzzle.init_path();
        puzzle
    }

    fn init_dir(&self) -> Dir {
        for dir in Dir::all() {
            let next = dir.get_next(self.start, self.board_size).unwrap();
            if EMPTY == self.get(next) {
                return dir;
            } 
        }
        panic!("No path from start");
    }

    fn get(&self, pos:(usize,usize)) -> u8 {
        self.map[pos.1].as_bytes()[pos.0]
    }

    fn check_move(&self, step:Step) -> Option<Step> {
        let next = step.1.get_next(step.0, self.board_size).unwrap();
        if EMPTY == self.get(next) {
            return Some((next,step.1));
        } else {
            return None;
        }
    }
    
    fn get_next_step(&self, pos:Step) -> Step {
        for dir in [pos.1, pos.1.left(), pos.1.right()] {
            if let Some(next_pos) = self.check_move((pos.0,dir)) {
                return next_pos;
            }
        }
        panic!("Not any path");
    }

    fn init_path(&mut self) {
        let mut pos = (self.start, self.init_dir());
        while pos.0 != self.end {
            //println!("setting path [{}] {:?}", ind, pos.0);
            self.path.push(pos.0);
            pos = self.get_next_step(pos);
        }
        self.path.push(self.end);
    }


    pub fn solve(&self, min_gain:usize, limit_cheat:usize) -> usize {
        let mut cheats = HashMap::new();
        for i in 0.. self.path.len() {
            for j in i+1..self.path.len() {
                let from  = self.path[i];
                let to = self.path[j];
                let dist = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);
                if dist <= limit_cheat {
                    if i + dist < j {
                        let gain = j - (i + dist);
                        let cg = cheats.get(&gain).unwrap_or(&0);
                        //println!("cheat found from {:?} to {:?} winning {} picosec.", pos, new_pos, gain);
                        cheats.insert(gain, cg + 1);
                    }
                }
            }
        }
        //println!("Cheats map : {:?}",cheats);
        let mut result:usize = 0;
        for c in cheats {
            if c.0 >= min_gain {
                result += c.1;
            }
        }
        result
    }
}

pub fn solve(part:usize, input:String) -> String {
    let puzzle = Puzzle::build(input);
    if part == 1 {
        puzzle.solve(100, 2).to_string()
    } else {
        puzzle.solve(100, 20).to_string()
    }
}