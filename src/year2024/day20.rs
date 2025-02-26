use std::collections::HashMap;

use crate::utils::grid::Dir;

const EMPTY:u8 = '.' as u8;

struct Puzzle {
    map:Vec<String>,
    start:(usize,usize),
    end:(usize,usize),
    board_size:(usize,usize),
    idx_on_path:HashMap<(usize,usize), usize>,
    path:Vec<(usize,usize)>,
}

type Step = ((usize,usize),Dir);

impl Puzzle {
    fn build(input:String) -> Puzzle {
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
        let mut puzzle = Puzzle{map,start,end, board_size, idx_on_path:HashMap::new(), path:Vec::new()};
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
        let mut ind:usize = 0;
        while pos.0 != self.end {
            //println!("setting path [{}] {:?}", ind, pos.0);
            self.path.push(pos.0);
            self.idx_on_path.insert(pos.0,ind);
            pos = self.get_next_step(pos);
            ind+=1;
        }
        self.path.push(self.end);
        self.idx_on_path.insert(self.end,ind);
    }


    fn solve(&self) -> usize {
        let mut cheats = HashMap::new();
        let mut walker = self.path.iter();
        let mut idx = 2;
        while let Some(&pos) = walker.next() {
            if pos == self.end {
                break;
            }
            for dir in Dir::all() {
                // make double step and check if is valid cheat and how many time cheat permit to win
                let new_pos = dir.get_next(pos, self.board_size).unwrap();
                if let Ok(new_pos) = dir.get_next(new_pos, self.board_size) {
                    if self.get(new_pos) == EMPTY {
                        let &idx_for_new_pos = self.idx_on_path.get(&new_pos).unwrap();
                        if idx_for_new_pos > idx {
                            let gain = idx_for_new_pos - idx;
                            let cg = cheats.get(&gain).unwrap_or(&0);
                            //println!("cheat found from {:?} to {:?} winning {} picosec.", pos, new_pos, gain);
                            cheats.insert(gain, cg + 1);
                        }
                    }
                }
            }
            idx += 1;
        }
        //println!("Cheats map : {:?}",cheats);
        let mut result:usize = 0;
        for c in cheats {
            if c.0 > 99 {
                result += c.1;
            }
        }
        result
    }
}

pub fn solve(_part:usize, input:String) -> String {
    let puzzle = Puzzle::build(input);
    let result = puzzle.solve();
    result.to_string()
}