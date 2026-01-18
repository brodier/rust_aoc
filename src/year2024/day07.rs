use std::{sync::atomic::AtomicUsize, thread::scope};
use std::sync::atomic::Ordering::Relaxed;

struct Equation {
    test_value:usize,
    numbers:Vec<usize>
}

const METHOD_2:[fn(usize,usize)->usize;3] = [Equation::add, Equation::multiply, Equation::join];
const METHOD_1:[fn(usize,usize)->usize;2] = [Equation::add, Equation::multiply];

impl Equation {

    #[inline]
    fn multiply(result:usize, value:usize) -> usize {
        result * value
    }

    #[inline]
    fn add(result:usize, value:usize) -> usize {
        result + value
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

    fn iter_result(&self, idx:usize, tmp_result:usize, methods:&[fn(usize,usize)->usize]) -> bool {
        if tmp_result > self.test_value {
            return false;
        }
        if idx < self.numbers.len() {
            for method in methods {
                if self.iter_result(idx+1, method(tmp_result, self.numbers[idx]), methods) {
                    return true;
                }
            }
            return false;
        } else {
            return tmp_result.eq(&self.test_value);
        }
    }

    fn method(step:usize) -> &'static [fn(usize,usize)->usize] {
        if step==1 { &METHOD_1 } else { &METHOD_2 }
    }
}

fn parallel_solving(thread_id:usize, nb_threads:usize, step:usize, equations:&Vec<Equation>,total:&AtomicUsize) {
    for i in (thread_id..equations.len()).step_by(nb_threads) {
        let e = &equations[i];
        if e.iter_result(1, e.numbers[0], Equation::method(step)) {
            total.fetch_add(e.test_value, Relaxed);
        }
    }
}

pub fn solve(step:usize, contents:String) -> String {
    let mut equations = Vec::new();
    for line in contents.lines() {
        let (test_value, nums) = line.split_once(":").unwrap();
        let numbers:Vec<usize> = nums.trim().split(" ").map(|v| v.parse().unwrap()).collect();
        equations.push(Equation{test_value:test_value.parse().unwrap(), numbers:numbers});
    }
    let nb_threads = std::thread::available_parallelism().unwrap().get();
    let result = AtomicUsize::new(0);
    let total = &result;
    let list = &equations;
    scope(|scope| {
        for i in 0..nb_threads {
            scope.spawn(move || parallel_solving(i, nb_threads, step, list, total));
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