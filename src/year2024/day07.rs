use std::{sync::atomic::AtomicUsize, thread::scope};
use std::sync::atomic::Ordering::Relaxed;

struct Equation {
    test_value:usize,
    numbers:Vec<usize>
}

impl Equation {
    fn test1(&self) -> bool {
        let mut result;
        for i in 0..usize::pow(2,(self.numbers.len() - 1) as u32) {
            let mut bitmap = i;
            let mut iterator = self.numbers.iter();
            result = *iterator.next().unwrap();
            while let Some(n) = iterator.next() {
                if bitmap & 1 == 1 {
                    result *= n;
                } else {
                    result += n;
                }
                bitmap/=2;
            }
            if result == self.test_value {
                return true;
            }
        }
        false
    }

    fn test2(&self) -> bool {
        let mut result;
        for i in 0..usize::pow(3,(self.numbers.len() - 1) as u32) {
            let mut bitmap = i;
            let mut iterator = self.numbers.iter();
            result = *iterator.next().unwrap();
            while let Some(n) = iterator.next() {
                if bitmap % 3 == 0 {
                    result *= n;
                } else if bitmap % 3 == 1{
                    result += n;
                } else {
                    result = format!("{}{}", result, n).parse().unwrap();
                }
                bitmap/=3;
            }
            if result == self.test_value {
                return true;
            }
        }
        false
    }
}

fn parallel_solving(thread_id:usize, nb_threads:usize, step:usize, equations:&Vec<Equation>,total:&AtomicUsize) {
    for i in (thread_id..equations.len()).step_by(nb_threads) {
        let e = &equations[i];
        if if step == 1 { e.test1() } else { e.test2() } {
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