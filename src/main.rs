use std::{fs, ops::Sub};
use regex::Regex;


fn day1() {
    let mut list1:Vec<usize> = Vec::new();
    let mut list2:Vec<usize> = Vec::new();
    let mut read_l1 = true;
    let contents = fs::read_to_string("day1.txt")
    .expect("Should have been able to read the file");
    for val in contents.split_whitespace().into_iter()  {
        let num:usize = val.parse().unwrap();
        if read_l1 {
            list1.push(num);
            read_l1=false;
        } else {
            list2.push(num);
            read_l1=true;
        }
    }
    list1.sort();
    list2.sort();
    let mut sum:usize = 0;
    for i in 0..list1.len() {
        let a = list1.get(i).unwrap();
        let b = list2.get(i).unwrap();
        sum += a.abs_diff(*b);
    }
    println!("far apart distance : {}", sum);
    let mut similarity_score = 0;
    for i in 0..list1.len() {
        let a = list1.get(i).unwrap();
        let mut sim_score_4_a = 0;
        for j in 0..list2.len() {
            let b = list2.get(j).unwrap();
            if a==b {
                sim_score_4_a+=1;
            }
        }
        similarity_score += a * sim_score_4_a;
    }
    println!("similarity_score : {}", similarity_score);
}

#[derive(Debug,PartialEq)]
enum Dir {
    UP,
    DOWN
}

#[derive(Debug)]
struct Report {
    vals:(usize,usize),
    skipped:Option<usize>,
    dir_set:bool    
}

impl Report {
    fn new(val0:usize, val1:usize) -> Report {
        if !Report::is_valid(val0, val1) {
            eprintln!("Invalid value for Report {} {}", val0, val1);
            panic!("Error in Report::new");
        }
        Report{vals:(val0,val1),skipped:None, dir_set:false}
    }

    fn new_fixed(val0:usize, val1:usize) -> Report {
        let mut r = Report::new(val0, val1);
        r.dir_set = true;
        return r;
    }

    fn new_skipped(val0:usize, val1:usize, skipped:usize) -> Report {
        let mut r = Report::new(val0, val1);
        r.dir_set = true;
        r.skipped = Some(skipped);
        r
    }

    fn get_dir(&self) -> Dir {
        return if self.vals.0 > self.vals.1 { Dir::DOWN } else { Dir::UP };
    }

    fn append(&self, new_val:usize) -> Vec<Report> {
    // 33 36 37 39 36 35 : safe in skipping 39 or safe in skipping 37 or is safe
    // Should be unsafe

        let _ = if self.vals.0 > self.vals.1 { Dir::DOWN } else { Dir::UP};
        let mut result = Vec::new();
        if Report::is_safe(self.vals.0, self.vals.1, new_val) {
            let mut new_rep = Report::new_fixed(self.vals.1, new_val);
            new_rep.skipped = self.skipped;
            result.push(new_rep);
        }
        if self.skipped.is_some() {
            return result;
        }

        if Report::is_valid(self.vals.0, new_val) {
            let mut rep = Report::new_skipped(self.vals.0, new_val, self.vals.1);
            if !self.dir_set || self.get_dir() == rep.get_dir() {
                result.push(rep);
            }
        }
        if Report::is_valid(self.vals.1, new_val) {
            let mut rep = Report::new_skipped(self.vals.1, new_val, self.vals.0);
            if !self.dir_set || self.get_dir() == rep.get_dir() {
                result.push(rep);
            }
        }
        // skipping new_val is also a possible choice
        let mut new_rep = Report::new_skipped(self.vals.0, self.vals.1, new_val);
        result.push(new_rep);
        result
    }

    fn is_safe(val0:usize, val1:usize, val2:usize) -> bool {
        if !Report::is_valid(val1, val2) {
            return false;
        }
        if !Report::is_valid(val0, val1) {
            return false;
        }
        (val0 > val1 && val1 > val2) || (val0 < val1 && val1 < val2)
    }

    fn is_valid(prev:usize, val:usize) -> bool {
        if prev == 0 {
            return true;
        }
        let delta = prev.abs_diff(val);
        if delta > 3 || delta == 0 {
            return false;
        }
        return true; 
    }


}


