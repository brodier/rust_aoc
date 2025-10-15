#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

#[derive(Debug)]
struct Univers {
    galaxies:Vec<(usize,usize)>
}

impl Univers {
    fn build(expand_size:usize, input:&str) -> Univers {
        let mut galaxies = Vec::new();
        let mut line_num = 0;
        // 1. look for expended line
        for line in input.lines() {
            if !line.contains("#") {
                // expend this line of the univers
                line_num += expand_size - 1;
            } else {
                for (x,b) in line.bytes().enumerate() {
                    if b == b'#' {
                        galaxies.push((x, line_num));
                    }
                }
            }
            line_num +=1;
        }
        // need to expend univers column by incressing x with some offset if not found galaxy on the current col
        galaxies.sort();
        let mut offset = 0;
        let mut last_x = galaxies.first().unwrap().0;
        for g in galaxies.iter_mut() {
            if last_x != g.0 && last_x + 1 != g.0 {
                    offset += (g.0 - last_x - 1)*(expand_size - 1);
            }
            last_x = g.0;
            g.0 += offset;
        }
        Univers{galaxies}        
    }

    fn sum_distance(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.galaxies.len() {
            for j in (i+1)..self.galaxies.len() {
                let gi = self.galaxies[i];
                let gj = self.galaxies[j];
                sum += gi.0.abs_diff(gj.0) + gi.1.abs_diff(gj.1);
            }
        }
        sum
    }
}
impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        Puzzle { step, input }
    }

    fn solve(&self) -> String {
        let expand_size = if self.step == 1 { 2 }  else { 1_000_000 };
        let u = Univers::build(expand_size, self.input);
        // println!("Univers : {:?}", u);
        u.sum_distance().to_string()
    }
}
pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
