use std::thread;
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

    pub fn solve1(&self, min_gain:usize) -> usize {
        let mut grid = vec![std::usize::MAX;self.board_size.0 * self.board_size.1];
        // Init grid
        for i in 0..self.path.len() {
            let pos = self.path[i];
            grid[pos.0 + self.board_size.0 * pos.1] = i;
        }
        let mut result = 0;
        let mut i = 0;
        for pos in self.path.iter() {
            let new_coord = pos.0 + 2 + self.board_size.0 * pos.1;
            if pos.0 + 2 < self.board_size.0 && grid[new_coord] != std::usize::MAX && grid[new_coord].abs_diff(i) >= min_gain + 2 {
                result += 1;
            }
            let new_coord = pos.0 + self.board_size.0 * (pos.1 + 2);
            if pos.1 + 2 < self.board_size.1 && grid[new_coord] != std::usize::MAX && grid[new_coord].abs_diff(i) >= min_gain + 2 {
                result += 1;
            }
            i+=1;
        }
        result
    }
    
    pub fn solve(&self, min_gain:usize, limit_cheat:usize) -> usize {
        if limit_cheat == 2 {
            return self.solve1(min_gain);
        }
        let nb_threads = std::thread::available_parallelism().unwrap().get();
        let mut workers = Vec::new();
        for i in 0..nb_threads {
            let path = self.path.clone();
            let worker = thread::spawn(move || {
                solve_one_part(i, nb_threads,  path, min_gain, limit_cheat)
            });
            workers.push(worker);
        }
        let mut result = 0;
        for worker in workers {
            result += worker.join().unwrap();
        }
        return result;
    }
}

fn solve_one_part(start:usize, step:usize, path:Vec<(usize,usize)>, min_gain:usize, limit_cheat:usize) -> usize {
    let mut r = 0;
    for from_idx in (start..path.len() - min_gain).step_by(step) {
        for to_idx in (from_idx + min_gain)..path.len() {
            let from = path[from_idx];
            let to = path[to_idx];
            let dist = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);
            if dist <= limit_cheat && from_idx + dist < to_idx && to_idx - (from_idx + dist) >= min_gain {
                r += 1;
            }
        }
    }
    return r;
}

pub fn solve(part:usize, input:String) -> String {
    let puzzle = Puzzle::build(input);
    if part == 1 {
        puzzle.solve(100, 2).to_string()
    } else {
        puzzle.solve(100, 20).to_string()
    }
}