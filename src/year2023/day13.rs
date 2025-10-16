use std::str;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

fn eval(pattern:&str, skip:usize) -> usize {
        let cols_skip = if skip >= 100 { 0 } else { skip };
        let sum = horizontal_eval(&flip_pattern(pattern), cols_skip);
        if sum > 0 {
            return sum;
        }
        let rows_skip = if skip >= 100 { skip/100 } else { 0 };
        return 100 * horizontal_eval(pattern, rows_skip);
}

fn eval_part2(pattern:&str) -> usize {
    let part1 = eval(pattern,0);
    let mut new_pattern = pattern.as_bytes().to_owned();
    let len = new_pattern.len();
    for i in 0..len {
        let tested_smug_char = new_pattern[i];
        if tested_smug_char == b'\n' {
            continue;
        }
        let fixed_smug = if tested_smug_char == b'#' {
            b'.'
        } else {
            b'#'
        };
        new_pattern[i] = fixed_smug;
        let copy_pattern = std::str::from_utf8(&new_pattern).unwrap();

        let r = eval(copy_pattern, part1);
        if r > 0 {
            return r;
        }
        new_pattern[i] = tested_smug_char;
    }
    return 0;
}

fn flip_pattern(pattern:&str) -> String {
    let lines:Vec<&str> = pattern.split("\n").collect();
    let line_len = lines[0].len();
    let mut flip_pattern = "".to_string();
    for i in 0..line_len {
        for &line in lines.iter() {
            flip_pattern.push(char::from_u32(line.as_bytes()[i] as u32).unwrap());
        }
        flip_pattern += "\n";
    }
    flip_pattern.pop();
    flip_pattern
}

fn horizontal_eval(pattern:&str, skip:usize) -> usize {
    let lines:Vec<&str> = pattern.split("\n").collect();
    let mut horizontal_candidates:Vec<usize> = Vec::new();
    let mut previous_line = "";
    let mut sum = 0;
    for (line_num, &line) in lines.iter().enumerate() {
        if previous_line == line {
            horizontal_candidates.push(line_num);
        }
        previous_line = line;
    }
    for &lc in horizontal_candidates.iter() {
        if lc == skip {
            continue;
        }
        let mut is_valid = true;
        let nb_lines_to_check = lc.min(lines.len() - lc);
        for i in 0..nb_lines_to_check {
            if lines[lc+i] != lines[lc-1-i] {
                is_valid=false;
                break;
            }
        }
        if is_valid {
            sum += lc;
            break; // if multiple valid reflective line on same pattern then we need to remove the break
        }
    }
    sum
}

impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        Puzzle { step, input }
    }

    fn solve(&self) -> String {
        let mut remain = Some(self.input);
        let mut sum = 0;
        while remain.is_some() {
            let pattern:&str;
            (pattern, remain) = if let Some((new_pattern,new_remain)) = remain.unwrap().split_once("\n\n") {
                (new_pattern, Some(new_remain))
            } else {
                (remain.unwrap(), None)
            };
            
            sum += if self.step == 1 {
                eval(pattern,0)
            } else {
                eval_part2(pattern)
            };
        }
        sum.to_string()
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
