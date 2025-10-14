use std::{fs, io::Error};

use regex::Regex;

pub enum PuzzleError {
    InvalidParams
}

pub fn load_puzzle(year:usize, day:usize) -> Result<String, Error> {
    fs::read_to_string(format!("puzzle/year{}/day{:02}.txt",year,day))
} 

pub fn parse_usize(input:&str) -> Vec<usize> {
    let in_re = Regex::new(r"\d+").unwrap();
    let mut iter = in_re.captures_iter(input);
    let mut result = Vec::new();
    loop {
        let number = iter.next();
        if number.is_some() {
            result.push(number.unwrap()[0].parse::<usize>().unwrap());
        } else {
            break;
        };
    }
    result
}

pub fn parse_i64(input:&str) -> Vec<i64> {
    let in_re = Regex::new(r"[+-]?\d+").unwrap();
    let mut iter = in_re.captures_iter(input);
    let mut result = Vec::new();
    loop {
        let number = iter.next();
        if number.is_some() {
            result.push(number.unwrap()[0].parse::<i64>().unwrap());
        } else {
            break;
        };
    }
    result
}