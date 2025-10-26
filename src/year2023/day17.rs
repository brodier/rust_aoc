use crate::utils::grid::{Dir,Grid};

struct CrucibleScout {
    pos:(usize,usize),
    dir:Dir,
    heat_lost:usize,
    remaining_step_in_current_dir:usize,
}

impl CrucibleScout {
    fn build(pos:(usize,usize), dir:Dir, heat_lost:usize) -> Self {
        CrucibleScout{pos, dir, heat_lost, remaining_step_in_current_dir:3}
    }

    fn step_forward(&self, grid:&Grid) -> Option<CrucibleScout> {
        if self.remaining_step_in_current_dir == 0 {
            return None;
        }
        let next_pos = self.dir.get_next(self.pos, grid.size()).ok()?;
        let heat_lost = self.heat_lost + grid.get_val(next_pos);
        Some(CrucibleScout{
            pos:next_pos,
            dir:self.dir,
            heat_lost,
            remaining_step_in_current_dir:self.remaining_step_in_current_dir -1,
        })
    }

    fn turn_left(&self) -> CrucibleScout {
        CrucibleScout{
            pos:self.pos,
            dir:self.dir.left(),
            heat_lost:self.heat_lost,
            remaining_step_in_current_dir:3,
        }
    }

    fn turn_right(&self) -> CrucibleScout {
        CrucibleScout{
            pos:self.pos,
            dir:self.dir.right(),
            heat_lost:self.heat_lost,
            remaining_step_in_current_dir:3,
        }
    }

    fn compare(&self, other:&CrucibleScout) -> std::cmp::Ordering {
        self.heat_lost.cmp(&other.heat_lost)
    }

}

impl Grid {
    fn get_val(&self, pos:(usize,usize)) -> usize {
        (self.get(pos).unwrap() - b'0') as usize
    }
}

struct HeatLostMap {
    map:Vec<usize>,
    width:usize,
    height:usize,
}

impl HeatLostMap {
    fn build(size:(usize,usize)) -> Self {
        HeatLostMap{map:vec![usize::MAX; size.0 * size.1 * 4 * 4], width: size.0, height:size.1}
    }

    fn update(&mut self,curcible_scout:&CrucibleScout) -> bool {
        let dir_index = match curcible_scout.dir {
            Dir::UP => 0,
            Dir::RIGHT => 1,
            Dir::DOWN => 2,
            Dir::LEFT => 3,
        };
        let index  = 16 * (curcible_scout.pos.1 * self.width + curcible_scout.pos.0 ) + 4 * dir_index +curcible_scout.remaining_step_in_current_dir;
        if self.map[index] > curcible_scout.heat_lost {
            self.map[index] = curcible_scout.heat_lost;
            true
        } else {
            false
        }
    }

    fn get_best_score(&self) -> usize {
        let mut best = usize::MAX;
        for dir_index in 0..4 {
            for step_index in 0..4 {
                let index = 16 * ( self.height * self.width -1 ) + 4 * dir_index + step_index;
                if self.map[index] < best {
                    best = self.map[index];
                }
            }
        }
        best
    }
}
pub fn parse(input:String) -> Grid {
    Grid::build(input)
}

pub fn part1(grid:&Grid) -> String {

    let mut hlmap = HeatLostMap::build(grid.size());
    let mut scouts = Vec::new();
    scouts.push(CrucibleScout::build((0,0), Dir::RIGHT, 0));
    scouts.push(CrucibleScout::build((0,0), Dir::DOWN, 0));
    while let Some(scout) = scouts.pop() {
        if let Some(next_scout) = scout.step_forward(grid) {
            if hlmap.update(&next_scout) {
                scouts.push(next_scout.turn_left());
                scouts.push(next_scout.turn_right());
                scouts.push(next_scout);
                scouts.sort_by(|a,b| b.compare(a))
            }
        }
    }
    hlmap.get_best_score().to_string()
}

pub fn part2(_:&Grid) -> String {
    "2".to_string()
}