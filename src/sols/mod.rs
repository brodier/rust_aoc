use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

fn load_puzzle(puzzle_id:usize) -> String {
    fs::read_to_string(format!("puzzle/day{}.txt",puzzle_id)).expect("Should have been able to read the file")
} 