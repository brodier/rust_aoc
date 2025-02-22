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

pub fn solve(step:usize, contents:String) -> usize {
    let mut equations = Vec::new();
    for line in contents.lines() {
        let (test_value, nums) = line.split_once(":").unwrap();
        let numbers:Vec<usize> = nums.trim().split(" ").map(|v| v.parse().unwrap()).collect();
        equations.push(Equation{test_value:test_value.parse().unwrap(), numbers:numbers});
    }
    let mut result = 0;
    for e in equations {
        if if step == 1 { e.test1() } else { e.test2() } {
            result += e.test_value;
        } 
    }
    result
}