use std::str::Lines;

use crate::utils::common::parse_usize;

#[derive(Debug)]
struct Puzzle {
    seeds:Vec<usize>,
    maps:Vec<Map>,
}

#[derive(Debug)]
struct Map {
    elems:Vec<MapElem>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MergingMap {
    gaps:Vec<usize>,
    offsets:Vec<i64>,
}

#[derive(Debug,Clone)]
struct MapElem {
    offset: i64,
    from: usize,
    len: usize,
}

impl MapElem    {

    fn next_merge(&mut self, map:&MergingMap) -> Option<MapElem> {
        if self.len == 0 {
            return None;
        }
        // return the first gap mapped by map updating from and len
        let search = (self.from as i64 + self.offset) as usize;
        let mut max_end = map.gaps[map.gaps.len()-1];
        if search >= max_end {
            let next = self.clone();
            self.len = 0;
            return Some(next);
        }
        let mut i = map.gaps.len() - 2;
        while search < map.gaps[i] {
            max_end = map.gaps[i];
            i-=1;
        }
        let mut next = self.clone();
        next.offset = self.offset + map.offsets[i];
        if search + self.len <= max_end {
            self.len = 0;
            return Some(next);
        } else {
            next.len = max_end - search;
            if next.len <= 0 {
                panic!("Should not pass here");
            }
            self.from = (max_end as i64 - self.offset) as usize;
            self.len -= next.len;
        }
        Some(next)
    }   
}

impl MergingMap {
    
    fn build_from_seeds(map:&Vec<usize>) -> MergingMap {
        let mut accumulator:Option<usize> = None;
        let mut seed_range = map.iter()
            .map(move |v| {
                if accumulator.is_none() {
                    accumulator = Some(*v);
                    None
                } else {
                    let acc = accumulator.unwrap();
                    accumulator = None;
                    Some((acc, *v))
                }
            })
            .filter(|v| v.is_some())
            .collect::<Vec<Option<(usize,usize)>>>();
        seed_range.sort_by(|a,b| a.unwrap().0.cmp(&b.unwrap().0));
        let mut seed_gaps = Vec::new();
        let mut next:usize = 0;
        seed_gaps.push(0);
        let mut itt = seed_range.iter();
        while let Some(v) = itt.next() {
            let (from,len) = v.unwrap();
            if next == from {
              panic!("unexpected contiguous seed ranges");
            }
            seed_gaps.push(from);
            next = from + len;
            seed_gaps.push(next);
        }
        let mut seed_offsets = Vec::new();
        seed_offsets.resize_with(seed_gaps.len(), Default::default);
        MergingMap { gaps: seed_gaps, offsets: seed_offsets }
    }

    fn build(map:&Map) -> MergingMap {
        let mut gaps = Vec::new();
        let mut offsets = Vec::new();
        let mut curr_gap = 0;
        for e in map.elems.iter() {
            if e.from < curr_gap {
                panic!("Overlapping maps");
            }
            if e.from > curr_gap {
                gaps.push(curr_gap);
                offsets.push(0);
                curr_gap = e.from;
            }
            if e.from == curr_gap {
                gaps.push(curr_gap);
                offsets.push(e.offset);
                curr_gap += e.len;
            }
        }
        let last_elem = map.elems.last().unwrap();
        gaps.push(last_elem.from + last_elem.len);
        offsets.push(0);
        MergingMap{gaps, offsets}
    }

    fn merge(&self, other:&MergingMap) -> MergingMap {
        let mut map = self.as_map();
        let mut new_map_elems = Vec::new();
        for e in map.elems.iter_mut() {
            while let Some(next) = e.next_merge(other) {
                new_map_elems.push(next);
            }
        }
        new_map_elems.sort_by(|a,b| a.from.cmp(&b.from));
        let new_map = Map{elems:new_map_elems};
        MergingMap::build(&new_map)
    }
    


    fn as_map(&self) -> Map {
        let mut elems = Vec::new();
        for i in 0..self.gaps.len()-1 {
            let from = self.gaps[i];
            let len = self.gaps[i+1] - from;
            let offset = self.offsets[i];
            elems.push(MapElem{offset, from, len});
        }
        Map{elems}
    }
}

impl Map {
    fn build(lines: &mut Lines<'_>) -> Option<Map> {
        let line = lines.next(); // skip map name
        if line.is_none() || line.unwrap().is_empty() {
            return None;
        }
        let mut elems= Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let nums = parse_usize(line);
            if nums.len() != 3 {
                eprintln!("Invalid map line: {}", line);
                break;
            }
            let offset = nums[0] as i64 - nums[1] as i64;
            elems.push(MapElem{offset, from:nums[1], len:nums[2]});
        }
        elems.sort_by(|a,b| a.from.cmp(&b.from));
        Some(Map{elems})
    }

    fn map(&self, value:&usize) -> usize {
        // Ineficient linear search
        for elem in self.elems.iter() {
            if *value < elem.from {
                break; // out of map range 
            } else if *value < elem.from + elem.len {
                return (*value as i64 + elem.offset) as usize;
            } else {
                continue;
            }
        }
        *value
    }
}   

impl Puzzle {
    fn build(input: &str) -> Puzzle {
        let mut lines = input.lines().into_iter();
        let seeds = parse_usize(lines.next().unwrap());
        lines.next(); // empty line
        let mut maps:Vec<Map> = Vec::new();
        while let Some(map) = Map::build(&mut lines) {
            maps.push(map);
        }
        Puzzle{seeds, maps}
    }

    fn map(&self, mut value:usize) -> usize {
        for map in self.maps.iter() {
            value = map.map(&value);
        }
        value
    }

    fn eval1(&mut self) -> usize {
        self.seeds.iter().map(|seed| {
            self.map(*seed)
        }).min().unwrap()
    }

    fn eval2(&mut self) -> usize {
        let mut merging_map = MergingMap::build(&self.maps[0]);
        for map in self.maps.iter().skip(1) {
            let other = MergingMap::build(map);
            merging_map = merging_map.merge(&other);
        }
        // build merging map from seeds then merge it with the big merging map
        
        let map = MergingMap::build_from_seeds(&self.seeds);
        merging_map = map.merge(&merging_map);
        // for each gap in merging map, if gap is un seed range add shifted go to list then return min
        let mut min = std::usize::MAX;
        let mut seed_itt = self.seeds.iter();
        while let Some(&from) = seed_itt.next() {
            let &len = seed_itt.next().unwrap();
            let mut seed_elem = MapElem{from: from, offset: 0, len: len};
            while let Some(next_elem) = seed_elem.next_merge(&merging_map) {
                let next_val:usize = (next_elem.from as i64 + next_elem.offset) as usize;
                if next_val < min  {
                    min = next_val;
                }
            }
        }
        min
    }

}
pub fn solve(step: usize, input: String) -> String {
    let mut p = Puzzle::build(&input);
    if step == 1 {
        p.eval1().to_string()
    } else {
        p.eval2().to_string()
    }
}


#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_merge() {
        let initial_map = MergingMap { gaps: vec![0, 50, 98, 100], offsets: vec![0, 2, -48, 0] };
        let next_map = MergingMap { gaps: vec![0, 15, 52, 54], offsets: vec![39, -15, -15, 0] };
        let expected_merged_map = MergingMap { gaps: vec![0, 15, 50, 52, 98, 100], offsets: vec![39, -15, -13, 2, -63, 0] };
        let merged_map = initial_map.merge(&next_map);
        assert_eq!(merged_map, expected_merged_map);
    }
}