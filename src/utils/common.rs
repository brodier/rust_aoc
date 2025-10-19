use std::{fs, io::Error};

use regex::Regex;

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[94m";
pub const WHITE: &str = "\x1b[97m";
pub const HOME: &str = "\x1b[H";
pub const CLEAR: &str = "\x1b[J";

pub enum PuzzleError {
    InvalidParams
}

pub fn load_puzzle(year:usize, day:usize) -> Result<String, Error> {
    fs::read_to_string(format!("puzzle/year{}/day{:02}.txt",year,day))
} 

pub fn parse_usize_old(input:&str) -> Vec<usize> {
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

pub fn _old_parse_i64(input:&str) -> Vec<i64> {
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

pub fn parse_i64(input:&str) -> Vec<i64> {
    let  (last,mut list, is_positive) = input.as_bytes().iter().fold((None, Vec::new(), true), |mut ctx, &b| {
        if b == b'-' {
            if ctx.0.is_none() {
                ctx.2 = false;
            } else {
                unreachable!();
            }
        } else if b.is_ascii_digit() {
            let inc = (b - b'0') as usize;
            if ctx.0.is_none() {
                ctx.0 = Some(inc);
            } else {
                ctx.0 = Some(ctx.0.unwrap() * 10 + inc);
            }
        } else if ctx.0.is_some() {
                let val:i64 = ctx.0.unwrap() as i64;
                if ctx.2 {
                    ctx.1.push(val);
                } else {
                    ctx.1.push(-val);
                }
                ctx.0 = None;
                ctx.2 = true;
        }
        ctx
    });
    if last.is_none() {
        return list;
    } else {
        let val:i64 = last.unwrap() as i64;
        if is_positive {
            list.push(val);
        } else {
            list.push(-val);
        }        
        return list;
    }

}


pub fn parse_usize(input:&str) -> Vec<usize> {
    let  (last,mut list) = input.as_bytes().iter().fold((None, Vec::new()), |mut ctx, &b| {
        if b.is_ascii_digit() {
            let inc = (b - b'0') as usize;
            if ctx.0.is_none() {
                ctx.0 = Some(inc);
            } else {
                ctx.0 = Some(ctx.0.unwrap() * 10 + inc);
            }
        } else if ctx.0.is_some() {
                ctx.1.push(ctx.0.unwrap());
                ctx.0 = None;
        }
        ctx
    });
    if last.is_none() {
        return list;
    } else {
        list.push(last.unwrap());
        return list;
    }

}
