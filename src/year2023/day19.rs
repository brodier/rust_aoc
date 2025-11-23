use std::{cmp::Ordering};
use std::fmt::Formatter;
use std::fmt::Result;
use crate::utils::common::parse_usize;

pub struct ParseResult {
    system:Vec<Instruction>,
    parts:Vec<Part>
}

#[derive(Clone)]
enum Instruction {
    Jump(usize),
    JumpIf(XmasCond,usize),
    Accept,
    Reject,
}

#[derive(Clone, Debug)]
struct Part {
    x:usize,
    m:usize,
    a:usize,
    s:usize,
}

#[derive(Clone)]
struct XmasCond {
    cat:u8,
    cmp_op:bool,
    value:usize,
}

impl std::fmt::Debug for XmasCond {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let op = if self.cmp_op { b'<' } else { b'>' };
    let prefix = String::from_utf8(vec![self.cat, 32, op, 32]).unwrap();
    write!(f, "{}{}", prefix,self.value)
  }
}

impl XmasCond {
    fn build(cat:u8, cmp_op:bool, value:usize) -> XmasCond {
        XmasCond { cat, cmp_op, value }
    }

    fn eval(&self, part:&Part) -> bool {
        (part.get(self.cat).cmp(&self.value) == Ordering::Greater) ^ self.cmp_op
    }

    // return ok_min_max and ko_min_max for this condition with current min max
    fn split_min_max(&self, cat:u8, (min,max):&(Part,Part)) -> (Option<(Part,Part)>,Option<(Part,Part)>) {
        let new_ok_min_val  = self.get_min_ok(min.get(cat));
        let new_ok_max_val  = self.get_max_ok(max.get(cat));
        let new_ok_min_max = if new_ok_min_val > new_ok_max_val {
            None
        } else {
            let (mut min_ok,mut max_ok) = (min.clone(),max.clone());
            min_ok.set(cat, new_ok_min_val);
            max_ok.set(cat, new_ok_max_val);
            Some((min_ok,max_ok))
        };
        let new_ko_min_val  = self.get_min_ko(min.get(cat));
        let new_ko_max_val  = self.get_max_ko(max.get(cat));
        let new_ko_min_max = if new_ko_min_val > new_ko_max_val {
            None
        } else {
            let (mut min_ko,mut max_ko) = (min.clone(), max.clone());
            min_ko.set(cat, new_ko_min_val);
            max_ko.set(cat, new_ko_max_val);
            Some((min_ko,max_ko))
        };

        (new_ok_min_max,new_ko_min_max)        
    }

    fn get_min_ok(&self, min:usize) -> usize {
        if self.cmp_op { min } else {  if min < self.value { self.value+1 } else { min } }
    }

    fn get_min_ko(&self, min:usize) -> usize {
        if self.cmp_op { if min < self.value { self.value } else { min } } else { min }
    }

    fn get_max_ok(&self, max:usize) -> usize {
        if self.cmp_op { if max > self.value { self.value-1 } else { max } } else { max }
    }

    fn get_max_ko(&self, max:usize) -> usize {
        if self.cmp_op { max } else { if max < self.value { max } else { self.value } }
    }

}

impl Part {
    fn build(prat_line:&str) -> Part {
        let cat_rates = parse_usize(prat_line);
        Part{x: cat_rates[0], m: cat_rates[1], a: cat_rates[2], s: cat_rates[3]}
    }

    fn get(&self, cat:u8) -> usize {
        match cat {
            b'x' => self.x,
            b'm' => self.m,
            b'a' => self.a,
            b's' => self.s,
            _ => unreachable!()
        }
    }

    fn set(&mut self, cat:u8, value:usize) {
        match cat {
            b'x' => self.x = value,
            b'm' => self.m = value,
            b'a' => self.a = value,
            b's' => self.s = value,
            _ => unreachable!()
        }
    }

    
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn nb_combinations(&self, min:&Part) -> u64 {
        let mut total = (self.x-min.x+1) as u64;
        total *= (self.m-min.m+1) as u64;
        total *= (self.a-min.a+1) as u64;
        total *= (self.s-min.s+1) as u64;
        total
    }
}

struct Compiler<'a> {
    labels:Vec<&'a str>,
    labels_addr:Vec<usize>,
    instructions:Vec<Instruction>,
    link_inst:Vec<(&'a str,usize)>
}

impl <'c>Compiler<'c> {
    fn new<'a>() -> Compiler<'a> {
        Compiler{labels:Vec::new(), labels_addr:Vec::new(), instructions:Vec::new(), link_inst:Vec::new()}
    }

