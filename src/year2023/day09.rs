use crate::utils::common::parse_i64;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

fn extrapolate(list:Vec<i64>) -> i64 {
    if list.len() < 2 {
        panic!("Should never pass here !!");
    }
    let mut new_list = Vec::new();
    let mut itt = list.iter();
    let mut a = *itt.next().unwrap();
    let mut b = *itt.next().unwrap();
    let mut diff = b -a ;
    let mut has_only_zero = diff == 0;
    new_list.push(b - a);
    while let Some(&c) = itt.next() {
        a = b;
        b = c;
        diff = b - a;
        has_only_zero &= diff == 0;
        new_list.push(diff);
    }

    if has_only_zero {
        return *list.last().unwrap();
    } else {
        return list.last().unwrap() + extrapolate(new_list);
    }

}

impl Puzzle<'_> {
    fn build<'a>(step: usize, input: &'a str) -> Puzzle<'a> {
        Puzzle{ step , input }
    }

    fn solve1(&self) -> String {
        self.input.lines().map(|l| extrapolate(parse_i64(l))).sum::<i64>().to_string()
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
