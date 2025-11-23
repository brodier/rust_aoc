use std::{collections::VecDeque, iter::repeat_with};

type ParseResult = Machine;
const BROADCASTER:&str = "broadcaster";
const TARGET_LABEL:&str = "rx";

#[derive(Clone,Debug)]
enum Module {
    FlipFlop(bool), // FlipFlop as a boolean state and a list of (target module address, entry number of the module
    Conjonction(Vec<bool>),    
    Broadcaster
}

#[derive(Clone,Debug)]
pub struct Machine {
    modules:Vec<Module>,
    links:Vec<Vec<(usize,usize)>>,
    broadcaster:usize,
}

impl Machine {
    fn press_button(&mut self, nb_iter:usize) -> (usize,usize) {
        let mut total_low_counter = 0;
        let mut total_high_counter = 0;
        for _ in 0..nb_iter {
            let (low_counter,high_counter, _) = self.press_button_once();
            (total_low_counter,total_high_counter) = (total_low_counter + low_counter,total_high_counter + high_counter)
        }
        (total_low_counter,total_high_counter)
    }

    fn press_button_once(&mut self) -> (usize,usize, bool) {
        let mut low_counter = 1;
        let mut high_counter = 0;
        let mut target_counter = (0,0);
        let mut pulses = VecDeque::new();
        pulses.push_back((false, 0, self.broadcaster)); // send low pulse on 0 entry of broadcaster
        while let Some((is_high,on_entry, module_addr)) = pulses.pop_front() {
            let m = &mut self.modules[module_addr];
            if let Some(is_high) = m.send_pulse(on_entry, is_high) {
                let dests = &self.links[module_addr];
                let counter= if is_high { &mut high_counter } else {&mut low_counter};
                *counter += dests.len();
                for (dest_module, module_entry) in dests.iter() {
                    if *dest_module < self.modules.len() {
                        pulses.push_back((is_high, *module_entry, *dest_module));
                    } else if *dest_module == self.modules.len() {
                        // println!("sending {} to target", if is_high {"high"} else {"low"});
                        if is_high {
                            target_counter.1 += 1;
                        } else {
                            target_counter.0 += 1;
                        }
                    }
                }
            }
        }
        // println!("target_counter {:?}", target_counter);
        (low_counter,high_counter, target_counter.0 == 1)
    }
    
    fn _get_state(&self) ->  Vec<bool> {
        let mut machine_state = Vec::new();
        for m in self.modules.iter() {
            match m {
                Module::FlipFlop(b) => machine_state.push(*b),
                Module::Conjonction(s) => s.iter().for_each(|cs| machine_state.push(*cs)),
                Module::Broadcaster => {},
            }
        }
        machine_state
    }
}
impl Module {
    fn send_pulse(&mut self, entry_number:usize, is_high:bool) -> Option<bool> {
        match self {
            &mut Module::Broadcaster => Some(false),
            &mut Module::FlipFlop(ref mut state) => if is_high { None } else { *state = !*state; Some(*state)},
            &mut Module::Conjonction(ref mut states) => {
                states[entry_number] = is_high;
                let mut high_pulse = false;
                for l in states { if !(*l) { high_pulse = true; break; } }
                Some(high_pulse)
            }
        }
    }
}

#[derive(Debug)]
struct LinkData<'a> {
    labels:Vec<&'a str>,
    links:Vec<(&'a str, usize)>
}

impl <'a>LinkData<'a> {
    fn new() -> Self {
        LinkData{labels: Vec::new(), links: Vec::new()}        
    }

    fn declare(&mut self, label:&'a str) {
        self.labels.push(label);
    }

    fn to_link(&mut self, dest:&'a str, module_address:usize) {
        self.links.push((dest, module_address));   
    }

    fn get_address(&self, dest:&str) -> Option<usize> {
        self.labels.iter().position(|&label| label == dest)
    }

    fn link(&self, mut modules:Vec<Module>) -> Machine {
        let mut links:Vec<Vec<(usize,usize)>> = repeat_with(Vec::new).take(modules.len()).collect();
        let mut link_by_label_counter:Vec<usize> = vec![0;self.labels.len()];
        let target_address = modules.len();
        let unknown_address = modules.len()+1;
        for (dest, module_address) in self.links.iter() {
            let dest_addr = self.get_address(*dest);
            let dest_addr = if dest_addr.is_some() {
                dest_addr.unwrap()
            } else if *dest==TARGET_LABEL {
                target_address
            } else {
                unknown_address
            };
            if dest_addr < modules.len() {
                links[*module_address].push((dest_addr, link_by_label_counter[dest_addr]));
                link_by_label_counter[dest_addr] += 1;
                // update state vec for Conjonction
                if let Module::Conjonction(states ) =  &mut modules[dest_addr] {
                    states.push(false); 
                }
            } else {
                links[*module_address].push((dest_addr, 0));
            }
        }
        Machine{modules, links, broadcaster: self.get_address(BROADCASTER).unwrap()}
    }
}

fn compile<'a>(input:&'a str) -> (Vec<Module>, LinkData<'a>) {
    let mut modules:Vec<Module> = Vec::new();
    let mut links_data = LinkData::new();
    for module_spec in input.lines() {
        let (module,dests) = module_spec.split_once(" -> ").unwrap();
        let dests:Vec<&str> = dests.split(", ").collect();
        if module.starts_with("%") || module.starts_with("&") {
            links_data.declare(&module[1..]);
            for dest in dests {
                links_data.to_link(dest, modules.len());
            }
            if module.starts_with("%") {
                modules.push(Module::FlipFlop(false));
            } else {
                modules.push(Module::Conjonction(Vec::new()));
            }
        } else {
            assert_eq!(module, BROADCASTER);
            links_data.declare(module);
            for dest in dests {
                links_data.to_link(dest, modules.len());
            }
            modules.push(Module::Broadcaster);
        }
    }
    (modules,links_data)
}

pub fn parse(input:String) -> ParseResult {
    let (program,link_data) = compile(&input);
    // println!("program:\n{:?}\nlinkdata:\n{:?}\n", program, link_data);
    link_data.link(program)
}

pub fn part1(m:&Machine) -> String {
    let mut machine = m.clone();
    let (nb_low_pulse, nb_high_pulse) = machine.press_button(1000);
    (nb_low_pulse * nb_high_pulse).to_string()
}

pub fn part2(m:&ParseResult) -> String {
    return "2".to_string();
    let mut machine = m.clone();
    let mut counter = 0;
    let mut press = true;
    while press {
        let (_,_,found)  = machine.press_button_once();
        counter += 1;
        // println!("{} {} {:?}", counter, found, machine);
        press = !found;
    }
    counter.to_string()
}
 
