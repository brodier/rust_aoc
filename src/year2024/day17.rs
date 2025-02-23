use crate::utils::common::parse_usize;

#[derive(Debug)]
struct State {
    a:usize,
    b:usize,
    c:usize,
    p:usize
}

#[derive(Debug)]
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

    pub fn run(&mut self) -> String {
        loop {
            let inst = self.prog[self.state.p];
            let op = self.prog[self.state.p+1];
            self.state.p+=2;
            self.compute(inst, op);
            if self.state.p >= self.prog.len() {
                break;
            }
            println!("{:?}", self);
        }
        format!("{:?}", self.output)
        .replace("[", "")
        .replace(" ", "")
        .replace("]", "")
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

pub fn solve(_step:usize, input:String) -> String {
    let mut vm = VirtualMachine::build(parse_usize(&input));
    println!("Start : {:?}", vm);
    vm.run()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn adv_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 0,1,5,4"));
        assert_eq!(vm.run(), "4");
        assert_eq!(vm.state.a, 364);
    }

    #[test]
    fn bxl_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,7,0 1,1"));
        assert_eq!(vm.run(), "");
        assert_eq!(vm.state.b, 6);
    }

    #[test]
    fn bst_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,354,129 2,6"));
        assert_eq!(vm.run(), "");
        assert_eq!(vm.state.a, 729);
        assert_eq!(vm.state.b, 1);
        assert_eq!(vm.state.c, 129);
    }

    #[test]
    fn jnz_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 0,1,5,4,3,0"));
        assert_eq!(vm.run(), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(vm.state.a, 0);
        assert_eq!(vm.state.b, 0);
        assert_eq!(vm.state.c, 0);
    }

    #[test]
    fn bxc_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,128,64 4,6"));
        assert_eq!(vm.run(), "");
        assert_eq!(vm.state.b, 192);
    }

    #[test]
    fn out_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 0,1,5,4,3,0"));
        assert_eq!(vm.run(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn bdv_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 6,1"));
        assert_eq!(vm.run(), "");
        assert_eq!(vm.state.a, 729);
        assert_eq!(vm.state.b, 364);
        assert_eq!(vm.state.c, 0);
    }

    #[test]
    fn cdv_test() {
        let mut vm = VirtualMachine::build(parse_usize("729,0,0 7,1"));
        assert_eq!(vm.run(), "");
        assert_eq!(vm.state.a, 729);
        assert_eq!(vm.state.b, 0);
        assert_eq!(vm.state.c, 364);
    }

}
