use crate::utils::common::parse_usize;

#[derive(Debug)]
struct State {
    a:usize,
    b:usize,
    c:usize,
    p:usize
}

#[derive(Debug)]
struct VirtualMachine {
    state:State,
    prog:Vec<usize>,
    output:usize,
    proc:[fn(VirtualMachine,usize) -> VirtualMachine;8]
}

fn combo_op(vm:&VirtualMachine, op:usize) -> usize {
    match op {
        0|1|2|3 => op,
        4 => vm.state.a,
        5 => vm.state.b,
        6 => vm.state.c,
        _ => panic!("should never pass here"),
    }
}

fn adv(mut vm:VirtualMachine, op:usize) -> VirtualMachine {
    vm.state.a /= 1 << combo_op(&vm,op );
    vm
}

fn bxl(mut vm:VirtualMachine, op:usize) -> VirtualMachine {
    vm.state.b ^= op;
    vm
}

fn bst(mut vm:VirtualMachine, op:usize) -> VirtualMachine {
    vm.state.b = combo_op(&vm, op) % 8;
    vm
}

fn jnz(mut vm:VirtualMachine, op:usize) -> VirtualMachine {
    if vm.state.a != 0 {
        vm.state.p = op;
    }
    vm
}

fn bxc(mut vm:VirtualMachine, _op:usize) -> VirtualMachine {
    vm.state.b ^= vm.state.c;
    vm
}

fn out(mut vm:VirtualMachine, op:usize) -> VirtualMachine {
    vm.output *= 10;
    vm.output += combo_op(&vm, op) % 8; 
    vm
}

fn bdv(mut vm:VirtualMachine, op:usize) -> VirtualMachine {
    vm.state.b = vm.state.a / 1 << combo_op(&vm,op );
    vm
}

fn cdv(mut vm:VirtualMachine, op:usize) -> VirtualMachine {
    vm.state.c = vm.state.a / 1 << combo_op(&vm,op );
    vm
}


impl VirtualMachine { 
    fn build(input:Vec<usize>) -> VirtualMachine {
        let mut prog = Vec::new();
        let proc = [adv, bxl, bst, jnz, bxc, out, bdv, cdv];

        for i in &input[3..] {
            prog.push(*i);
        }
        VirtualMachine{state:State{a:input[0], b:input[1], c:input[2], p:0}, prog, output:0, proc}
    }

    fn run(mut self) -> usize {
        loop {
            let inst = self.prog[self.state.p];
            let op = self.prog[self.state.p+1];
            self.state.p+=2;
            self = self.proc[inst](self, op);
            if self.state.p >= self.prog.len() {
                break;
            }
            println!("{:?}", self);
        }
        self.output
    }
    

}

pub fn solve(_step:usize, input:String) -> usize {
    let vm = VirtualMachine::build(parse_usize(&input));
    println!("Start : {:?}", vm);
    vm.run()
}