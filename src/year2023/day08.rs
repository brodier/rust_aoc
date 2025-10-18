use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    walk: Vec<bool>,
    map:HashMap<&'a str, (&'a str,&'a str)>
}


impl Puzzle<'_> {
    fn build<'a>(step: usize, input: &'a str) -> Puzzle<'a> {
        let mut lines = input.lines();
        let walk = lines.next().unwrap();
        let walk = walk.bytes().map(|c|
        match c {
            b'L' => true,
            b'R' => false,
            _ => panic!("Invalid Value")
        }).collect();
        lines.next(); // skip empty line;
        let re = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
        let mut map = HashMap::new();
        while let Some(line) = lines.next() {
            let (_, [key, left, right])  = re.captures(line).unwrap().extract();
            map.insert(key, (left, right));
        }
        Puzzle{ step, walk, map}
    }
    fn solve1(&self) -> String {
        let mut pos = "AAA";
        let mut counter = 0;
        while pos != "ZZZ" {
            pos =  self.make_step(counter, pos);
            counter+=1;
        }
        counter.to_string()
    }
    
    fn make_step<'a>(&self, counter:usize, old_pos:&'a str) -> &str {
        if self.walk[counter%self.walk.len()] {
            self.map[old_pos].0
        } else {
            self.map[old_pos].1
        }
    }

    fn solve2(&self) -> String {
        let mut all_pos:Vec<&str> = self.map.keys().filter(|&&k|  {
            return k.ends_with("A");
        }).map(|k| *k).collect();

        let mut counter = 0;
        let mut counters = Vec::new();
        // for each starting determine number of step to reach ending position
        while all_pos.len() > 0 {
            all_pos =  all_pos.iter()
                .map(|&pos| self.make_step(counter, pos)).collect();
            counter += 1;
            let previous_len = all_pos.len();
            all_pos = all_pos.iter().filter(|&&pos| !pos.ends_with("Z")).map(|&s| s ).collect();
            if all_pos.len() < previous_len {
                counters.push(counter);
            }
        }
        
        let nb_walk = self.walk.len();
        let mut result = nb_walk;
        for &c in counters.iter() {
            if c % nb_walk == 0 {
                result *= c / nb_walk;
            } else {
                result *= c;
            }
            
        }
        result.to_string()
    }

    fn solve(&self) -> String {
        if self.step == 1 {
            self.solve1()
        } else {
            self.solve2()
        }
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
