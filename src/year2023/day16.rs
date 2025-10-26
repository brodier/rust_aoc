use std::{sync::Arc, thread};


#[derive(Clone,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone,Debug)]
pub struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
struct BeamState {
    pos:(usize, usize),
    direction:Direction,
}

impl BeamState {

    fn id(&self, gird:&Grid) -> usize {
        match self.direction {
            Direction::Up => self.pos.1 * gird.width + self.pos.0,
            Direction::Down => (gird.height + self.pos.1) * gird.width + self.pos.0,
            Direction::Left => (2 * gird.height + self.pos.1) * gird.width + self.pos.0,
            Direction::Right => (3 * gird.height + self.pos.1) * gird.width + self.pos.0,
        }
    }

    fn next(&self, grid:&Grid) -> (Option<BeamState>, Option<BeamState>) {
        match self.direction {
            Direction::Up => self.next_up(grid),
            Direction::Down => self.next_down(grid),
            Direction::Left => self.next_left(grid),
            Direction::Right => self.next_right(grid),
        }
    }

    
    fn init_dir(&self, on:u8) -> (Option<BeamState>, Option<BeamState>) {
        let mut result = self.clone();
        match (on, self.direction.clone()) {
            (b'.', _) => return (Some(result), None),
            (b'-', Direction::Left|Direction::Right) => { return (Some(result), None); },
            (b'|', Direction::Down|Direction::Up) => { return (Some(result), None); },
            (b'/', Direction::Left) => { result.direction = Direction::Down; return (Some(result), None); },
            (b'/', Direction::Right) => { result.direction = Direction::Up; return (Some(result), None); },
            (b'/', Direction::Down) => { result.direction = Direction::Left; return (Some(result), None); },
            (b'/', Direction::Up) => { result.direction = Direction::Right; return (Some(result), None); },
            (b'\\', Direction::Left) => { result.direction = Direction::Up; return (Some(result), None); },
            (b'\\', Direction::Right) => { result.direction = Direction::Down; return (Some(result), None); },
            (b'\\', Direction::Down) => { result.direction = Direction::Right; return (Some(result), None); },
            (b'\\', Direction::Up) => { result.direction = Direction::Left; return (Some(result), None); },
            (b'|', Direction::Left|Direction::Right) => { 
                let mut result2 = result.clone();
                result.direction = Direction::Up; 
                result2.direction = Direction::Down; 
                return (Some(result), Some(result2)); 
            },
            (b'-', Direction::Down|Direction::Up) => {
                let mut result2 = result.clone();
                result.direction = Direction::Left; 
                result2.direction = Direction::Right;
                return (Some(result), Some(result2)); },
            _ => unreachable!()
        }
    }

    fn next_up(&self, grid:&Grid) -> (Option<BeamState>, Option<BeamState>) {
        if self.pos.1 == 0 {
            return (None,None);
        }
        let new_pos = (self.pos.0, self.pos.1 - 1);
        let new_pos_is = grid.grid[new_pos.1 * (grid.width + 1) + new_pos.0];
        match new_pos_is {
            b'.' => return (Some(BeamState{pos:new_pos, direction:Direction::Up}), None),
            b'/' => return (Some(BeamState{pos:new_pos, direction:Direction::Right}), None),
            b'\\' => return (Some(BeamState{pos:new_pos, direction:Direction::Left}), None),
            b'|' => return (Some(BeamState{pos:new_pos, direction:Direction::Up}), None),
            b'-' => return (Some(BeamState{pos:new_pos, direction:Direction::Right}), Some(BeamState{pos:new_pos, direction:Direction::Left})),
            _ => unreachable!()
        }
    }

    fn next_down(&self, grid:&Grid) -> (Option<BeamState>, Option<BeamState>) {
        if self.pos.1 == grid.height - 1 {
            return (None,None);
        }
        let new_pos = (self.pos.0, self.pos.1 + 1);
        let new_pos_is = grid.grid[new_pos.1 * (grid.width + 1) + new_pos.0];
        match new_pos_is {
            b'.' => return (Some(BeamState{pos:new_pos, direction:Direction::Down}), None),
            b'/' => return (Some(BeamState{pos:new_pos, direction:Direction::Left}), None),
            b'\\' => return (Some(BeamState{pos:new_pos, direction:Direction::Right}), None),
            b'|' => return (Some(BeamState{pos:new_pos, direction:Direction::Down}), None),
            b'-' => return (Some(BeamState{pos:new_pos, direction:Direction::Right}), Some(BeamState{pos:new_pos, direction:Direction::Left})),
            _ => unreachable!()
        }
    }

