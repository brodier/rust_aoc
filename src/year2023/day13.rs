#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

fn eval(pattern:&str) -> usize {
        // println!("search rows");
        let mut sum = 100 * horizontal_eval(pattern);
        // println!("search cols");
        sum +=  horizontal_eval(&flip_pattern(pattern));
        // println!("search result : {}", sum);
        sum
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

fn horizontal_eval(pattern:&str) -> usize {
    let lines:Vec<&str> = pattern.split("\n").collect();
    let mut horizontal_candidates:Vec<usize> = Vec::new();
    // println!("eval pattern:\n{}", pattern);
    let mut previous_line = "";
    let mut sum = 0;
    for (line_num, &line) in lines.iter().enumerate() {
        if previous_line == line {
            horizontal_candidates.push(line_num);
        }
        previous_line = line;
    }
    for &lc in horizontal_candidates.iter() {
        // println!("checking candidate {}", lc);
        let mut is_valid = true;
        let nb_lines_to_check = lc.min(lines.len() - lc);
        for i in 0..nb_lines_to_check {
            if lines[lc+i] != lines[lc-1-i] {
                // println!("{}:[{}]  <> {}:[{}]", lc+i, lines[lc+i], lc-1-i, lines[lc-1-i]);
                is_valid=false;
                break;
            }
        }
        if is_valid {
            // println!("line found : {}", lc);
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
        if self.step == 2 {
            return self.step.to_string();
        }
        let mut remain = Some(self.input);
        let mut sum = 0;
        while remain.is_some() {
            let pattern:&str;
            (pattern, remain) = if let Some((new_pattern,new_remain)) = remain.unwrap().split_once("\n\n") {
                (new_pattern, Some(new_remain))
            } else {
                (remain.unwrap(), None)
            };
            sum += eval(pattern);
        }
        sum.to_string()
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
