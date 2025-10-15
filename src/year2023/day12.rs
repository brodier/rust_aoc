#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        Puzzle { step, input }
    }

    fn solve(&self) -> String {
        eprintln!("{}", self.input);
        self.step.to_string()
    }
}
pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
