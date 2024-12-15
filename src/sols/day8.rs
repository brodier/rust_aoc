use std::collections::HashMap;

use super::load_puzzle;

const SIZE:usize = 50;

fn mark_step1(antinodes_map:&mut [[bool;SIZE];SIZE],vec:(i32,i32),mut pos:(i32,i32)) {
    pos = (pos.0 + vec.0, pos.1+vec.1);
    if pos.0 >= 0 && pos.0 < SIZE as i32 && pos.1 >= 0 && pos.1 < SIZE as i32 {
        antinodes_map[pos.1 as usize][pos.0 as usize] = true;
    }
    let vec = (-vec.0,-vec.1);
    pos = (pos.0 + vec.0, pos.1+vec.1);
    pos = (pos.0 + vec.0, pos.1+vec.1);
    pos = (pos.0 + vec.0, pos.1+vec.1);
    if pos.0 >= 0 && pos.0 < SIZE as i32 && pos.1 >= 0 && pos.1 < SIZE as i32 {
        antinodes_map[pos.1 as usize][pos.0 as usize] = true;
    }
}

fn mark_step2(antinodes_map:&mut [[bool;SIZE];SIZE],vec:(i32,i32),mut pos:(i32,i32)) {
    let from = pos.clone();
    while pos.0 >= 0 && pos.0 < SIZE as i32 && pos.1 >= 0 && pos.1 < SIZE as i32 {
        antinodes_map[pos.1 as usize][pos.0 as usize] = true;
        pos = (pos.0 + vec.0, pos.1+vec.1)
    }
    let vec = (-vec.0,-vec.1);
    pos = (from.0 + vec.0,from.1+vec.1);
    while pos.0 >= 0 && pos.0 < SIZE as i32 && pos.1 >= 0 && pos.1 < SIZE as i32 {
        antinodes_map[pos.1 as usize][pos.0 as usize] = true;
        pos = (pos.0 + vec.0, pos.1+vec.1)
    }
}

fn mark_antinode(antinodes_map:&mut [[bool;SIZE];SIZE], (x1,y1):(i32,i32), (x2,y2):(i32,i32), step:usize) {
    let vec = ((x2-x1),(y2-y1));    
    let pos = (x2,y2);
    if step == 1 {
        mark_step1(antinodes_map, vec, pos);
    } else {
        mark_step2(antinodes_map, vec, pos);
    }
}

pub fn day8(step:usize) -> usize {
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
                antennas_map.get_mut(&c).unwrap().push((x,y));
            }
            x+=1;
        }
        y+=1;
    }
    for antenna in antennas_map.keys() {
        let antenna_pos = antennas_map.get(antenna).unwrap();
        for i in 0..antenna_pos.len() {
            for j in i+1..antenna_pos.len() {
                let (x1,y1) = antenna_pos.get(i).unwrap();
                let (x2,y2) = antenna_pos.get(j).unwrap();
                mark_antinode(&mut antinodes_map, (*x1,*y1),(*x2,*y2), step);
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