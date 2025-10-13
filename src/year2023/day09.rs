use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}


impl Puzzle<'_> {
    fn build<'a>(step: usize, input: &'a str) -> Puzzle<'a> {
        Puzzle{ step , input }
    }

    fn solve1(&self) -> String {
        0.to_string()
    }

    fn solve2(&self) -> String {
        0.to_string()
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
