
struct Puzzle {
    falling_bytes:Vec<(usize,usize)>,
    board_size:usize,
}

impl Puzzle {
    fn build(init_seq: Vec<usize>) -> Puzzle {
        let mut itt = init_seq.iter();
         
        Puzzle{falling_bytes:Vec::new(), 0}
    }
}

pub fn solve(step:usize, input:String) -> String {
    ""
}