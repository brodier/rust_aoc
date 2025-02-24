use std::collections::{HashMap, VecDeque};

use crate::utils::common::parse_usize;
use crate::utils::grid::*;

struct Puzzle {
    falling_bytes:Vec<(usize,usize)>,
    board_size:usize,
    limit:usize,
}

impl Puzzle {
    fn build(init_seq: Vec<usize>) -> Puzzle {
        let mut falling_bytes = Vec::new();
        let mut board_size = 0;
        let mut itt = init_seq.iter();
        while let Some(&pos_x) = itt.next() {
            let &pos_y = itt.next().unwrap();
            falling_bytes.push((pos_x,pos_y));
            board_size = if pos_x > board_size || pos_y > board_size {
                if pos_x > pos_y { pos_x } else { pos_y }
            } else { board_size };
        }
        board_size += 1;
        let limit = if board_size == 7 {
            12
        } else {
            1024
        };
        Puzzle{falling_bytes, board_size, limit}
    }

    fn minimal_path(&self) -> Option<usize> {
        let mut map = HashMap::new();
        map.insert((0,0),0);
        let mut to_visit:VecDeque<((usize,usize),usize)> = VecDeque::new();
        to_visit.push_front(((0, 0), 0));
        while to_visit.len() > 0 {
            let (pos,dist) = to_visit.pop_back().unwrap();
            for dir in Dir::all() {
                if let Ok(new_pos) = dir.get_next(pos, (self.board_size,self.board_size)) {
                    if self.falling_bytes[..self.limit].contains(&new_pos) {
                        // println!("reject due to visited or corrupted area");
                        continue;
                    }
                    if map.contains_key(&new_pos) {
                        if *map.get(&new_pos).unwrap() > dist + 1 {
                            panic!("Should never pass here");
                        }
                    } else {
                        map.insert(new_pos, dist+1);
                        to_visit.push_front((new_pos,dist+1));
                    }
                }
            }
        }
        map.remove(&(self.board_size-1,self.board_size-1))
    }

}

pub fn solve(part:usize, input:String) -> String {
    let puzzle = Puzzle::build(parse_usize(&input));
    if part == 1 {
        puzzle.minimal_path().unwrap().to_string()
    } else {
        let mut puzzle = puzzle;
        puzzle.limit = puzzle.falling_bytes.len();
        while puzzle.minimal_path().is_none()  {
            puzzle.limit-=1;
            if puzzle.limit > puzzle.falling_bytes.len() {
                break;
            }
        }
        let result = puzzle.falling_bytes.get(puzzle.limit).unwrap();
        format!("{},{}",result.0,result.1).to_string()
    }
    
}