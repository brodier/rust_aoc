use std::{cmp::Ordering, collections::HashMap, fs};
use regex::Regex;


fn day1() -> usize {
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
    similarity_score
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
            let rep = Report::new_skipped(self.vals.0, new_val, self.vals.1);
            if !self.dir_set || self.get_dir() == rep.get_dir() {
                result.push(rep);
            }
        }
        if Report::is_valid(self.vals.1, new_val) {
            let rep = Report::new_skipped(self.vals.1, new_val, self.vals.0);
            if !self.dir_set || self.get_dir() == rep.get_dir() {
                result.push(rep);
            }
        }
        // skipping new_val is also a possible choice
        let new_rep = Report::new_skipped(self.vals.0, self.vals.1, new_val);
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
    }
    return valid_reports.len() > 0;
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
                    return false;
                }
                init_line = true;
            }
        } else {
            let delta = prev.abs_diff(val);
            if  delta == 0 || delta > 3 {
                return false;
            }

            if !match dir {
                Dir::UP => prev < val,
                Dir::DOWN => prev > val,
            } {
                return false;
            }
            prev = val;
        }
    }
    return true;
}

fn day2(step:usize) -> i32 {
    let contents = fs::read_to_string("day2.txt")
    .expect("Should have been able to read the file");
    let mut safe_counter = 0;
    for line in contents.lines() {
        let safe:bool = if step==1 {day2_valid_line(line)} else {day2_valid_line_accepting_one_err(line)};
        if safe {
            safe_counter += 1;
        }
    }
    safe_counter
}

fn day3(step:usize) -> i32 {
    let contents = fs::read_to_string("day3.txt").expect("Should have been able to read the file");
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

fn day4(step:usize) -> i32 {
    let contents = fs::read_to_string("day4.txt").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut chars = Vec::new();
    for line in lines {
        chars.push(line.as_bytes());
    }    
    if step == 1 {
        return day4_step1(chars);
    } else {
        return day4_step2(chars);
    }

}

#[derive(Debug)]
struct Constraint(usize,usize);

#[derive(Debug)]
struct Update(Vec<usize>);

impl Update {
    fn get_middle(&self) -> usize {
        if self.0.len() % 2 != 1 {
            eprintln!("expecting odd number of elems in update report");
        }
        return *self.0.get(self.0.len() / 2).unwrap();
    }

    fn check_constraint(&self, c:&Constraint) -> bool {
        let my_slice = self.0.as_slice();
        if my_slice.contains(&c.0) {
            let mut itt = my_slice.split(|num| *num == c.0);
            let a = itt.next().unwrap();
            let _ = itt.next().unwrap();
            if a.contains(&c.1) {
                return false;
            }
        }
        true
    }

    fn is_appliable_constraint(&self, c:&Constraint) -> bool {
        self.0.contains(&c.0) && self.0.contains(&c.1)
    }

    fn update_with_constraint(&mut self, c:&Constraint) {
        let mut itt = self.0.iter();
        let idx_c1 = itt.position(|&e| e == c.1).unwrap();
        let idx_c0 = itt.position(|&e| e == c.0).unwrap() + idx_c1;
        self.0.remove(idx_c1);
        self.0.insert(idx_c0 + 1, c.1);
    }
}

fn day5_step1(constraints:Vec<Constraint>, updates:Vec<Update>) -> i32 {
    let mut result = 0;
    'update: for u in updates.iter() {
        for c in constraints.iter() {
            if !u.check_constraint(c) {
                continue 'update;
            }
        }
        result += u.get_middle();
    }
    result as i32
}


fn day5_step2(constraints:Vec<Constraint>, updates:&mut Vec<Update>) -> i32 {
    let mut updates_to_fix = Vec::new();
    for u in updates.iter_mut() {
        let mut is_valid = true;
        let mut app_c = Vec::new();
        for c in constraints.iter() {
            if u.is_appliable_constraint(c) {
                app_c.push(c);
            }
            if !u.check_constraint(c) {
                is_valid = false;
            }
        }
        if !is_valid {
            updates_to_fix.push((Update(u.0.clone()), app_c));
        }
    }
    
    let mut result = 0;
    for u in updates_to_fix.into_iter() {
        let mut update = Update(u.0.0.clone());
        let constraints_list = u.1;
        // 1. sort applicable constraints 
        // in brief if  a Constraint(a before b) exist then all Constraint(x before a) should be proced before this constraint
        // grouping constraint by first elems in hash map
        let mut c_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for c in constraints_list {
            if !c_map.contains_key(&c.0) {
                c_map.insert(c.0, Vec::new());
            }
            c_map.get_mut(&c.0).unwrap().push(c.1);
        }
        let mut c_seq:Vec<usize>= c_map.keys().map(|k| *k).collect();
        c_seq.sort_by(|s,o| cmp_constraints_group(&c_map, s, o));
        let mut sorted_constraints_list = Vec::new();
        for itt in c_seq {
           for in_list_elem in c_map.get(&itt).unwrap() {
              sorted_constraints_list.push(Constraint(itt,*in_list_elem));
           } 
        }
        // 2. fix update by applying all constraint
        for c in sorted_constraints_list {
            if !update.check_constraint(&c) {
                update.update_with_constraint(&c);
            }
        }
        // 3. compute result
        result += update.get_middle();
    }
    result as i32
}

fn cmp_constraints_group(c_map:&HashMap<usize,Vec<usize>>, elem:&usize, other:&usize) -> Ordering {
    if c_map.get(elem).unwrap().contains(other) {
        Ordering::Less
    } else if c_map.get(other).unwrap().contains(elem) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn day5(step:usize) -> i32 {
    let contents = fs::read_to_string("day5.txt").expect("Should have been able to read the file");
    let mut lines = contents.lines().into_iter();
    let mut load_contraints = true;
    let mut constraints = Vec::new();
    let mut updates = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            load_contraints = false;
            continue;
        } 
        if load_contraints {
            if let Some((a,b)) = line.split_once('|') {
                constraints.push(Constraint(a.parse().unwrap(),b.parse().unwrap()));
            } else {
                panic!("Should never pass here");
            }
        } else {
            let splitted = line.split(',');
            let result:Update = Update(splitted.map(|s| s.parse().unwrap()).collect());
            updates.push(result);
        }
    }
    if step == 1 {
        return day5_step1(constraints, updates);
    } else {
        return day5_step2(constraints, &mut updates);
    }

}

fn main() {
    println!("Result day 1 - step 1): // TODO directly done with google sheet ");
    println!("Result day 1 - step 2): {}", day1());
    println!("Result day 2 - step 1): {}", day2(1));
    println!("Result day 2 - step 2): {}", day2(2));
    println!("Result day 3 step 1: {}", day3(1));
    println!("Result day 3 step 2: {}", day3(2));
    println!("Result day 4 step 1 : {}", day4(1));
    println!("Result day 4 step 2 : {}", day4(2));
    println!("======= Day 5 - Step 1 ==========");
    println!("Result day 5 step 1 : {}", day5(1));
    println!("Result day 5 step 2 : {}", day5(2));
}
