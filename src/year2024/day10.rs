use std::collections::HashMap;

#[derive(Debug)]
struct Puzzle {
    start_pos:Vec<(usize,usize)>,
    map:Vec<Vec<u8>>,
    size:usize,
}


fn step1(puzzle:&Puzzle) -> usize {
    let mut from = HashMap::new();
    for start in puzzle.start_pos.iter().map(|v| *v) {
        from.insert(start, vec![start]);
    }
    for _ in 0..9 {
        let mut reachable = HashMap::new();
        'previous: for previous in from.keys().map(|v| *v) {
            let neighboors = puzzle.get_neighboors(previous);
            if neighboors.is_empty() {
                continue 'previous;
            }
            for neighboor in neighboors {
                if !reachable.contains_key(&neighboor) {
                    reachable.insert(neighboor, Vec::new());
                }
                let neighboor_reach_by = reachable.get_mut(&neighboor).unwrap();
                for previous_reach_by in from.get(&previous).unwrap() {
                    if !neighboor_reach_by.contains(previous_reach_by) {
                        neighboor_reach_by.push(*previous_reach_by);
                    }
                }
            }
        }
        from = reachable;
    }
    let mut count = 0;
    for sources in from.values() {
        count += sources.len();
    }
    count
}

fn step2(puzzle:&Puzzle) -> usize {
    let mut from = HashMap::new();
    for start in puzzle.start_pos.iter().map(|v| *v) {
        from.insert(start, vec![start]);
    }
    for _ in 0..9 {
        let mut reachable = HashMap::new();
        'previous: for previous in from.keys().map(|v| *v) {
            let neighboors = puzzle.get_neighboors(previous);
            if neighboors.is_empty() {
                continue 'previous;
            }
            for neighboor in neighboors {
                if !reachable.contains_key(&neighboor) {
                    reachable.insert(neighboor, Vec::new());
                }
                let neighboor_reach_by = reachable.get_mut(&neighboor).unwrap();
                for previous_reach_by in from.get(&previous).unwrap() {
                    neighboor_reach_by.push(*previous_reach_by);
                }
            }
        }
        from = reachable;
    }
    let mut count = 0;
    for sources in from.values() {
        count += sources.len();
    }
    count
}

impl Puzzle {
    fn build(contents:String) -> Puzzle {
        let mut map = Vec::new();
        let mut start_pos = Vec::new();
        let size = contents.lines().count();
        let mut line_number = 0;
        for line in contents.lines() {
            let mut row_number = 0;
            let mut row_content = Vec::new();
            for b in line.bytes().map(|b| b - b'0') {
                if b == 0 {
                    start_pos.push((row_number,line_number));
                }
                row_content.push(b);
                row_number += 1;
            }
            map.push(row_content);
            line_number+=1;
        }
        Puzzle{start_pos,map,size}
    }
    fn get_neighboors(&self, (x,y):(usize,usize)) -> Vec<(usize,usize)> {
        let mut neighboors = Vec::new();
        let next_floor = self.map[y][x] + 1;
        if x > 0 { // handle left
            if self.map[y][x-1] == next_floor {
                neighboors.push((x-1,y));
            }
        }
        if y > 0 { //handle up
            if self.map[y-1][x] == next_floor {
                neighboors.push((x,y-1));
            }
        }
        if x + 1 < self.size { //handle right
            if self.map[y][x+1] == next_floor {
                neighboors.push((x+1,y));
            }
        }
        if y + 1 < self.size { //handle up
            if self.map[y+1][x] == next_floor {
                neighboors.push((x,y+1));
            }
        }
        neighboors
    }
}
pub fn solve(step:usize,contents:String) -> usize {
    if step == 1 {
        step1(&Puzzle::build(contents))
    } else {
        step2(&Puzzle::build(contents))
    }
}