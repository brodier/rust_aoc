
pub fn solve(step:usize, input:String) -> String {
    let mut result:usize = 0;
    for line in input.lines()  {
        let step_line = if step == 2 {
            let mut tmp_line = line.to_string();
            for sdigit in [("one","o1e"), 
                ("two","t2o"), ("three","t3e"), ("four","f4r"), ("five","f5e"), ("six","s6x"), 
                ("seven","s7n"), ("eight","e8t"), ("nine","n9e")] {
                tmp_line = tmp_line.replace(sdigit.0, sdigit.1);
            }
            tmp_line
        } else {
            line.to_string()
        };
        let mut first = None;
        let mut last = 0;
        for c in step_line.bytes() {
            if c < b'0' || c > b'9' {
                continue;
            }
            let digit = c - b'0';
            if first.is_none() {
                first = Some(digit);
            }
            last = digit;
        }
        // println!("(first, last) digits ({:?}, {:?} for : {:?}", first, last, line);
        result += (first.unwrap() * 10 + last) as usize;
    }
    result.to_string()
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