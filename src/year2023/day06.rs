use crate::utils::common::parse_usize;

#[derive(Debug)]
struct Puzzle {
    sub_puzzle:Vec<(usize,usize)>
}

impl Puzzle {
    fn build(input: &str) -> Puzzle {
        let mut times = parse_usize(input);
        let max_dist = times.split_off(times.len()/2);
        let mut sub_puzzle = Vec::new();
        for i in 0..times.len() {
            sub_puzzle.push((times[i], max_dist[i]));
        }
        Puzzle { sub_puzzle }
    }

    fn solve1(&self) -> usize {
        let results:Vec<usize> = self.sub_puzzle.iter().map(Self::eval1).collect();
        let mut total = 1;
        for r in results {
            total *= r;
        }
        total
    }

    fn solve2(&self) -> usize {
        let mut time_string = "".to_string();
        let mut dist_string = "".to_string();
        for t in self.sub_puzzle.iter() {
            time_string += &t.0.to_string();
            dist_string += &t.1.to_string();
        }

        Self::eval1(&(time_string.parse().unwrap(), dist_string.parse().unwrap()))
    }

    fn eval1((time, max_dist):&(usize,usize)) -> usize {
        // solving 2nd degree equation
        let delta = time * time - 4 * max_dist; // will panic if negative
        if delta == 0 {
            panic!("No way to perform better dist");
        }
        let sqr_delta = (delta as f64).sqrt();
        let min = (*time as f64 - sqr_delta) / 2.0;
        let mut max = (*time as f64 + sqr_delta) / 2.0;
        // println!("Float Solutions between : {} and {}", min, max);
        if max == max.floor() {
            max -= 1.0;
        }
        let min = min.floor() as usize + 1 ;
        let max = max.floor() as usize;
        // println!("Final Solutions between : {} and {}", min, max);
        max - min + 1
    } 
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(&input);
    if step == 1 {
        p.solve1().to_string()
    } else {
        p.solve2().to_string()
    }
}

