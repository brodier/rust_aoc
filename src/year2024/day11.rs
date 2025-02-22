use crate::utils::common::load_puzzle;

use std::collections::HashMap;

struct Corridor {
    stones:HashMap<usize,usize>
}

impl Corridor {
    fn build(stones:Vec<usize>) -> Corridor {
        let mut corridor = Corridor{stones:HashMap::new()};
        for stone in stones {
            corridor.stones.insert(stone, 1);
        }
        corridor
    }

    fn add(&mut self, stone:usize, nb:usize) {
        if self.stones.contains_key(&stone) {
            let counter = self.stones.get_mut(&stone).unwrap();
            *counter += nb;
        } else {
            self.stones.insert(stone, nb);
        }
    }

    fn iterate(&mut self) {
        let mut corridor = Corridor{stones:HashMap::new()};
        for (stone,nb_occ) in self.stones.iter() {
            let (new_stone, opt_new_other_stone) = process_one_stone_one_step(*stone);
            corridor.add(new_stone, *nb_occ);
            if opt_new_other_stone.is_some() {
                corridor.add(opt_new_other_stone.unwrap(), *nb_occ);
            }
        }
        self.stones = corridor.stones;
    }

    fn count(&self) -> usize {
        let mut result = 0;
        for (_,v) in self.stones.iter() {
            result += v;
        }
        result
    }
    
}
fn process_one_stone_one_step(stone:usize) -> (usize, Option<usize>) {
    if stone == 0 {
        return (1,None);
    } 
    let stone_str = format!("{}", stone);
    if stone_str.len() % 2 == 0 {
        let middle = stone_str.len()/2;
        return (stone_str[..middle].parse().unwrap(), Some(stone_str[middle..].parse().unwrap()));
    }
    (stone*2024,None)
}


pub fn day11(step:usize) -> usize {
    let content = load_puzzle(11);
    let stones:Vec<usize> = content.split(" ").map(|v| v.parse::<usize>().unwrap()).collect();
    let nb_iter = if step == 1 { 25 } else  { 75 };
    let mut corridor = Corridor::build(stones);
    for _ in 0..nb_iter {
        corridor.iterate();
    }
    corridor.count()
}