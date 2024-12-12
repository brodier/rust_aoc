use regex::Regex;

use super::load_puzzle;

pub fn day3(step:usize) -> i32 {
    let contents = load_puzzle(3);
    if step==1 {
        return  day3_step1(contents) as i32;
    } else {
        return day3_step2(contents) as i32;
    }
 }

fn day3_step2(contents:String) -> usize {
    let mut haystack = contents.clone();
    let mut enable:bool = true;
    let mut enable_contents:Vec<String> = Vec::new();
    while haystack.len() > 0 {
        if enable {
            if let Some((head,tail)) = haystack.split_once(r"don't()") {
                enable_contents.push(head.to_string());
                haystack = tail.to_string();
            } else {
                enable_contents.push(haystack.clone());
                haystack.clear();
            }
            enable = false;
        } else {
            if let Some((_,tail)) = haystack.split_once(r"do()") {
                haystack = tail.to_string();
            } else {
                haystack.clear();
            }
            enable = true;
        }
    }
    let mut result = 0;
    for content in enable_contents {
        result += day3_step1(content);
    }
    result
}

fn day3_step1(contents:String) -> usize {
    let mul_a_b_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mul_list: Vec<(usize, usize)> = mul_a_b_re.captures_iter(&contents).map(|caps| {
        let (_, [a, b]) = caps.extract();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect();
    let mut result = 0;
    for (a,b) in mul_list {
        if a < 1000 && b < 1000 {
            result += a * b;
        }
    }
    return result
}