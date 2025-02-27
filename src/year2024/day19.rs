
struct Puzzle<'a> {
    towels_trie:Node,
    designs:Vec<&'a str>,
}

fn color_to_index(c:char) -> usize {
    match c {
        'w' => 0,
        'u' => 1,
        'b' => 2,
        'r' => 3,
        'g' => 4,
        _ => panic!("Invalid params"),
    }
}
#[derive(Debug,Clone)]
struct Node {
    next:Vec<Option<Box<Node>>>,
    towel:bool
}

impl Node {
    fn new() -> Node {
        Node{next:vec![None,None,None,None,None], towel:false}
    }
}
impl Puzzle<'_> {
    fn build(input:&str) -> Puzzle<'_> {
        let mut lines = input.lines();
        let towel_patterns:Vec<&str> = lines.next().unwrap().split(", ").collect();
        let mut root = Node::new();
        for towel in towel_patterns {
            let mut curr_node = &mut root;
            for b in towel.as_bytes() {
                let color = color_to_index(*b as char);
                if curr_node.next[color].is_none() {
                    curr_node.next[color] = Some(Box::new(Node::new()));
                } 
                curr_node = curr_node.next[color].as_mut().unwrap();
            }
            curr_node.towel = true;
        }
        lines.next(); // skip empty line
        let designs:Vec<&str> = lines.collect();
        //println!("trie : {:?}", root);
        Puzzle{towels_trie:root, designs}
    }
    
    fn solve(&self, part:usize) -> usize {
        let mut result = 0;
        for &design in self.designs.iter() {
            // println!("Starting research for design [{}]", design);
            let mut nb_config = [0;1000].to_vec();
            nb_config.resize(design.len()+1, 0);
            nb_config[0]=1;
            for idx in 0..design.len() {
                let mut curr_node = &self.towels_trie;            
                if nb_config[idx]>0 {
                    for n in idx..design.len() {
                        let digit = design.as_bytes()[n] as char;
                        let child = curr_node.next[color_to_index(digit)].as_ref();
                        if child.is_none() {
                            // println!("stop no child for {} on {} at {}, {:?}", design, digit, idx, nb_config);
                            break;
                        }
                        curr_node = child.unwrap().as_ref();
                        if curr_node.towel {
                            nb_config[n+1] += nb_config[idx];
                            // println!("update config for pattern {} on {}, {:?}", design, digit, nb_config);
                        } else {
                            // println!("unchanged config for pattern {} on {}, {:?}", design, digit, nb_config);
                        }
                    }
                }
            }
            result += if part == 1 && 
                nb_config[design.len()] > 0 {
                 1 
                } else {
                    nb_config[design.len()]  
                };
        }
        result
    }
}

pub fn solve(part:usize, input:String) -> String {

    let puzzle = Puzzle::build(&input);
    let result = puzzle.solve(part).to_string();
    println!("result part{} {}", part, result);
    result
    
}