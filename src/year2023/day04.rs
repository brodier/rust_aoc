

#[derive(Debug)]
struct Puzzle {
   result1:usize,
}

impl Puzzle {
    fn build() -> Puzzle {
        Puzzle{result1: 0}
    }

    fn eval(&mut self) -> usize {
        self.result1
    }
}

pub fn solve(_step: usize, _input: String) -> String {
    let mut puzzle = Puzzle::build();
    puzzle.eval().to_string()
}