    fn compile<'b>(&mut self, program:&'c str) {
        self.link_inst.push((&"in", 0));
        self.instructions.push(Instruction::Jump(0));
        self.labels.push("A");
        self.labels_addr.push(self.instructions.len());
        self.instructions.push(Instruction::Accept);
        self.labels.push("R");
        self.labels_addr.push(self.instructions.len());
        self.instructions.push(Instruction::Reject);
        for inst_line in program.lines() {
            if inst_line.is_empty() {
                break;
            }
            let (label,inst_seq) = inst_line.split_once("{").unwrap();
            let inst_seq = &inst_seq[..inst_seq.len()-1];
            self.labels.push(label);
            self.labels_addr.push(self.instructions.len());
            for inst in inst_seq.split(",") {
                let op_code = inst.as_bytes();
                if op_code[0] == b'A' {
                    self.instructions.push(Instruction::Accept);
                } else if op_code[0] == b'R' {
                    self.instructions.push(Instruction::Reject);
                } else if op_code[1] == b'<' || op_code[1] == b'>'{
                    let cat = op_code[0];
                    let value = parse_usize(inst)[0];
                    let pos = self.instructions.len();
                    self.instructions.push(Instruction::JumpIf(XmasCond::build(cat, op_code[1] == b'<', value), 0));
                    let (_,jump_adr) = inst.split_once(":").unwrap();
                    self.link_inst.push((jump_adr, pos));
                } else {
                    let pos = self.instructions.len();
                    self.link_inst.push((inst, pos));
                    self.instructions.push(Instruction::Jump(0));
                }
            }
        }
    }

    fn link(&mut self) -> Vec<Instruction> {
        for (jump_adr,pos) in self.link_inst.iter() {
            let &pos = pos;
            // Probably not optimal for retriving position of label in instructions vector
            let adr = self.labels_addr[self.labels.iter().position(|&label| label == *jump_adr).unwrap()];
            self.instructions[pos] = match &self.instructions[pos] {
                Instruction::JumpIf(cond, _) => Instruction::JumpIf(cond.clone(), adr),
                Instruction::Jump(_) => Instruction::Jump(adr),
                _ => unreachable!()
            };
        }
        self.instructions.clone()
    }

}


pub fn parse(input:String) -> ParseResult {
    let mut compiler = Compiler::new();
    compiler.compile(&input);
    let system = compiler.link();
    let mut skip_inst = true;
    let mut parts = Vec::new();
    for part_line in input.lines() {
        if skip_inst && !part_line.is_empty() {
            continue;
        } else if skip_inst {
            skip_inst = false;
            continue;
        }
        parts.push(Part::build(part_line));
    }
    ParseResult { system, parts }
}

fn exec_system_on(p:&Part, system:&Vec<Instruction>) -> bool {
        let mut code_pointer:usize = 0;
        loop {
            code_pointer = match &system[code_pointer] {
                Instruction::Accept => {
                    return true;
                },
                Instruction::Reject => {
                    return false;
                },
                Instruction::Jump(addr) => *addr,
                Instruction::JumpIf(cond, addr) => if cond.eval(p) {
                    *addr
                } else {
                    code_pointer+1
                }

            }
        }
}

pub fn part1(pr:&ParseResult) -> String {
    let mut accepted = Vec::new();
    for p in pr.parts.iter() {
        if exec_system_on(p, &pr.system) {
            accepted.push(p);
        }
    }
    accepted.iter().map(|&p| p.sum()).sum::<usize>().to_string()    
}

pub fn part2(pr:&ParseResult) -> String {
    let system = &pr.system;
    let seed = ((Part::build(&"1,1,1,1"),Part::build(&"4000,4000,4000,4000")), 0);
    let mut todo = Vec::new();
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    todo.push(seed);
    while let Some((min_max, init_pos)) = todo.pop() {
        let mut code_pointer = init_pos;
        loop {
            code_pointer = match &system[code_pointer] {
                Instruction::Accept => {
                    accepted.push(min_max);
                    break;
                },
                Instruction::Reject => {
                    rejected.push(min_max);
                    break;
                },
                Instruction::Jump(addr) => *addr,
                Instruction::JumpIf(cond, addr) => {
                    let (ok_min_max, ko_min_max) =  cond.split_min_max(cond.cat, &min_max);
                    // println!("on cond {:?} from {:?} \n=> OK: {:?}\n=> KO: {:?}", cond, min_max, ok_min_max, ko_min_max);
                    let nb_combi = min_max.1.nb_combinations(&(min_max.0));
                    let mut nb_combi_ok = 0;
                    let mut nb_combi_ko = 0;

                    if let Some(min_max) = ok_min_max {
                        nb_combi_ok = min_max.1.nb_combinations(&(min_max.0));
                        todo.push((min_max, *addr));
                    }                    
                    if let Some(min_max) = ko_min_max {
                        nb_combi_ko = min_max.1.nb_combinations(&(min_max.0));
                        todo.push((min_max, code_pointer + 1));
                    }
                    if nb_combi != nb_combi_ok + nb_combi_ko {
                        // println!("loosing combination on slit {} =/= {} + {}", nb_combi, nb_combi_ok, nb_combi_ko);
                    }
                    break;
                }
            }
        }
    }
    accepted.iter().map(|(min_part,max_part)| max_part.nb_combinations(min_part)).sum::<u64>().to_string()
}
