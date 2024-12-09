use std::{cmp::Ordering, collections::HashMap, fs};

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

pub fn day5(step:usize) -> i32 {
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