fn day2_valid_line_accepting_one_err(line:&str) -> bool {
    let mut iter = line.split_whitespace().into_iter();
    let val0:usize = iter.next().unwrap().parse().unwrap();
    let val1:usize = iter.next().unwrap().parse().unwrap();
    let mut valid_reports = Vec::new();
    if Report::is_valid(val0,val1) {
        valid_reports.push(Report::new(val0,val1));
    } else {
        let val2 = iter.next().unwrap().parse().unwrap();
        if Report::is_valid(val0, val2) {
            let mut rep = Report::new(val0,val2);
            rep.skipped=Some(val1);
            valid_reports.push(rep);
        }
        if Report::is_valid(val1, val2) {
            let mut rep = Report::new(val1,val2);
            rep.skipped=Some(val0);
            valid_reports.push(rep);
        }
    }

    let mut nb_reports_to_check= valid_reports.len();  
    while nb_reports_to_check > 0 {
        let mut new_reports:Vec<Report> = Vec::new();
        if let Some(next_val) = iter.next() {
            let new_val:usize = next_val.parse().unwrap();
            for vr in &valid_reports {
                for new_report in vr.append(new_val) {
                    new_reports.push(new_report); 
                }
            }
        } else {
            break;
        }
        valid_reports = new_reports;
        nb_reports_to_check = valid_reports.len();
        println!("reports: {:?}", valid_reports);
    }
    let result = valid_reports.len() > 0;
    if !result {
        println!("is unsafe");
    } else {
        for r in &valid_reports {
            if r.skipped.is_none() {
                println!("safe without skipping ");
                break;
            } else {
                print!("safe in skipping {} or ", r.skipped.unwrap());
            }
        }
    }
    return result;
}

fn day2_valid_line(line:&str) -> bool {
    let mut prev =0;
    let mut dir = Dir::UP;
    let mut init_line:bool = false;
    for elem in line.split_whitespace().into_iter() {
        let val:usize = elem.parse().unwrap();
        if val == 0 {
            panic!("Unexpected value 0 for level");
        } 
        if !init_line {
            if prev == 0 {
                prev = val;
                continue;
            } else {
                if prev < val && 4 > (val - prev)  {
                    dir = Dir::UP;
                    prev = val;
                } else if prev > val && 4 > (prev - val) {
                    dir = Dir::DOWN;
                    prev = val;
                } else {
                    // report unsafe read next report
                    println!("is unsafe on init {} {}", prev, val);
                    return false;
                }
                init_line = true;
            }
        } else {
            let delta = prev.abs_diff(val);
            if  delta == 0 || delta > 3 {
                // report unsafe read next report
                println!("is unsafe delta({},{}) > 3 or 0", prev, val);
                return false;
            }

            if !match dir {
                Dir::UP => prev < val,
                Dir::DOWN => prev > val,
            } {
                println!("is unsafe {} {}", prev, val);
                return false;
            }
            prev = val;
        }
    }
    return true;
}

fn day2(step:usize) {
    let contents = fs::read_to_string("day2.txt")
    .expect("Should have been able to read the file");
    let mut safe_counter = 0;
    for line in contents.lines() {
        print!("{} : ", line);
        let safe:bool = if step==1 {day2_valid_line(line)} else {day2_valid_line_accepting_one_err(line)};
        if safe {
            println!("is safe");
            safe_counter += 1;
        }
    }
    println!("Nb safe report : {}", safe_counter);
}

fn day3(step:usize) {
    let contents = fs::read_to_string("day3.txt").expect("Should have been able to read the file");
    if step==1 {
        println!("Result day 3 step 1: {}", day3_step1(contents));
    } else {
        println!("Result day 3 step 2 : {}", day3_step2(contents));
    }
 }

