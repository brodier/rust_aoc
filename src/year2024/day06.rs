use crate::utils::grid::*;

type State = ((usize, usize), Dir);

struct Puzzle {
    state:State,
    visited:[bool; 130*130*5],
    nb_visted:usize,
    nb_loop:usize,
}

fn idx(s:State) -> usize {
    s.0.0 * 130 * 5 + s.0.1 * 5 + s.1 as usize
}

fn pos_idx(s:State) -> usize {
    s.0.0 * 130 * 5 + s.0.1 * 5 + 4
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
        let mut visited = [false; 130*130*5];
        visited[idx(start)] = true;
         (Grid::build(input.to_string()), Puzzle{state: start, visited, nb_visted:0, nb_loop:0})
    }

    fn fork(&self) -> Puzzle {
        Puzzle { state: self.state, visited: self.visited.clone(), nb_visted: self.nb_visted , nb_loop: self.nb_loop}
    }

    fn walk(&mut self, grid:&mut Grid, part:usize) -> bool {
        let gsize = grid.size();
        while let Ok(next_pos) = self.state.1.get_next(self.state.0,gsize) {
            if grid.get(next_pos).unwrap() == b'#' {
                self.state.1 = self.state.1.right()
            } else {
                if !self.visited[pos_idx((next_pos,self.state.1))] {
                    // on walk 2 try to put block clone current puzzle and walk until end or loop
                    if part == 2 {
                        let previous = grid.set(next_pos, b'#').unwrap();
                        let mut fork_puzzle = self.fork();
                        if fork_puzzle.walk(grid,1) == true {
                            self.nb_loop+=1;
                        }
                        let _ = grid.set(next_pos, previous);
                    }
                    self.visited[pos_idx((next_pos,self.state.1))] = true;
                    self.nb_visted += 1;
                }
                self.state.0 = next_pos;
            }
            if self.visited[idx(self.state)] {
                return true;
            }
            self.visited[idx(self.state)] = true;
        }
        return false;
    }
}

pub fn parse(input:String) -> String {
    input
}

pub fn part1(input:&str) -> String {
    let (mut g, mut p) = Puzzle::build(input);
    p.walk(&mut g, 1);
    p.nb_visted.to_string()
}


pub fn part2(input:&str) -> String {
    let (mut g, mut p) = Puzzle::build(input);
    p.walk(&mut g, 2);
    p.nb_loop.to_string()
}