use std::collections::HashMap;
use std::fs;

#[derive(Debug,Clone)]
enum BoolEntry<'a> {
    FixBool(bool),
    AndBool(&'a str,&'a str),
    OrBool(&'a str,&'a str),
    XorBool(&'a str,&'a str),
}

impl <'a>BoolEntry<'a> {
    
    fn build(line:&'a str) -> (&'a str,BoolEntry<'a>) {
        if line.contains(":") {
            if let Some((name,value)) = line.split_once(": ") {
                return (name, BoolEntry::FixBool(value == "1"));
            }
        }
        if line.contains("->") {
            if let Some((exp,name)) = line.split_once(" -> ") {
                if line.contains("AND") {
                    if let Some((left,right)) = exp.split_once(" AND ") {
                        return (name, BoolEntry::AndBool(left,right));
                    }
                } else if line.contains("XOR") {
                    if let Some((left,right)) = exp.split_once(" XOR ") {
                        return (name, BoolEntry::XorBool(left,right));
                    }
                } else if line.contains("OR") {
                    if let Some((left,right)) = exp.split_once(" OR ") {
                        return (name, BoolEntry::OrBool(left,right));
                    }
                }
            }
        }
        panic!("Failed to build BoolEntry with this line args {}", line);
    }
    
    fn eval(&self, map:&HashMap<&str,BoolEntry>) -> bool {
        match self {        
            Self::FixBool(value) => *value,
            Self::AndBool(left, right) => map.get(*left).unwrap().eval(map) && map.get(*right).unwrap().eval(map),
            Self::OrBool(left, right) => map.get(*left).unwrap().eval(map) || map.get(*right).unwrap().eval(map),
            Self::XorBool(left, right) => map.get(*left).unwrap().eval(map) ^ map.get(*right).unwrap().eval(map),
        }
    }
}


fn solve1(map:&HashMap<&str,BoolEntry>) -> u64 {
    let mut z:u64 = 0;
    for key in map.keys() {
        if key.starts_with("z") {
            if map.get(key).unwrap().eval(&map) {
                let idx:u8 = String::from_utf8(key.as_bytes()[1..].to_vec()).unwrap().parse().unwrap();
                z |= 1 << idx;
            }
        }
    }
    z
}

struct Puzzle<'a> {
    map:HashMap<&'a str,BoolEntry<'a>>,
    xor_map:HashMap<&'a str,&'a str>,
    and_map:HashMap<&'a str,&'a str>,
    or_map:HashMap<&'a str,&'a str>,
}

impl <'a>Puzzle<'a> {
    
    fn check(&self) -> String {
        let mut try_exchange = Vec::new();
        for i in 1..44 {
            let x = format!("x{:02}", i);
            let x = x.as_str();
            let exp_z = format!("z{:02}", i); 
            let &exp_z = self.map.keys().find(|key| **key == exp_z.as_str()).unwrap();
            let &t = self.xor_map.get(x).unwrap();
            let &s = self.and_map.get(x).unwrap();
            let u = self.and_map.get(&t);
            if u.is_none() {
                // println!("No and op with {} target of {} xor y..", t, x);
            } else {
                let u = u.unwrap();
                let r = self.or_map.get(u);
                if r.is_none() {
                   //  println!("No or op with {} target of {} with and op that is target of {} xor y..", u, t, x);
                }
            }
            let z = self.xor_map.get(&t);
            if u.is_none() && z.is_none() {
                // println!("Neigther xor op with {} target of {} xor y.. try to reverse {} {}", t, x, s, t);
                try_exchange.push(s);
                try_exchange.push(t);
            } else if z.is_none() {
                // println!("No xor op with {} target of {} xor y..", t, x);
            } else {
                let &z = z.unwrap();
                if z != exp_z {
                    // println!("unexpected value of xor on {} target of {} xor y. found {} expecting {}.", t, x, z, exp_z);
                    try_exchange.push(z);
                    try_exchange.push(exp_z);
                }
            }
        }
        try_exchange.sort();
        try_exchange.join(",").to_string()
    }

}

pub fn solve(part:usize, input:String) -> String {
    let mut map = HashMap::new();
    let mut xor_map = HashMap::new();
    let mut and_map = HashMap::new();
    let mut or_map = HashMap::new();

    for line in input.lines() {
        if line.len() > 0 {
            let (key,val) = BoolEntry::build(line);
            map.insert(key, val.clone());            
            match val {
                BoolEntry::AndBool(left,right ) => { 
                    and_map.insert(left, key);
                    and_map.insert(right, key);
                },
                BoolEntry::XorBool(left,right ) => { 
                    xor_map.insert(left, key);
                    xor_map.insert(right, key);
                },
                BoolEntry::OrBool(left , right ) => { 
                    or_map.insert(left, key);
                    or_map.insert(right, key);
                },  
                _ => {}              
            }
        }
    }
    if let Ok(s) = fs::read_to_string("path.data") {
        for token in s.split(" ") {
            println!("{}", token);
        }
    } else {
        // println!("failed to read string");
    }
    if part == 1 {
        return solve1(&map).to_string();
    } else {
        let p = Puzzle{map, xor_map, and_map, or_map};
        return p.check();
    }
}


pub fn parse(input:String) -> String {
    input
}

pub fn part1(input:&String) -> String {
    solve(1, input.clone())
}

pub fn part2(input:&String) -> String {
    solve(2, input.clone())
}