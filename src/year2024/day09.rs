use std::{str::{Chars, from_utf8}, usize};

const FREE:i32 = -1;

#[derive(Clone, Debug)]
struct Block {
    id: i32,
    length: usize
}

#[derive(Clone)]
pub struct FileMap {
    blocks: Vec<Vec<Block>>
}

impl FileMap {
    pub fn load(chars:Chars<'_>) -> FileMap {
        let mut read_block = true;
        let mut blocks = Vec::new();
        let mut curr_block = Block{id:0, length:0};
        for c in chars {        
            let s:u8 = c as u8 - b'0';
            if read_block && s==0 {
                panic!("unexpected block size");
            }
            read_block = if read_block { 
                curr_block.length = s as usize;
                false 
            } else { 
                blocks.push(vec![curr_block.clone(), Block{id:FREE, length: s as usize}]);
                curr_block.id += 1;
                true 
            };
        }
        if read_block == false {
            blocks.push(vec![curr_block, Block{id:FREE, length: 0}]);
        }
        FileMap{blocks}
    }

    fn compress(&mut self) {
        let mut block_itt = self.blocks.len() - 1;
        let mut free_itt = 0;
        while free_itt < block_itt {
            let mut remaining_length = self.blocks[block_itt][0].length;
            let file_id =  self.blocks[block_itt][0].id;
            loop {
                let free_len = self.blocks[free_itt].last().unwrap().length;
                if free_itt < block_itt && free_len < remaining_length {
                    remaining_length -= free_len;
                    self.blocks[free_itt].last_mut().unwrap().id = file_id;
                    free_itt+=1;
                } else if free_itt == block_itt {
                    let moved_len = self.blocks[block_itt][0].length - remaining_length;
                    self.blocks[block_itt][0].length=remaining_length;
                    self.blocks[free_itt].insert(1, Block { id: FREE, length: moved_len });
                    break;
                } else { 
                    let remain_free_len = free_len - remaining_length;
                    self.blocks[free_itt].last_mut().unwrap().length=remaining_length;
                    self.blocks[free_itt].last_mut().unwrap().id = file_id;
                    self.blocks[free_itt].push(Block { id: FREE, length: remain_free_len });
                    self.blocks[block_itt][0].id = FREE;
                    break;
                }
            }
            block_itt -= 1;
            // println!("{:?}", self);
        }
    }

    fn compress_without_frags(&mut self)  {
        let mut block_itt = self.blocks.len() - 1;
        let mut left_free_itt = 0;
        while left_free_itt < block_itt {
            let block_len = self.blocks[block_itt][0].length;
            let file_id =  self.blocks[block_itt][0].id;
            let mut free_itt = left_free_itt;
            while free_itt < block_itt {
                let free_len = self.blocks[free_itt].last().unwrap().length;
                if free_len < block_len {
                    free_itt+=1;
                } else {
                    let remain_free_len = free_len - block_len;
                    self.blocks[free_itt].last_mut().unwrap().id = file_id;
                    self.blocks[free_itt].last_mut().unwrap().length = block_len;
                    self.blocks[free_itt].push(Block { id: FREE, length: remain_free_len });
                    self.blocks[block_itt][0].id = FREE;
                    if remain_free_len == 0 && free_itt == left_free_itt {
                        left_free_itt += 1;
                    }
                    break;
                }
            }
            block_itt -= 1;
            // println!("{:?}", self);
        }
    }

    fn get_checksum(&self) -> usize {
        let mut itt = 0;
        let mut checksum=0;
        for span in self.blocks.iter() {
            for block in span {
                if block.id == FREE {
                    itt += block.length;
                    continue;
                }
                for _ in 0..block.length {
                    checksum += itt * block.id as usize;
                    itt += 1;
                }
            }
        }
        checksum
    }
}

impl std::fmt::Debug for FileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut file_map_state = Vec::new();
        for b in self.blocks.iter() {
            for s in b.iter() {
                for _ in 0..s.length {
                    let e = if s.id == FREE { b'.' } else { b'0' + s.id as u8 };
                    file_map_state .push(e);
                }
            }
        }
        write!(f, "{}", from_utf8(&file_map_state.as_slice()).unwrap())
    }
}

pub fn parse(input:String) -> FileMap {
    FileMap::load(input.lines().next().unwrap().chars())
}

pub fn part1(file_map:&FileMap) -> String {
    let mut file_map = file_map.clone();
    file_map.compress();
    file_map.get_checksum().to_string()
}

pub fn part2(file_map:&FileMap) -> String {
    let mut file_map = file_map.clone();
    file_map.compress_without_frags();
    file_map.get_checksum().to_string()
}