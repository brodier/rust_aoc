use std::collections::HashMap;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}


fn compute_hash(ins:&str) -> usize {
    let mut hash = 0;
    for &b in ins.as_bytes() {
        hash += b as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

struct Step2Processor {
    boxes:HashMap<u8,Vec<(String,u8)>>,
}

impl Step2Processor {
    fn build() -> Step2Processor {
        Step2Processor { boxes: HashMap::new() }
    }

    fn solve(&mut self, input:&str) -> usize {
        for ins in input.split(",") {
            if ins.contains("=") {
                let (label,focal) = ins.split_once("=").unwrap();
                self.add(label, focal.parse::<u8>().unwrap());
            } else {
                let (label, _) = ins.split_once("-").unwrap();
                self.remove(label);
            }
        }
        self.eval()
    }

    fn add(&mut self, label:&str, focal:u8) {
        let box_id = compute_hash(label) as u8;
        if let Some(cbox) = self.boxes.get_mut(&box_id) {
            // look for lens in the box
            for lens in cbox.iter_mut() {
                if lens.0 == label {
                    lens.1 = focal;
                    return;
                }
            }
            cbox.push((label.to_string(), focal));
        } else {
            let mut box_lenses = Vec::new();
            box_lenses.push((label.to_string(), focal));
            self.boxes.insert(box_id as u8, box_lenses);
        }
    }

    fn remove(&mut self, label:&str) {
        let box_id = compute_hash(label) as u8;
        if let Some(cbox) = self.boxes.get(&box_id) {
            let new_content = cbox.iter().filter(|v| v.0 != label).map(|v| (v.0.clone(), v.1)).collect();
            self.boxes.insert(box_id, new_content);
        }
    }

    fn eval(&self) -> usize {
        let mut result = 0;
        for (&box_id, cbox) in self.boxes.iter() {
            for (lens_slot, &(_,focal)) in cbox.iter().enumerate() {
                result += (box_id as usize + 1) * (lens_slot + 1)  * (focal as usize);
            }
        }
        result
    }


}
impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        Puzzle { step, input }
    }

    fn step1(&self) -> String {
        let mut hash = 0;
        for ins in self.input.split(",") {
            hash += compute_hash(ins);
        }
        return hash.to_string();
    }

    fn solve(&self) -> String {
        if self.step == 1 {
            return self.step1();
        } else {
            Step2Processor::build().solve(self.input).to_string()
        }
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
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