fn day3_step2(contents:String) -> usize {
    let mut haystack = contents.clone();
    let mut enable:bool = true;
    let mut enable_contents:Vec<String> = Vec::new();
    while haystack.len() > 0 {
        if enable {
            if let Some((head,tail)) = haystack.split_once(r"don't()") {
                println!("Don't found");
                enable_contents.push(head.to_string());
                haystack = tail.to_string();
            } else {
                println!("Don't not found");
                enable_contents.push(haystack.clone());
                haystack.clear();
            }
            enable = false;
        } else {
            if let Some((_,tail)) = haystack.split_once(r"do()") {
                println!("Do found");
                haystack = tail.to_string();
            } else {
                println!("Do not found");
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
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
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
    println!("Partial result {}", result);
    return result
}

fn day4_step1(chars:Vec<&[u8]>) -> i32 {
    let search_index:[[(i32,i32);3];8] = [
        [(0,1),(0,2),(0,3)], // DOWN
        [(1,1),(2,2),(3,3)], // DIAG DOWN-RIGHT
        [(1,0),(2,0),(3,0)], // RIGHT
        [(1,-1),(2,-2),(3,-3)], // DIAG RIGHT-UP
        [(0,-1),(0,-2),(0,-3)], //UP
        [(-1,-1),(-2,-2),(-3,-3)], // DIAG UP-LEFT
        [(-1,0),(-2,0),(-3,0)], // LEFT
        [(-1,1),(-2,2),(-3,3)], // DIAG LEFT-DOWN
    ];
    let search_patter = [b'M',b'A',b'S'];

    let line_width = chars.get(0).unwrap().len() as i32;
    let line_height = chars.len() as i32;
    let mut counter = 0;
    for x in 0..line_width as usize {
        for y in 0..line_height as usize {
            let letter = chars.get(y).unwrap().get(x).unwrap();
            if *letter == b'X' {
                'ind: for indexes in search_index {
                    let mut let_itt = 0;
                    for (a,b) in indexes {
                        let new_x = x as i32 + a;
                        let new_y = y as i32 + b;
                        if new_x >=0 && new_y >=0 && new_y < line_height  && new_x < line_width{
                            println!("Get letter for {} {}",new_x, new_y);
                            let next_letter = chars.get(new_y as usize).unwrap().get(new_x as usize).unwrap();
                            if *next_letter != search_patter[let_itt] {
                                continue 'ind;
                            }
                        } else {
                            continue 'ind;
                        }
                        let_itt+=1;
                    }
                    counter+=1;
                }
            }
        }
    }
    counter
}

fn pick_letter(chars:&Vec<&[u8]>, indexes:[(i32,i32);4], a_x_pos:i32, a_y_pos:i32) -> [u8;4] {
    let mut i = 0;
    let mut result:[u8;4] = [0,0,0,0];
    for (a,b) in indexes {
            let new_x = a +a_x_pos;
            let new_y = b + a_y_pos;
            let next_letter = chars.get(new_y as usize).unwrap().get(new_x as usize).unwrap();
            result[i] = *next_letter;
            i+=1;
    }
    return result;
}

fn day4_step2(chars:Vec<&[u8]>) -> i32 {
    let x_indexes:[(i32,i32);4] = [(1,1),(-1,-1),(1,-1),(-1,1)];
    let search_pattern = [
        [b'M',b'S',b'M',b'S'],
        [b'M',b'S',b'S',b'M'],
        [b'S',b'M',b'M',b'S'],
        [b'S',b'M',b'S',b'M']
    ];
    let line_width = chars.get(0).unwrap().len() as i32;
    let line_height = chars.len() as i32;
    let mut counter = 0;
    for x in 1..line_width-1 {
        for y in 1..line_height-1 {
            let letter = chars.get(y as usize).unwrap().get(x as usize ).unwrap();
            if *letter == b'A' {
                let x_letters = pick_letter(&chars, x_indexes, x, y);
                for pattern in search_pattern {
                    if pattern == x_letters {
                        counter+=1;
                    }
                }
            }
        }
    }
    counter
}

fn day4(step:usize) {
    let contents = fs::read_to_string("day4.txt").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut chars = Vec::new();
    for line in lines {
        chars.push(line.as_bytes());
    }    
    if step == 1 {
        println!("day 4 step 1 result : {}", day4_step1(chars));
    } else {
        println!("day 4 step 2 result : {}", day4_step2(chars));
    }

}
fn main() {
    println!("======= Day 1 ==========");
    day1();
    println!("======= Day 2 - Step 1 ==========");
    day2(1);
    println!("======= Day 2 - Step 2 ==========");
    day2(2);
    println!("======= Day 3 - Step 1 ==========");
    day3(1);
    println!("======= Day 3 - Step 2 ==========");
    day3(2);
    println!("======= Day 4 - Step 1 ==========");
    day4(1);
    println!("======= Day 4 - Step 1 ==========");
    day4(2);
}
