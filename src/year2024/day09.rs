use std::str::Chars;

const FREE:i32 = -1;

pub fn load(chars:Chars<'_>) -> Vec<i32> {
    let mut read_block = true;
    let mut blocks = Vec::new();
    let mut id = 0;
    for c in chars {        
        let s:u8 = format!("{}",c).parse().unwrap();
        if read_block && s==0 {
            panic!("unexpected block size");
        }
        for _ in 0..s {
            blocks.push(if read_block { id } else { FREE });
        }
        read_block = if read_block { id+=1; false } else { true };
    }
    blocks
}

fn compress(blocks:&mut [i32]) -> &[i32] {
    let mut free_space_itt:usize = 0;
    let mut data_block_itt = blocks.len() - 1;
    while data_block_itt > 0 && free_space_itt < blocks.len() {
        // 1. find right most data block
        while data_block_itt > 0 && blocks[data_block_itt] == FREE {
            data_block_itt -= 1;
        }
        // 2. find left most free space
        while free_space_itt < blocks.len() && blocks[free_space_itt] != FREE {
            free_space_itt +=1;
        }
        // check for end
        if data_block_itt < free_space_itt {
            break;
        }
        // 3. swap value
        blocks[free_space_itt] = blocks[data_block_itt];
        blocks[data_block_itt] = FREE;
    }
    blocks
}

fn compress_without_frags(blocks:&mut [i32]) -> &[i32]  {
    let mut data_block_itt = blocks.len() - 1;
    let mut done_file_id = FREE;
    while data_block_itt > 0 {
        // 1 find right most data block
        while data_block_itt > 0 && blocks[data_block_itt] == FREE {
            data_block_itt -= 1;
        }
        let end_pos = data_block_itt + 1;
        let file_id = blocks[data_block_itt];
        // 2 compute file size
        while data_block_itt > 0 && blocks[data_block_itt] == file_id {
            data_block_itt -= 1;
        }
        let start_pos = data_block_itt + 1;
        if done_file_id == FREE || file_id < done_file_id {
            done_file_id = file_id;
        } else {
            // move file only once so skip file already move
            continue;
        }
        let size = end_pos - start_pos;
        let mut free_space_itt:usize = 0;
        let mut free_size = 0;
        while free_space_itt < blocks.len() && free_size < size {
            free_size = 0;
            // 3. find left most free space
            while free_space_itt < blocks.len() && blocks[free_space_itt] != FREE {
                free_space_itt +=1;
            }
            // 4. check free space size
            while free_space_itt < blocks.len() && blocks[free_space_itt] == FREE && free_size < size {
                free_space_itt +=1;
                free_size+=1;
            }
        }

        // check for end
        if start_pos < free_space_itt || free_size < size {
            // not enough space found for moving this file continue with the next one
            // println!("Failed to move ({},{})", file_id, size);
            //print_file(&blocks);
            continue;
        }
        // 3. swap file
        free_space_itt -= free_size;
        for d in start_pos..end_pos {
            blocks[free_space_itt] = blocks[d];
            blocks[d] = FREE;
            free_space_itt +=1;
        }
        // print_file(blocks);
    }
    blocks
}

fn get_checksum(blocks:&[i32]) -> usize {
    let mut itt = 0;
    let mut checksum=0;
    while itt < blocks.len() {
        if blocks[itt] != FREE {
            checksum += itt * blocks[itt] as usize;
        }
        itt += 1;
    }
    checksum
}

fn _print_file(file:&[i32]) {
    let mut count = 0;
    let mut curr_data = 0;
    for c in file {
        if *c != curr_data {
            if curr_data == FREE {
                print!("(free;{})",count);
            } else {
                print!("({};{})",curr_data, count);
            }
            curr_data = *c;
            count=0;
        } 
        count += 1;
    }
    if curr_data == FREE {
        print!("(free;{})",count);
    } else {
        print!("({};{})",curr_data, count);
    }
    println!("");
}

use rand::prelude::*;

fn _print_random_puzzle() -> String {
    let mut rng = rand::thread_rng();
    let mut puzzle_blocks = Vec::new();
    for _ in 0..30 {
        let y: u8 = rng.gen(); // generates a float between 0 and 1
        let y = y % 9;
        puzzle_blocks.push((y + b'1') as char);
    }
    let mut puzzle_space = Vec::new();
    for _ in 0..29 {
        let y: u8 = rng.gen(); // generates a float between 0 and 1
        let y = y % 10;
        puzzle_space.push((y + b'0') as char);
    }
    let mut puzzle = Vec::new();
    for i in 0..29 {
        puzzle.push(*puzzle_blocks.get(i).unwrap());
        puzzle.push(*puzzle_space.get(i).unwrap());
    }
    puzzle.push(*puzzle_blocks.get(29).unwrap());
    String::from(puzzle.iter().cloned().collect::<String>())
}

pub fn solve(step:usize, contents:String) -> String {
    let mut data = load(contents.lines().next().unwrap().chars());
    let file = &mut data[..];
    // print_file(file);
    let file = if step == 1 {
        compress(file)
    } else {        
        compress_without_frags(file)
    };
    // print_file(file);
    // println!("Random puzzle : {}", print_random_puzzle());
    get_checksum(file).to_string()
}