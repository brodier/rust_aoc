use crate::utils::common::parse_i64;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

fn extrapolate(list:Vec<i64>, is_forward:bool) -> i64 {
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
        if is_forward {
            return *list.last().unwrap();
        } else {
            return *list.first().unwrap();
        }
        
    } else {
        if is_forward {
            return list.last().unwrap() + extrapolate(new_list, is_forward);
        } else {
            return *list.first().unwrap() - extrapolate(new_list, is_forward);
        }        
    }

}

impl Puzzle<'_> {
    fn build<'a>(step: usize, input: &'a str) -> Puzzle<'a> {
        Puzzle{ step , input }
    }

    fn solve(&self) -> String {
        self.input.lines().map(|l| extrapolate(parse_i64(l), self.step == 1)).sum::<i64>().to_string()
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
