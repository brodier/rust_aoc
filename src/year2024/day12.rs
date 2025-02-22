use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Region {
    area:usize,
    fence_lenght:usize,
    promoted_fence:usize,
}

#[derive(Debug)]
struct ExtendFlags {
    left_up_eq:bool,
    up_eq:bool,
    right_up_eq:bool,
    left_eq:bool,
}

impl Region {
    fn build() -> Region {
        Region{area:1,fence_lenght:4, promoted_fence:4}
    }

    fn extends(&mut self) {
        self.area += 1;
        self.fence_lenght += 2;
    }

    fn extends_promoted(&mut self, flags:ExtendFlags) {
        // println!("extends : {:?}, {:?}", self, flags);
        if !flags.left_eq && !flags.up_eq {
            return;
        }
        if flags.left_eq && !flags.up_eq  {
            if flags.left_up_eq {
                self.promoted_fence += 2;
            }
            return;
        }
        if !flags.left_eq && flags.up_eq {
            if flags.left_up_eq && flags.right_up_eq {
                self.promoted_fence += 4;
            } else if flags.left_up_eq != flags.right_up_eq {
                self.promoted_fence += 2
            } 
            return;
        }
        if !flags.right_up_eq {
            self.promoted_fence -= 2;
        }
    }

    fn merge(&mut self, other:&Region) {
        self.area += other.area;
        self.fence_lenght += other.fence_lenght - 2;
        self.promoted_fence += other.promoted_fence;
    }
}

struct RegionMap {
    regions:HashMap<usize, Region>,
    width:usize,
    col:usize,
    mem:VecDeque<(usize,u8)>,
    rg_idx:usize
}

impl RegionMap {
    fn build(width:usize) -> RegionMap {
        return RegionMap{regions:HashMap::new(), width, col: 0, mem:VecDeque::with_capacity(width), rg_idx:0};
    }

    fn update_cache(&mut self, region_id:usize, plot:u8) {
        if self.mem.len() == self.width + 1 {
            _ = self.mem.pop_front();
       }
       self.mem.push_back((region_id,plot));
    } 

    fn add_new_reg(&mut self, plot:u8) {
        self.update_cache(self.rg_idx, plot);
        self.regions.insert(self.rg_idx, Region::build());
        self.rg_idx+=1;
    }

    fn extend_right(&mut self, left_idx:usize) {
        let (rg_id,rg_plot) = self.mem.get(left_idx).unwrap().clone(); 
        self.update_cache(rg_id, rg_plot);
        self.regions.get_mut(&rg_id).unwrap().extends();
    }

    fn extend_down(&mut self, up_idx:usize) {
        let (rg_id,rg_plot) = self.mem.get(up_idx).unwrap().clone(); 
        self.update_cache(rg_id, rg_plot);
        self.regions.get_mut(&rg_id).unwrap().extends();
    }


    fn update_mem_after_merge_region(&mut self, from:usize,to:usize) {
        for mem_cell in self.mem.iter_mut() {
            if mem_cell.0 == from {
                mem_cell.0 = to;
            }
        }
    }

    fn init_left_n_up(&self, new_line:bool) -> (Option<usize>,Option<usize>) {
        let cache_len = self.mem.len();
        if new_line {
            if cache_len < self.width {
                (None,None)
            } else if cache_len == self.width {
                (None,Some(0))
            } else {
                (None, Some(1))
            }
        } else {
            if cache_len < self.width {
                (Some(cache_len - 1), None)
            } else {
                (Some(cache_len - 1), Some(1))
            }
        }
    }

    fn update(&mut self, plot:u8) {
        // print!("{} ", plot as char);
        let new_line = self.col == 0;
        let end_line = self.col == self.width - 1;
        self.col += 1;
        if self.col == self.width {
            self.col = 0;
        }
        let (left, up) = self.init_left_n_up(new_line);
        let left_eq = left.is_some() && self.mem.get(left.unwrap()).unwrap().1 == plot;
        let up_eq = up.is_some() && self.mem.get(up.unwrap()).unwrap().1 == plot;
        let left_up_eq = new_line == false && self.mem.len() == self.width+1 && self.mem.get(0).unwrap().1 == plot;
        let right_up_eq = 
            if end_line {
                false
            } else if new_line && self.mem.len() == self.width {
             self.mem.get(1).unwrap().1 == plot
            } else if self.mem.len() == self.width + 1 {
                self.mem.get(2).unwrap().1 == plot
            } else {
                false
            };
        // println!("log cache : {:?}", self.mem);
        // println!("log {:?} {:?} {:?} {:?} {:?} {:?}", left, up, left_up_eq, up_eq, right_up_eq, left_eq);
        if !left_eq && !up_eq {
            // println!("new region");
            self.add_new_reg(plot);
            return;
        } else if left_eq {
            // print!("extends right ");
            let opt_up_rg_id = if up.is_some() { Some(self.mem.get(up.unwrap()).unwrap().0) } else { None };
            let left_rg_id = self.mem.get(left.unwrap()).unwrap().0;
            self.extend_right(left.unwrap());
            if up_eq {
                let up_rg_id = opt_up_rg_id.unwrap();
                // print!(" cmp {:?} with {} ", opt_up_rg_id, left_rg_id);
                if opt_up_rg_id == Some(left_rg_id) {
                    // println!(" => update fence (connect left and up)");
                    self.regions.get_mut(&up_rg_id).unwrap().fence_lenght -= 2;
                } else {
                    // merge up and left region
                    // println!(" => merge up and left {}, {}", up_rg_id, left_rg_id);
                    let up_region = self.regions.remove(&up_rg_id).unwrap();
                    let left_region = self.regions.get_mut(&left_rg_id).unwrap();
                    left_region.merge(&up_region);
                    self.update_mem_after_merge_region(up_rg_id, left_rg_id);
                }
            } else {
                // up is a different plot/region
                // println!("");
            }
        } else if up_eq {
            // println!("extends down");
            self.extend_down(up.unwrap());
        } else {
            // panic!("Should never pass here");
        }
        let update_region_id = self.mem.back().unwrap().0;
        self.regions.get_mut(&update_region_id).unwrap().extends_promoted(ExtendFlags{left_up_eq,up_eq,right_up_eq,left_eq});
    }

    fn compute_result(&self) -> usize {
        let mut result = 0;
        for (_,v) in self.regions.iter() {
            // print!("{} * {} + ", v.area, v.fence_lenght);
            // io::stdout().flush().unwrap();
            result += v.area * v.fence_lenght;
        }
        // println!("");
        result
    }

    fn compute_result_step2(&self) -> usize {
        let mut result = 0;
        let mut append = false;
        for (_,v) in self.regions.iter() {
            if append {
                // print!(" + ");
            }
            // print!("{} * {} ({})", v.area, v.promoted_fence, v.plot);
            // io::stdout().flush().unwrap();
            result += v.area * v.promoted_fence;
            append = true;
        }
        // println!("");
        result
    }

}

pub fn solve(step:usize,content:String) -> usize {
    // NOTE : expecting square puzzle
    let mut regions = RegionMap::build(content.lines().count());
    for line in content.lines() {
        for plot in line.bytes() {
            regions.update(plot);
            //regions.compute_result_step2();
        }
    }

    if step == 1 {
        regions.compute_result()
    } else {
        regions.compute_result_step2()
    }
    
}