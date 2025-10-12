use crate::utils::common::parse_usize;

#[derive(Debug)]
struct Puzzle {
}

impl Puzzle {
    fn build(input: &str) -> Puzzle {
        Puzzle {}
    }

    fn solve1(&self) -> usize {
        0
    }

    fn solve2(&self) -> usize {
        0
    }

}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(&input);
    if step == 1 {
        p.solve1().to_string()
    } else {
        p.solve2().to_string()
    }
}

