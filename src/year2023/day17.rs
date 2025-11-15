use crate::utils::grid::{Dir,Grid};

#[derive(Clone, Debug)]
struct CrucibleScout {
    pos:(usize,usize),
    dir:Dir,
    heat_lost:usize,
}

impl CrucibleScout {
    fn build(pos:(usize,usize), dir:Dir, heat_lost:usize) -> Self {
        CrucibleScout{pos, dir, heat_lost}
    }

    fn step_forward(&self, grid:&Grid) -> Option<CrucibleScout> {
        let next_pos = self.dir.get_next(self.pos, grid.size()).ok()?;
        let heat_lost = self.heat_lost + grid.get_val(next_pos);
        Some(CrucibleScout{
            pos:next_pos,
            dir:self.dir,
            heat_lost
        })
    }

    
    fn find_nexts(&self, min:usize, max:usize, grid:&Grid) -> Option<Vec<CrucibleScout>> {
        let mut scout = self.clone();
        let mut i = 0;
        while i < min {
            scout = scout.step_forward(grid)?;
            i+=1;
        }
        let mut result = Vec::new();
        while i <= max {
            result.push(scout.turn_left());
            result.push(scout.turn_right());
            if let Some(new_scout) = scout.step_forward(grid) {
                scout = new_scout;
                i+=1;
            } else {
                break;
            }
        }
        Some(result)
    }

    fn turn_left(&self) -> CrucibleScout {
        CrucibleScout{
            pos:self.pos,
            dir:self.dir.left(),
            heat_lost:self.heat_lost
        }
    }

    fn turn_right(&self) -> CrucibleScout {
        CrucibleScout{
            pos:self.pos,
            dir:self.dir.right(),
            heat_lost:self.heat_lost
        }
    }

    fn get_weight(&self, grid_size:usize) -> usize {
        (2 * grid_size  - self.pos.0 - self.pos.1)*2 + self.heat_lost
    }

}

impl Grid {
    fn get_val(&self, pos:(usize,usize)) -> usize {
        (self.get(pos).unwrap() - b'0') as usize
    }

    fn get_end(&self) -> (usize, usize) {
        (self.size().0 - 1, self.size().1 - 1)
    }
}

struct HeatLostMap {
    map:Vec<usize>,
    width:usize,
    height:usize,
}

impl HeatLostMap {
    fn build(size:(usize,usize)) -> Self {
        HeatLostMap{map:vec![usize::MAX; size.0 * size.1 * 4 ], width: size.0, height:size.1}
    }

    fn update(&mut self,curcible_scout:&CrucibleScout) -> bool {
        let dir_index = match curcible_scout.dir {
            Dir::UP => 0,
            Dir::RIGHT => 1,
            Dir::DOWN => 2,
            Dir::LEFT => 3,
        };
        let index  = 4 * (curcible_scout.pos.1 * self.width + curcible_scout.pos.0 ) + dir_index;
        // println!("update hlmap with {:?} on index {}", curcible_scout, index);
        if self.map[index] > curcible_scout.heat_lost {
            // println!("map updated");
            self.map[index] = curcible_scout.heat_lost;
            true
        } else {
            // println!("map not updated");
            false
        }
    }
}

pub fn parse(input:String) -> Grid {
    Grid::build(input)
}

pub fn solve(min:usize, max:usize, grid:&Grid) -> String {
    let mut hlmap = HeatLostMap::build(grid.size());
    let mut scouts = Vec::new();
    let fp = (0,0);
    let initial_scout = CrucibleScout::build(fp, Dir::RIGHT, 0);
    let initial_weight = initial_scout.get_weight(grid.size().0);
    scouts.push((initial_scout.turn_right(), initial_weight));
    scouts.push((initial_scout,initial_weight));
    while let Some((scout,_)) = scouts.pop() {
        if scout.pos == grid.get_end() {
            return scout.heat_lost.to_string();
        }
        // println!("-- find nexts -- ");
        if let Some(next_scouts) = scout.find_nexts(min, max, grid) {
            for s in next_scouts.into_iter() {
                if hlmap.update(&s) {
                    let sw = s.get_weight(grid.size().0);
                    scouts.push((s, sw));
                }
            }
            scouts.sort_by(|a,b| b.1.cmp(&a.1));
            println!("nb path to check {}", scouts.len());
        }
    }
    unreachable!("Result not found");
}

pub fn part1(grid:&Grid) -> String {
    solve(1,3, grid)
}

pub fn part2(grid:&Grid) -> String {
    solve(4, 10, grid)
}