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
        let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
        let mut map = HashMap::new();
        while let Some(line) = lines.next() {
            let (_, [key, left, right])  = re.captures(line).unwrap().extract();
            map.insert(key, (left, right));
        }
        Puzzle{ step, walk, map}
    }

    fn solve(&self) -> String {
        let mut pos = "AAA";
        let mut counter = 0;
        while pos != "ZZZ" {
            pos =  if self.walk[counter%self.walk.len()] {
                self.map[pos].0
            } else {
                self.map[pos].1
            };
            counter+=1;
        }
        counter.to_string()
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
