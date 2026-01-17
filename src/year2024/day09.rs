use std::{cmp::Reverse, collections::BinaryHeap, str::{Chars, from_utf8}, usize};

#[derive(Clone, Debug)]
struct Block {
    id: usize,
    length: usize
}

#[derive(Clone, Debug)]
struct Span {
    data: Vec<Block>,
    free: usize,
}

impl Span {

    fn build(id:usize, length:u8) -> Span {
        Span{data:vec![Block{id, length: length as usize}], free:0}
    }

    fn with_free(&mut self, free_length:u8) {
        self.free = free_length as usize;
    }

}
const MAX_SPAN_LEN:usize=9;
const SPAN_NOT_FOUND:(usize,usize) = (MAX_SPAN_LEN,0);


struct FreeSpanIndexor {
    spans_by_length: [BinaryHeap<Reverse<usize>>;MAX_SPAN_LEN]
}

impl FreeSpanIndexor {
    fn new() -> FreeSpanIndexor {
        FreeSpanIndexor { spans_by_length: Default::default() }
    }

    fn push(&mut self, free_idx:usize, free_len:usize) {
        if free_len > 9 {
            unreachable!("Unsupported free len upper 9");
        } else if free_len > 0 {
            self.spans_by_length[free_len-1].push(Reverse(free_idx));
        }
    }

    fn search_idx(&mut self, block_len:usize, idx_limit:usize) -> Option<usize> {
        let mut min = SPAN_NOT_FOUND;
        for i in (block_len-1)..9 {
            if let  Some(&idx) = self.spans_by_length[i].peek() {
                if  idx.0 < idx_limit && ( min.0 == 9 || min.1 > idx.0){
                    min = (i,idx.0);
                }
            }
        }
        if min == SPAN_NOT_FOUND {
            return None
        } else {
            self.spans_by_length[min.0].pop();
            if min.0 + 1 > block_len {
                self.spans_by_length[min.0 - block_len].push(Reverse(min.1));
            }
            Some(min.1)
        }
    }
}

#[derive(Clone)]
pub struct FileMap {
    blocks: Vec<Span>
}

impl FileMap {
    pub fn load(chars:Chars<'_>) -> FileMap {
        let mut blocks = Vec::new();
        let mut curr_block: Option<Span> = None;
        for c in chars {        
            let s:u8 = c as u8 - b'0';
            if curr_block.is_none() && s==0 {
                panic!("unexpected block size");
            }
            if let Some(mut block) = curr_block {
                block.with_free(s);
                blocks.push(block);
                curr_block = None;
            } else {
                curr_block = Some(Span::build(blocks.len(), s))
            }
        }
        if let Some(last_block) = curr_block {
            blocks.push(last_block);
        }
        FileMap{blocks}
    }

    fn compress(&mut self) {
        let mut block_itt = self.blocks.len() - 1;
        let mut free_itt = 0;
        while free_itt < block_itt {
            let mut remaining_length = self.blocks[block_itt].data[0].length;
            let file_id =  self.blocks[block_itt].data[0].id;
            loop {
                let free_len = self.blocks[free_itt].free;
                if free_itt < block_itt && free_len < remaining_length {
                    remaining_length -= free_len;
                    self.blocks[block_itt-1].free += free_len;
                    self.blocks[block_itt].data[0].length -= free_len;
                    self.blocks[free_itt].data.push(Block { id: file_id, length: free_len });
                    self.blocks[free_itt].free = 0;
                    free_itt+=1;
                } else if free_itt == block_itt {
                    let moved_len = self.blocks[block_itt].data[0].length - remaining_length;
                    self.blocks[block_itt].data[0].length=remaining_length;
                    self.blocks[free_itt].free += moved_len;
                    break;
                } else { 
                    self.blocks[block_itt-1].free += remaining_length;
                    self.blocks[block_itt].data.pop();
                    self.blocks[free_itt].free = free_len - remaining_length;
                    self.blocks[free_itt].data.push(Block { id: file_id, length: remaining_length });
                    break;
                }
            }
            block_itt -= 1;
            // println!("{:?} free_itt ({}) block_itt ({})", self, free_itt, block_itt);
        }
    }

    fn compress_without_frags(&mut self)  {
        let mut fsi = FreeSpanIndexor::new();
        for (idx, span) in self.blocks.iter().enumerate() {
            if span.free > 0 {
                fsi.push(idx, span.free);
            }
        }
        let mut itt = self.blocks.len() - 1;
        while itt > 0 {
            let block_len = self.blocks[itt].data[0].length;
            if let Some(free_idx) = fsi.search_idx(block_len, itt) {
                self.blocks[itt-1].free += block_len;
                let moved_data = self.blocks[itt].data.remove(0);
                self.blocks[free_idx].data.push(moved_data);
                self.blocks[free_idx].free -= block_len;
            }
            itt-=1;
        }
    }

    fn get_checksum(&self) -> usize {
        let mut itt = 0;
        let mut checksum=0;
        for span in self.blocks.iter() {
            for block in span.data.iter() {
                let to = block.length + itt - 1;
                let mut from = itt;
                if from > 0  {
                    from -= 1;
                } 
                let sum = (to * (to + 1)) / 2 - ((from * (from + 1))/2);
                //println!("sum from {} to {} = {}", itt, to , sum);
                checksum += sum * block.id as usize;
                itt += block.length;
            }
            itt += span.free;
        }
        checksum
    }
}

impl std::fmt::Debug for FileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut file_map_state = Vec::new();
        for b in self.blocks.iter() {
            for s in b.data.iter() {
                for _ in 0..s.length {
                    file_map_state .push(b'0' + s.id as u8);
                }
            }
            for _ in 0..b.free {
                file_map_state .push(b'.');
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