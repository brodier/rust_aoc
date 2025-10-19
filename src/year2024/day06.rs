#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    OBSTACLE,
    EMPTY,
    VISITED
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir6 {
    UP, LEFT, DOWN, RIGHT
}
impl Dir6 {
    fn get_coord(&self, (x,y):(usize,usize)) -> Option<(usize,usize)> {
        match self {
            Dir6::UP => if y == 0 { None } else { Some((x,y-1))},
            Dir6::LEFT=> if x==0  { None } else { Some((x-1,y))},
            Dir6::DOWN=> if y < 129  { Some((x,y+1)) } else { None },
            Dir6::RIGHT=> if x < 129  { Some((x+1,y)) } else { None },
        }
    }

    fn turn_right(&self) -> Dir6 {
        match self {
            Dir6::UP => Dir6::RIGHT,
            Dir6::LEFT=> Dir6::UP,
            Dir6::DOWN=> Dir6::LEFT,
            Dir6::RIGHT=> Dir6::DOWN,
        }
    }
}
struct WalkCtx {
    curr_pos:(usize,usize),
    curr_dir:Dir6,
    next_pos:(usize,usize)
}


fn day6_step1(map:&mut [[CellState;130];130], walk_ctx:&WalkCtx, opt_obstacle:Option<(usize,usize)>) -> (Vec<WalkCtx>,bool) {
    // 1. visit guard's path
    let mut curr_pos = walk_ctx.curr_pos;
    let mut curr_dir = walk_ctx.curr_dir;
    let mut looping = false;
    let mut path:[[(bool,bool,bool,bool);130];130] = [[(false,false,false,false);130];130];
    let mut visited:Vec<WalkCtx> = Vec::new();
    if let Some((x,y)) = opt_obstacle {
        // Put obstacle on selected position
        map[y][x] = CellState::OBSTACLE
    }
    path[curr_pos.1][curr_pos.0] = (true,false,false,false);
    while let Some((x,y)) = curr_dir.get_coord(curr_pos) {
        match map[y][x] {
            CellState::EMPTY => { 
                map[y][x]=CellState::VISITED;
                visited.push(WalkCtx{curr_pos, curr_dir, next_pos:(x,y)});
                curr_pos = (x,y);
            },
            CellState::VISITED => { 
                looping = match curr_dir {
                    Dir6::UP => path[y][x].0,
                    Dir6::LEFT => path[y][x].1,
                    Dir6::DOWN => path[y][x].2,
                    Dir6::RIGHT => path[y][x].3,
                };
                if looping {
                    break;
                }
                curr_pos = (x,y);
            },
            CellState::OBSTACLE => { 
                curr_dir = curr_dir.turn_right();
            }
        }
        match curr_dir {
            Dir6::UP => path[curr_pos.1][curr_pos.0].0 = true,
            Dir6::LEFT => path[curr_pos.1][curr_pos.0].1 = true,
            Dir6::DOWN => path[curr_pos.1][curr_pos.0].2 = true,
            Dir6::RIGHT => path[curr_pos.1][curr_pos.0].3 = true,
        }
    }
    if let Some((x,y)) = opt_obstacle {
        // Restore visited state
        map[y][x] = CellState::VISITED
    }

    (visited,looping)
}

pub fn solve(step:usize,contents:String) -> String {
    // Loading Map
    let mut map:[[CellState;130];130] = [[CellState::EMPTY; 130];130];
    let mut lines = contents.lines().into_iter();
    let mut start_pos = (0,0);
    for y in 0..130 {
        let line = lines.next().unwrap();
        let line = line.as_bytes();
        for x in 0..130 {
            map[y][x] = match line[x] {
                b'#' => CellState::OBSTACLE,
                b'.' => CellState::EMPTY,
                b'^' => { start_pos = (x,y); CellState::VISITED},
                _ => panic!("unexpected value in map ({}, {}) = {}", x, y, line[x])
            };
        }
    }
    let start_ctx =  WalkCtx { curr_pos: start_pos, curr_dir: Dir6::UP, next_pos: (0,0) };
    if step == 1 {
        return (day6_step1(&mut map, &start_ctx, None).0.len() + 1).to_string();
    } else {
        let mut counter = 0;
        let (visited,_) = day6_step1(&mut map, &start_ctx, None);
        for walk_ctx in visited.iter() {
            if day6_step1(&mut map, &start_ctx, Some(walk_ctx.next_pos)).1 {
                counter += 1;
            }
        }
        return counter.to_string();
    }
}
