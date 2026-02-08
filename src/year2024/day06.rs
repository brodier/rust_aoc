use crate::utils::grid::*;

type State = ((usize, usize), Dir);

pub struct Puzzle {
    state:State,
    try_num:u16,
    opt_block:bool,
    visited_state:[u16; 130*130*4],
    visited_pos:[bool; 130*130],
    nb_visted:u16,
    nb_loop:usize,
}

#[inline]
fn st_idx(((x,y),d):State) -> usize {
    y * 130 * 4 + x * 4 + d as usize
}

#[inline]
fn pos_idx(((x,y),_):State) -> usize {
    y * 130 + x
}

impl Puzzle {
    fn build(input:&str) -> (Grid, Puzzle) {
        let mut start = ((0,0),Dir::DOWN);
        for (numline, sline) in input.lines().enumerate() {
            if sline.contains("^") {    
                start = ((sline.find("^").unwrap(), numline), Dir::UP);
            }
        }
        if  matches!(start.1, Dir::DOWN)  {
            unreachable!("start has not be initialized !");
        }
        let visited_state = [0; 130*130*4];
        let mut visited_pos = [false; 130*130];
        visited_pos[pos_idx(start)] = true;
         (Grid::build(input.to_string()), Puzzle{state: start, try_num: 0, opt_block: true, visited_state, visited_pos, nb_visted:1, nb_loop:0})
    }

    fn walk(&mut self, grid:&mut Grid) -> bool {
        let gsize = grid.size();
        while let Ok(next_pos) = self.state.1.get_next(self.state.0,gsize) {
            if grid.get(next_pos).unwrap() == b'#' {
                self.state.1 = self.state.1.right()
            } else {
                if !self.visited_pos[pos_idx((next_pos,self.state.1))] {
                    if self.opt_block {
                        let previous = grid.set(next_pos, b'#').unwrap();
                        self.opt_block = false;
                        self.nb_visted += 1;
                        self.try_num = self.nb_visted;
                        let prev_state = self.state;
                        if self.walk(grid) == true {
                            self.nb_loop+=1;
                        }
                        self.try_num = 1;
                        self.state = prev_state;
                        self.opt_block = true;
                        let _ = grid.set(next_pos, previous);
                        self.visited_pos[pos_idx((next_pos,self.state.1))] = true;
                    }
                }
                self.state.0 = next_pos;
            }
            let new_state = st_idx(self.state);
            if self.visited_state[new_state] == 1 || self.visited_state[new_state] == self.try_num {
                return true;
            }
            self.visited_state[new_state] = self.try_num;
        }
        return false;
    }
}

pub fn parse(input:String) -> Puzzle {
    let (mut g, mut p) = Puzzle::build(&input);
    p.walk(&mut g);
    p
}

pub fn part1(p:&Puzzle) -> String {
    p.nb_visted.to_string()
}


pub fn part2(p:&Puzzle) -> String {
    p.nb_loop.to_string()
}