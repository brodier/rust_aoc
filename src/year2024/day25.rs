use std::str::Lines;



type Design = [u8;5];

fn load_design(lines_itt:&mut Lines<'_>) -> Design {
    let mut design:Design;
    design = [0;5];
    for _ in 0..5 {
        let new_line = lines_itt.next().unwrap();
        for j in 0..5 {
            if new_line.chars().nth(j) == Some('#') {
                    design[j]+=1;
            }
        }
    }
    let _ =lines_itt.next(); // skip last line of design
    design
}

fn load(input:String) -> (Vec<Design>,Vec<Design>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();    
    let mut lines_itt = input.lines().into_iter();
    while let Some(first_line) = lines_itt.next() {
        if first_line == "#####" {
            locks.push(load_design(&mut lines_itt));
        } else if first_line == "....." {
            keys.push(load_design(&mut lines_itt));
        }
    }
    (keys,locks)
}

fn is_fit(key:&Design, lock:&Design) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    return true;
}
pub fn solve(_part:usize, input:String) -> String {
    let (keys,locks) = load(input);
    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if is_fit(key,lock) {
                count += 1;
            }
        }
    }
    count.to_string()
}