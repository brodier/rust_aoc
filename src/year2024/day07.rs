use std::{sync::atomic::AtomicUsize, thread::scope};
use std::sync::atomic::Ordering::Relaxed;

struct Equation {
    test_value:usize,
    nb_eq:usize,
    numbers:Vec<usize>
}

#[inline]
fn join(result:usize, value:usize) -> usize {
    let mut pow = 10;
    let mut v = value;
    while v >= 10 {
        v /= 10;
        pow *= 10;
    }
    result * pow + value
}

impl Equation {

    fn iter_result(&self, idx:usize, tmp_result:usize) -> bool {
        if tmp_result > self.test_value {
            return false;
        }
        if idx < self.numbers.len() {
            for op in 0..self.nb_eq {
                let next_result = match op {
                    0 => tmp_result * self.numbers[idx],
                    1 => tmp_result + self.numbers[idx],
                    2 => join(tmp_result, self.numbers[idx]),
                    _ => unreachable!()
                };
                if self.iter_result(idx+1, next_result) {
                    return true;
                }
            }
            return false;
        } else {
            return tmp_result.eq(&self.test_value);
        }
    }

}

fn parallel_solving(thread_id:usize, nb_threads:usize, equations:&Vec<Equation>,total:&AtomicUsize) {
    for i in (thread_id..equations.len()).step_by(nb_threads) {
        let e = &equations[i];
        if e.iter_result(1, e.numbers[0]) {
            total.fetch_add(e.test_value, Relaxed);
        }
    }
}

pub fn solve(step:usize, contents:String) -> String {
    let mut equations = Vec::new();
    let nb_eq= if step == 1 { 2 } else { 3 };
    for line in contents.lines() {
        let (test_value, nums) = line.split_once(":").unwrap();
        let numbers:Vec<usize> = nums.trim().split(" ").map(|v| v.parse().unwrap()).collect();
        equations.push(Equation{test_value:test_value.parse().unwrap(), nb_eq, numbers:numbers});
    }
    let nb_threads = std::thread::available_parallelism().unwrap().get();
    let result = AtomicUsize::new(0);
    let total = &result;
    let list = &equations;
    scope(|scope| {
        for i in 0..nb_threads {
            scope.spawn(move || parallel_solving(i, nb_threads, list, total));
        }
    });
    result.into_inner().to_string()
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