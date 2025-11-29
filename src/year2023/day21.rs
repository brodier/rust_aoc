use crate::utils::grid::{Dir, Grid};

pub struct Puzzle {
    nb_step:usize,
    grid:Grid,
    start:(usize,usize)
}

pub fn parse(input:String) -> Puzzle {
    let mut start:(usize,usize) = (0,0);
    for (start_y, l) in input.lines().enumerate() {
        if let Some(start_x) = l.find('S')  {
            start = (start_x,start_y);
        }
    }
    let grid = Grid::build(input);
    let nb_step = if grid.height() < 16 { 6 } else { 64 };
    Puzzle{nb_step, grid, start}
}

pub fn part1(puzzle:&Puzzle) -> String {
    let (w,h) = puzzle.grid.size();
    let mut reached:[bool;17161] = [false; 17161];
    reached[puzzle.start.0 + puzzle.start.1 * w] = true;
    let mut reachable:[bool;17161] = [false; 17161];
    for _ in 0..puzzle.nb_step {
        for x in 0..w {
            for y in 0..h {
                if reached[w*x+y] {
                    for d in Dir::all() {
                        if let Ok(new_pos) = d.get_next((x,y), (w,h)) {
                            let c = puzzle.grid.get(new_pos).unwrap();
                            if c == b'.' || c == b'S' {
                                reachable[new_pos.0+new_pos.1*w] = true;
                            }
                        }
                    }
                }
            }
        }
        reached = reachable;
        reachable = [false; 17161];
    }
    reached.iter().filter(|v| **v).count().to_string()
}

pub fn part2(_:&Puzzle) -> String {
    "2".to_string()
}