#[derive(Debug,PartialEq)]
enum Dir {
    UP,
    DOWN
}

#[derive(Debug)]
struct Report {
    vals:(usize,usize),
    skipped:Option<usize>,
    dir_set:bool    
}

impl Report {
    fn new(val0:usize, val1:usize) -> Report {
        if !Report::is_valid(val0, val1) {
            panic!("Error in Report::new");
        }
        Report{vals:(val0,val1),skipped:None, dir_set:false}
    }

    fn new_fixed(val0:usize, val1:usize) -> Report {
        let mut r = Report::new(val0, val1);
        r.dir_set = true;
        return r;
    }

    fn new_skipped(val0:usize, val1:usize, skipped:usize) -> Report {
        let mut r = Report::new(val0, val1);
        r.dir_set = true;
        r.skipped = Some(skipped);
        r
    }

    fn get_dir(&self) -> Dir {
        return if self.vals.0 > self.vals.1 { Dir::DOWN } else { Dir::UP };
    }

    fn append(&self, new_val:usize) -> Vec<Report> {
    // 33 36 37 39 36 35 : safe in skipping 39 or safe in skipping 37 or is safe
    // Should be unsafe

        let _ = if self.vals.0 > self.vals.1 { Dir::DOWN } else { Dir::UP};
        let mut result = Vec::new();
        if Report::is_safe(self.vals.0, self.vals.1, new_val) {
            let mut new_rep = Report::new_fixed(self.vals.1, new_val);
            new_rep.skipped = self.skipped;
            result.push(new_rep);
        }
        if self.skipped.is_some() {
            return result;
        }

        if Report::is_valid(self.vals.0, new_val) {
            let rep = Report::new_skipped(self.vals.0, new_val, self.vals.1);
            if !self.dir_set || self.get_dir() == rep.get_dir() {
                result.push(rep);
            }
        }
        if Report::is_valid(self.vals.1, new_val) {
            let rep = Report::new_skipped(self.vals.1, new_val, self.vals.0);
            if !self.dir_set || self.get_dir() == rep.get_dir() {
                result.push(rep);
            }
        }
        // skipping new_val is also a possible choice
        let new_rep = Report::new_skipped(self.vals.0, self.vals.1, new_val);
        result.push(new_rep);
        result
    }

    fn is_safe(val0:usize, val1:usize, val2:usize) -> bool {
        if !Report::is_valid(val1, val2) {
            return false;
        }
        if !Report::is_valid(val0, val1) {
            return false;
        }
        (val0 > val1 && val1 > val2) || (val0 < val1 && val1 < val2)
    }

    fn is_valid(prev:usize, val:usize) -> bool {
        if prev == 0 {
            return true;
        }
        let delta = prev.abs_diff(val);
        if delta > 3 || delta == 0 {
            return false;
        }
        return true; 
    }


}


fn day2_valid_line_accepting_one_err(line:&str) -> bool {
    let mut iter = line.split_whitespace().into_iter();
    let val0:usize = iter.next().unwrap().parse().unwrap();
    let val1:usize = iter.next().unwrap().parse().unwrap();
    let mut valid_reports = Vec::new();
    if Report::is_valid(val0,val1) {
        valid_reports.push(Report::new(val0,val1));
    } else {
        let val2 = iter.next().unwrap().parse().unwrap();
        if Report::is_valid(val0, val2) {
            let mut rep = Report::new(val0,val2);
            rep.skipped=Some(val1);
            valid_reports.push(rep);
        }
        if Report::is_valid(val1, val2) {
            let mut rep = Report::new(val1,val2);
            rep.skipped=Some(val0);
            valid_reports.push(rep);
        }
    }

    let mut nb_reports_to_check= valid_reports.len();  
    while nb_reports_to_check > 0 {
        let mut new_reports:Vec<Report> = Vec::new();
        if let Some(next_val) = iter.next() {
            let new_val:usize = next_val.parse().unwrap();
            for vr in &valid_reports {
                for new_report in vr.append(new_val) {
                    new_reports.push(new_report); 
                }
            }
        } else {
            break;
        }
        valid_reports = new_reports;
        nb_reports_to_check = valid_reports.len();
    }
    return valid_reports.len() > 0;
}

fn day2_valid_line(line:&str) -> bool {
    let mut prev =0;
    let mut dir = Dir::UP;
    let mut init_line:bool = false;
    for elem in line.split_whitespace().into_iter() {
        let val:usize = elem.parse().unwrap();
        if val == 0 {
            panic!("Unexpected value 0 for level");
        } 
        if !init_line {
            if prev == 0 {
                prev = val;
                continue;
            } else {
                if prev < val && 4 > (val - prev)  {
                    dir = Dir::UP;
                    prev = val;
                } else if prev > val && 4 > (prev - val) {
                    dir = Dir::DOWN;
                    prev = val;
                } else {
                    return false;
                }
                init_line = true;
            }
        } else {
            let delta = prev.abs_diff(val);
            if  delta == 0 || delta > 3 {
                return false;
            }

            if !match dir {
                Dir::UP => prev < val,
                Dir::DOWN => prev > val,
            } {
                return false;
            }
            prev = val;
        }
    }
    return true;
}

pub fn parse(input:String) -> String {
    input
}

pub fn part1(input:&String) -> String {
    let mut safe_counter = 0;
    for line in input.lines() {
        let safe:bool = day2_valid_line(line);
        if safe {
            safe_counter += 1;
        }
    }
    safe_counter.to_string()
}

pub fn part2(input:&String) -> String {
    let mut safe_counter = 0;
    for line in input.lines() {
        let safe:bool = day2_valid_line_accepting_one_err(line);
        if safe {
            safe_counter += 1;
        }
    }
    safe_counter.to_string()
}