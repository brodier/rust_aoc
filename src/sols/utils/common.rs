use std::fs;


fn load_puzzle(puzzle_id:usize) -> String {
    fs::read_to_string(format!("puzzle/day{}.txt",puzzle_id)).expect("Should have been able to read the file")
} 