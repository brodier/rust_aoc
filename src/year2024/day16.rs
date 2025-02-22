use std::collections::{HashMap, VecDeque};


#[derive(Debug)]
enum PuzzleError {
    OutOfBoard,
    _Wall
}


struct Puzzle {
    map:Vec<String>,
    start:(usize,usize),
    end:(usize,usize),
    work_map:HashMap<Position,usize>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    pos:(usize,usize),
    orientation:Dir,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl Dir {
    fn left(&self) -> Dir {
        match self {
            Dir::DOWN => Dir::RIGHT,
            Dir::RIGHT => Dir::UP,
            Dir::UP => Dir::LEFT,
            Dir::LEFT => Dir::DOWN,
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,
            Dir::UP => Dir::RIGHT,
            Dir::RIGHT => Dir::DOWN,
        }
    }

    fn back(&self) -> Dir {
        match self {
            Dir::DOWN => Dir::UP,
            Dir::LEFT => Dir::RIGHT,
            Dir::UP => Dir::DOWN,
            Dir::RIGHT => Dir::LEFT,
        }
    }
}

impl Dir {

    fn get_next(&self, pos:(usize,usize), board_size:&(usize,usize)) -> Result<(usize,usize), PuzzleError> {
        match self {
            Dir::UP => if pos.1 > 0 {
                 Ok((pos.0, pos.1 - 1))
            } else {
                Err(PuzzleError::OutOfBoard)
            },
            Dir::DOWN => if pos.1 + 1 < board_size.1 {
                Ok((pos.0,pos.1+1))
            } else  {
                Err(PuzzleError::OutOfBoard)
            },
            Dir::LEFT => if pos.0 > 0 {
                Ok((pos.0-1,pos.1))
            } else {
                Err(PuzzleError::OutOfBoard)
            },
            Dir::RIGHT => if pos.0 +1 < board_size.0 {
                Ok((pos.0+1,pos.1))
            } else {
                Err(PuzzleError::OutOfBoard)
            }
        }
    }

    fn all_dirs() -> [Dir;4] {
        [Dir::UP, Dir::LEFT, Dir::DOWN, Dir::RIGHT]
    }
}


impl Position {
    fn build(pos:(usize,usize), orientation:Dir) -> Position {
        Position{pos,orientation}
    }

    // return posible next positions with associate value
    fn get_next(&self, board_size:&(usize,usize)) -> [(Position,usize);3] {
        self.get_step(board_size,false)
    }

    fn get_prev(&self, board_size:&(usize,usize)) -> [(Position,usize);3] {
        self.get_step(board_size, true)
    }

    fn get_step(&self, board_size:&(usize,usize), prev:bool) -> [(Position,usize);3] {
        let orient = if prev { self.orientation.back() } else { self.orientation.clone() };
        let step_forward = Position::build(orient.get_next(self.pos, board_size).unwrap(),self.orientation.clone());
        let left = Position::build(self.pos, self.orientation.left());
        let right = Position::build(self.pos, self.orientation.right());
        [(left,1000), (right,1000),(step_forward,1) ]
    }
}

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
        Puzzle{map,start,end, work_map:HashMap::new()}
    }

    fn solve(&mut self) -> usize {
        let board_size = (self.map.first().unwrap().len(), self.map.len());
        let start = Position::build(self.start, Dir::RIGHT);
        self.work_map.insert(start.clone(), 0);
        let mut todo = VecDeque::new();
        todo.push_back(start.clone());
        while todo.len() > 0 {
            let pos = todo.pop_front().unwrap();
            let from_val = self.work_map.get(&pos).unwrap().clone();
            for next in pos.get_next(&board_size) {
                let c =  self.map[next.0.pos.1].as_bytes()[next.0.pos.0] as char;
                let new_val = from_val + next.1;
                if c == '#' {
                    continue;
                }
                if self.work_map.contains_key(&next.0) {
                    let current_val = self.work_map.get(&next.0).unwrap().clone();
                    if current_val > new_val {
                        self.work_map.insert(next.0.clone(), new_val);
                        todo.push_back(next.0);
                    }
                    // do nothing ignore longest path
                } else {
                    if let Some(_) = self.work_map.insert(next.0.clone(), new_val) {
                        panic!("Should never pass here")
                    }
                    todo.push_back(next.0);
                }
            }
        }
        let mut result = None;
        for end_dir in Dir::all_dirs() {
            let possible_end = Position::build(self.end, end_dir);
            if self.work_map.contains_key(&possible_end) && (result.is_none() || result.unwrap() > *self.work_map.get(&possible_end).unwrap()) {
                result = Some(*self.work_map.get(&possible_end).unwrap());
            }
        }
        if result.is_none() {
            panic!("Failed to found path to E from S");
        }
        result.unwrap()
    }

    fn solve_step2(self, step1_result:usize) -> usize {
        // go back from end to start and keep track of position on best bast
        let board_size = (self.map.first().unwrap().len(), self.map.len());
        let mut best_pos:Vec<(usize,usize)> = Vec::new();
        let mut visited = Vec::new();
        let mut todo = VecDeque::new();
        for end_dir in Dir::all_dirs() {
            let possible_end = Position::build(self.end, end_dir);
            if *self.work_map.get(&possible_end).unwrap() == step1_result {
                todo.push_back(possible_end);
            }
        }
        while todo.len() > 0 {
            let pos = todo.pop_front().unwrap();
            visited.push(pos.clone());
            for prev in pos.get_prev(&board_size) {
                if !self.work_map.contains_key(&prev.0) {
                    continue;
                }
                if *self.work_map.get(&pos).unwrap() <  self.work_map.get(&prev.0).unwrap() + prev.1 {
                    // minimal path from prev position is over the minial path to end so it is not on the best past
                    continue;
                }
                if visited.contains(&prev.0) || todo.contains(&prev.0){
                   continue; 
                }
                todo.push_back(prev.0);
            }
        }
        for pos in visited {
            if !best_pos.contains(&pos.pos) {
                best_pos.push(pos.pos);
            }
        }
        best_pos.len() // TODO return the good result
    }


}

pub fn solve(step:usize, puzzle_input:String) -> usize {
    let mut puzzle = Puzzle::build(puzzle_input);
    let result_step1 = puzzle.solve();
    if step == 1 {
        return result_step1;
    } else {
        return puzzle.solve_step2(result_step1);
    }
}