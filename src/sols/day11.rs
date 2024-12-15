use super::load_puzzle;
use std::collections::HashMap;
use std::io;
use std::io::Write;

struct StonePredictor {
    value:usize,
    on_iter:usize,
}

impl StonePredictor {
    fn eval(&self) -> usize {
        let mut stones = vec![self.value];
        for _ in 0..self.on_iter {
            io::stdout().flush().unwrap();
            let mut itt = 0;
            while itt < stones.len() {
                let (new_stone, opt_complementary_stone) = process_one_stone_one_step(stones[itt]);
                stones[itt] = new_stone;
                if opt_complementary_stone.is_some() {
                    itt += 1;
                    stones.insert(itt, opt_complementary_stone.unwrap());
                }
                itt += 1;
            }
        }
        stones.len()
    }
}

///
/// iter(0,n) => iter(1,n-1) => iter(2024, n-2) => iter(20, n-3) + iter(24,n-3) => 2 * iter(2,n-4)+ iter(4,n-4) + iter(0,n-4)
/// 1
/// 2024
/// 20 24
/// 2 0 2 4
/// 4048 1 4048 8096
/// 40 48 2024 40 48 80 96
/// 4 0 4 8 20 24 4 0 4 8 8 0 9 6
/// 

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
    let mut result = 0;
    for stone in stones {
        let stone_predict = StonePredictor{value: stone, on_iter: nb_iter};
        result += stone_predict.eval();
    }
    result
}