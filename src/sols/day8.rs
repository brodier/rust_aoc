use std::collections::HashMap;

use super::load_puzzle;

const SIZE:usize = 50;

fn save_antinode(antinodes_map:&mut [[bool;SIZE];SIZE], pos:(i32,i32)) {
    println!("trying to save antinode ({},{})", pos.0, pos.1);
    if pos.0 >= 0 && pos.0 < SIZE as i32 && pos.1 >= 0 && pos.1 < SIZE as i32 {
        antinodes_map[pos.1 as usize][pos.0 as usize] = true;
    }
}

pub fn day8(_step:usize) -> usize {
    let contents = load_puzzle(8);
    let mut antinodes_map =  [[false;SIZE];SIZE];
    let mut antennas_map = HashMap::new();
    let lines = contents.lines();
    if contents.lines().count() != SIZE {
        panic!("Unexpected number of lines in puzzle input !");
    }
    let mut y = 0;
    for line in lines {
        let chars = line.chars();
        if line.chars().count() != SIZE {
            panic!("Unexpected number of chars in a line of this puzzle input !");
        }
        let mut x = 0; 
        for c in chars {
            if c != '.' {
                if !antennas_map.contains_key(&c) {
                    antennas_map.insert(c, Vec::new());    
                }
                println!("found antenna {} at ({},{})", c, x,y);
                antennas_map.get_mut(&c).unwrap().push((x,y));
            }
            x+=1;
        }
        y+=1;
    }
    for antenna in antennas_map.keys() {
        println!("processing antenna frequence {}", antenna);
        let antenna_pos = antennas_map.get(antenna).unwrap();
        for i in 0..antenna_pos.len() {
            for j in i+1..antenna_pos.len() {
                let (x1,y1) = antenna_pos.get(i).unwrap();
                let (x2,y2) = antenna_pos.get(j).unwrap();
                println!("Combinning ({},{}) with ({},{})", x1,y1,x2,y2);
                save_antinode(&mut antinodes_map, (2*x1-x2,2*y1-y2));
                save_antinode(&mut antinodes_map, (2*x2-x1,2*y2-y1));
            }
        } 
    }
    let mut count_antinodes = 0;
    for x in 0..SIZE {
        for y in 0..SIZE {
            if antinodes_map[y][x] {
                count_antinodes+=1;
            }
        }
    }
    count_antinodes
}