#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

struct Context<'a> {
    index:usize,
    line:&'a str,
    seq:&'a Vec<usize>
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Candidate {
    index_seq:usize,
    curr_nb_damage:usize,
    weight:usize
}

impl Candidate {

    fn next(&self, ctx:&Context) -> (Option<Candidate>, Option<Candidate>) {
        // set current_char from line at index self.index if current_char is '?' create the two variant '.' and '#'
        let current_char = ctx.line.as_bytes()[ctx.index];
        if current_char == b'?' {
            let candidate_on_ok  = self.eval_next(ctx, b'.');
            // println!("eval next {:?} on {} => {:?}", self, ".", candidate_on_ok);
            let candidate_on_damage = self.eval_next(ctx, b'#');
            // println!("eval next {:?} on {} => {:?}", self, "#", candidate_on_damage);
            (candidate_on_ok, candidate_on_damage)
        } else {
            (self.eval_next(ctx, current_char),None)
        }
    }

    fn eval_next(&self, ctx:&Context, current_char:u8) -> Option<Candidate> {
        if self.index_seq == ctx.seq.len() {
            if current_char == b'.' {
                return Some(Candidate{index_seq: self.index_seq, curr_nb_damage: 0, weight: self.weight});
            } else {
                return None;
            }
             
        }
        if self.curr_nb_damage == ctx.seq[self.index_seq]{
            if current_char == b'.' {
                return Some(Candidate{index_seq: self.index_seq + 1, curr_nb_damage: 0, weight: self.weight});
            } else {
                return None;
            }
        } 
        if current_char == b'.' {
            let min_remaing_len = ctx.seq[self.index_seq..].into_iter().sum::<usize>() + ctx.seq[self.index_seq..].len() - self.curr_nb_damage;
            let remaining_len = ctx.line.len() - ctx.index;
            if  min_remaing_len > remaining_len || self.curr_nb_damage > 0 {
                return None;
            } else {
                return Some(Candidate{index_seq: self.index_seq, curr_nb_damage: 0, weight: self.weight});
            } 
        } else {
            Some(Candidate{index_seq: self.index_seq, curr_nb_damage: self.curr_nb_damage+1, weight: self.weight})
        }
    }

}


impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        Puzzle { step, input }
    }

    fn solve(&self) -> String {
        // println!("{}", self.input);
        let mut sum = 0;
        for (_i, line) in self.input.lines().enumerate() {
            let line = if self.step == 2 {
                Self::unfold_line(line)
            } else {
                line.to_string()
            };
            sum += Self::eval_line(&line);
            // println!("sum on line {} : {}", _i, sum);
        }
        sum.to_string()
    }

    fn unfold_line(line:&str) -> String {
        let (chain, seq) = line.split_once(" ").unwrap();
        let chain = chain.to_string() + "?";
        let mut chain = chain.repeat(5);
        chain.remove(chain.len()-1);
        let seq = seq.to_string() + ",";
        let mut seq = seq.repeat(5);
        seq.remove(seq.len()-1);
        return chain + " " + &seq;
    }

    fn eval_line<'a>(line:&'a str) -> usize {
        let (chain, seq) = line.split_once(" ").unwrap();
        let seq:Vec<usize> = seq.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        // println!("chain [{}], seq: {:?}", chain, seq);
        let mut ctx = Context{index: 0, line: chain, seq: &seq};
        let c = Candidate{index_seq: 0, curr_nb_damage: 0, weight: 1};
        let mut candidates = Vec::new();
        candidates.push(c);
        for _ in 0..chain.len() {
            let mut new_candidates = Vec::new();
            for c in candidates.iter() {
                // println!("Checking {:?}", c);
                let (c1, c2) = c.next(&ctx);
                // println!("found : ({:?},{:?})", c1, c2);
                if c1.is_some() {
                    new_candidates.push(c1.unwrap());
                }
                if c2.is_some() {
                    new_candidates.push(c2.unwrap());
                }
            }
            // Merge candidates with same index_seq and curr_nb_damage by summing their weight in the new candidate
            new_candidates.sort();
            candidates.clear();
            let mut weight = 0;
            let first_candidate = new_candidates.first().unwrap();
            let mut index_seq = first_candidate.index_seq;
            let mut curr_nb_damage = first_candidate.curr_nb_damage;
            for c in new_candidates {
                if index_seq == c.index_seq && curr_nb_damage == c.curr_nb_damage {
                    weight += c.weight;
                } else {
                    candidates.push(Candidate { index_seq, curr_nb_damage, weight });
                    index_seq = c.index_seq;
                    curr_nb_damage = c.curr_nb_damage;
                    weight = c.weight;
                }                
            }
            candidates.push(Candidate { index_seq, curr_nb_damage, weight });
            ctx.index += 1;
            // println!("candidates  {:?}", candidates);
        }
        candidates.iter().map(|c| c.weight).sum()
    }

}
pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
