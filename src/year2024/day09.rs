use std::{collections::BTreeSet, str::{Chars, from_utf8}, usize};

const FREE:i32 = -1;

#[derive(Clone, Debug)]
struct Block {
    id: i32,
    length: usize
}

struct FreeSpanIndexor {
    spans_by_length: [BTreeSet<usize>;9]
}

impl FreeSpanIndexor {
    fn new() -> FreeSpanIndexor {
        FreeSpanIndexor { spans_by_length: Default::default() }
    }

    fn push(&mut self, free_idx:usize, free_len:usize) {
        if free_len > 9 {
            panic!("Unsupported free len upper 9");
        } else if free_len > 0 {
            self.spans_by_length[free_len-1].insert(free_idx);
        }
    }

    fn search_idx(&mut self, block_len:usize, idx_limit:usize) -> Option<usize> {
        let mut candidates = Vec::new();
        for i in (block_len-1)..9 {
            if let  Some(&idx) = self.spans_by_length[i].first() {
                if  idx < idx_limit {
                    candidates.push((idx,i));
                }
            }
        }
        if candidates.is_empty() {
            return None;
        }
        candidates.sort();
        let selected = candidates.first().unwrap();
        self.spans_by_length[selected.1].pop_first();
        let remaining_free_len = (selected.1 + 1) - block_len;
        let free_idx = Some(selected.0);
        if remaining_free_len > 0 {
            self.spans_by_length[remaining_free_len-1].insert(selected.0);
        }
        free_idx
    }
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
        let mut fsi = FreeSpanIndexor::new();
        for (idx, span) in self.blocks.iter().enumerate() {
            if span.len() == 2 {
                fsi.push(idx, span[1].length);
            }
        }
        let mut itt = self.blocks.len() - 1;
        while itt > 0 {
            let cur_block = &mut self.blocks[itt][0];
            if let Some(free_idx) = fsi.search_idx(cur_block.length, itt) {
                let block_id = cur_block.id;
                let block_len = cur_block.length;
                cur_block.id = FREE;
                let free_block = self.blocks[free_idx].last_mut().unwrap();
                free_block.id = block_id;
                if free_block.length < block_len {
                    panic!("Error on search for id {} with len {} found id {} with len {}", itt, block_len, free_idx, free_block.length);
                }
                let free_len = free_block.length - block_len;
                free_block.length = block_len;
                if free_len > 0 {
                    self.blocks[free_idx].push(Block { id: FREE, length: free_len });
                }
            }
            itt-=1;
        }
    }

    fn get_checksum(&self) -> usize {
        let mut itt = 0;
        let mut checksum=0;
        for span in self.blocks.iter() {
            for block in span {
                if block.id != FREE {
                    let to = block.length + itt - 1;
                    let mut from = itt;
                    if from > 0  {
                        from -= 1;
                    } 
                    let sum = (to * (to + 1)) / 2 - ((from * (from + 1))/2);
                    //println!("sum from {} to {} = {}", itt, to , sum);
                    checksum += sum * block.id as usize;
                }
                itt += block.length;
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