    fn next_left(&self, grid:&Grid) -> (Option<BeamState>, Option<BeamState>) {
        if self.pos.0 == 0 {
            return (None,None);
        }
        let new_pos = (self.pos.0 - 1, self.pos.1);
        let new_pos_is = grid.grid[new_pos.1 * (grid.width + 1) + new_pos.0];
        match new_pos_is {
            b'.' => return (Some(BeamState{pos:new_pos, direction:Direction::Left}), None),
            b'/' => return (Some(BeamState{pos:new_pos, direction:Direction::Down}), None),
            b'\\' => return (Some(BeamState{pos:new_pos, direction:Direction::Up}), None),
            b'-' => return (Some(BeamState{pos:new_pos, direction:Direction::Left}), None),
            b'|' => return (Some(BeamState{pos:new_pos, direction:Direction::Down}), Some(BeamState{pos:new_pos, direction:Direction::Up})),
            _ => unreachable!()
        }
    }

    fn next_right(&self, grid:&Grid) -> (Option<BeamState>, Option<BeamState>) {
        if self.pos.0 == grid.width - 1 {
            return (None,None);
        }
        let new_pos = (self.pos.0 + 1, self.pos.1);
        let new_pos_is = grid.grid[new_pos.1 * (grid.width + 1) + new_pos.0];
        match new_pos_is {
            b'.' => return (Some(BeamState{pos:new_pos, direction:Direction::Right}), None),
            b'/' => return (Some(BeamState{pos:new_pos, direction:Direction::Up}), None),
            b'\\' => return (Some(BeamState{pos:new_pos, direction:Direction::Down}), None),
            b'-' => return (Some(BeamState{pos:new_pos, direction:Direction::Right}), None),
            b'|' => return (Some(BeamState{pos:new_pos, direction:Direction::Down}), Some(BeamState{pos:new_pos, direction:Direction::Up})),
            _ => unreachable!()
        }
    }
   
}

pub fn parse(input:String) -> Grid {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    Grid{grid:input.as_bytes().to_vec(), width, height}
}

fn solve(init_beam:&BeamState, grid:&Grid) -> usize {
    let mut beams:Vec<Option<BeamState>> = Vec::new();
    let mut loop_breaker = vec![false;grid.width * grid.height * 4];
    let mut energized_position = vec![false;grid.width * grid.height];
    let index = init_beam.pos.1 * grid.width + init_beam.pos.0;
    energized_position[index] = true;
    let start_on = grid.grid[init_beam.pos.1 * (grid.width + 1) + init_beam.pos.0];
    let init_beams = init_beam.init_dir(start_on);
    beams.push(init_beams.0);
    beams.push(init_beams.1);
    while let Some(beam) = beams.pop() {
        if let Some(beam) = beam {
            if !loop_breaker[beam.id(grid)] {
                let index = beam.pos.1 * grid.width + beam.pos.0;
                energized_position[index] = true;
                loop_breaker[beam.id(grid)] = true;
                let (next1, next2) = beam.next(grid);
                beams.push(next1);
                beams.push(next2);

            }
        }
    }
    energized_position.iter().filter(|&&v| v).count()
}


pub fn part1(grid:&Grid) -> String {
    let init_beam = BeamState{pos:(0,0), direction:Direction::Right};
    solve(&init_beam, grid).to_string()
}

pub fn part2(grid:&Grid) -> String {
    let mut results = Vec::new();
    let grid = Arc::new(grid.clone());
    for i in 0..grid.width {
        let init_beam = BeamState{pos:(i,0), direction:Direction::Down};
        let th_grid = Arc::clone(&grid);
        results.push(thread::spawn(move || solve(&init_beam, &th_grid.as_ref())));
        let init_beam = BeamState{pos:(i,grid.height-1), direction:Direction::Up};
        let th_grid = Arc::clone(&grid);
        results.push(thread::spawn(move || solve(&init_beam, &th_grid.as_ref())));
    }
    for i in 0..grid.height {
        let init_beam = BeamState{pos:(0,i), direction:Direction::Right};
        let th_grid = Arc::clone(&grid);
        results.push(thread::spawn(move || solve(&init_beam, &th_grid.as_ref())));
        let init_beam = BeamState{pos:(grid.width-1,i), direction:Direction::Left};
        let th_grid = Arc::clone(&grid);
        results.push(thread::spawn(move || solve(&init_beam, &th_grid.as_ref())));
    }
    results.into_iter().map(|tr| tr.join().unwrap()).max().unwrap().to_string()
}