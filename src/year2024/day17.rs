use crate::utils::common::parse_usize;

#[derive(Debug,Clone)]
struct State {
    a:usize,
    b:usize,
    c:usize,
    p:usize
}

#[derive(Debug,Clone)]
pub struct VirtualMachine {
    state:State,
    prog:Vec<usize>,
    output:Vec<usize>,
}


impl State {
    fn combo_op(&self, op:usize) -> usize {
        match op {
            0|1|2|3 => op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("should never pass here"),
        }
    }
}


impl VirtualMachine { 
    pub fn build(input:Vec<usize>) -> VirtualMachine {
        let mut prog = Vec::new();
        for i in &input[3..] {
            prog.push(*i);
        }
        VirtualMachine{state:State{a:input[0], b:input[1], c:input[2], p:0}, prog, output:Vec::new()}
    }

    pub fn compute(&mut self, ins:usize, op:usize) {
        match ins {
            0 => self.adv(op),
            1 => self.bxl(op),
            2 => self.bst(op),
            3 => self.jnz(op),
            4 => self.bxc(op),
            5 => self.out(op),
            6 => self.bdv(op),
            7 => self.cdv(op),
            _ => panic!("Invalid instruction")
        }
    }

    pub fn run(&mut self) -> Vec<usize> {
        while self.state.p < self.prog.len() {
            let inst = self.prog[self.state.p];
            let op = self.prog[self.state.p+1];
            self.state.p+=2;
            self.compute(inst, op);
        }
        self.output.clone()
    }

    fn run_with_a(&self, new_a:usize) -> Vec<usize> {
        let mut copy = self.clone();
        copy.state.a = new_a;
        copy.run()
    }
    
    fn run_iter(&self, num:usize, a:usize) -> Option<usize> {
        for sub_a_to_test in 0..8 {
            let new_a = 8 * a + sub_a_to_test;
            let sub_prog = self.prog[(self.prog.len()-(num+1))..].to_vec();
            let run_result = self.run_with_a(new_a);
            if run_result == sub_prog {
                if run_result.len() == self.prog.len() {
                    return Some(new_a);
                }
                if let Some(rec_result) = self.run_iter(num+1, new_a) {
                    return Some(rec_result);
                } else {
                    continue;
                }
            }
        }
        return None;
    }

    pub fn solve_step2(&self) -> usize {
        return self.run_iter(0,0).unwrap();
    }    

        
    fn adv(&mut self, op:usize) {
        self.state.a /= 1 << self.state.combo_op(op);
    }

    fn bxl(&mut self, op:usize) {
        self.state.b ^= op;
    }

    fn bst(&mut self, op:usize) {
        self.state.b = self.state.combo_op(op) % 8;
    }

    fn jnz(&mut self, op:usize) {
        if self.state.a != 0 {
            self.state.p = op;
        }
    }

    fn bxc(&mut self, _op:usize) {
        self.state.b ^= self.state.c;
    }

    fn out(&mut self, op:usize) {
        self.output.push(self.state.combo_op(op) % 8);
    }

    fn bdv(&mut self, op:usize) {
        self.state.b = self.state.a / (1 << self.state.combo_op(op));
    }

    fn cdv(&mut self, op:usize) {
        self.state.c = self.state.a / (1 << self.state.combo_op(op));
    }

}

pub fn solve(step:usize, input:String) -> String {
    let mut vm = VirtualMachine::build(parse_usize(&input));
    if step == 1 {
        format!("{:?}", vm.run()).replace("[", "")
        .replace(" ", "").replace("]", "")
    } else {
       return vm.solve_step2().to_string();
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn adv_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 0,1,5,4"));
        assert_eq!(vm.run(), vec![4]);
        assert_eq!(vm.state.a, 364);
    }

    #[test]
    fn bxl_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,7,0 1,1"));
        assert_eq!(vm.run(), vec![]);
        assert_eq!(vm.state.b, 6);
    }

    #[test]
    fn bst_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,354,129 2,6"));
        assert_eq!(vm.run(), vec![]);
        assert_eq!(vm.state.a, 729);
        assert_eq!(vm.state.b, 1);
        assert_eq!(vm.state.c, 129);
    }

    #[test]
    fn jnz_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 0,1,5,4,3,0"));
        assert_eq!(vm.run(), vec![4,6,3,5,6,3,5,2,1,0]);
        assert_eq!(vm.state.a, 0);
        assert_eq!(vm.state.b, 0);
        assert_eq!(vm.state.c, 0);
    }

    #[test]
    fn bxc_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,128,64 4,6"));
        assert_eq!(vm.run(), vec![]);
        assert_eq!(vm.state.b, 192);
    }

    #[test]
    fn out_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 0,1,5,4,3,0"));
        assert_eq!(vm.run(), vec![4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn bdv_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 6,1"));
        assert_eq!(vm.run(), vec![]);
        assert_eq!(vm.state.a, 729);
        assert_eq!(vm.state.b, 364);
        assert_eq!(vm.state.c, 0);
    }

    #[test]
    fn cdv_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 7,1"));
        assert_eq!(vm.run(), vec![]);
        assert_eq!(vm.state.a, 729);
        assert_eq!(vm.state.b, 0);
        assert_eq!(vm.state.c, 364);
    }
}
