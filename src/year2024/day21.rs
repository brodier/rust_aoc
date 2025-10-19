
// 1 construire la map des mouvement mimaux en fonction des transition pour numerical key pad
// 2 construire la map des mouvement mimaux en fonction des transition pour le dir key pad
// 3 pour chaque transition numérique minal deterimner les transition directionnal minimal
// pour chaque transition d'une touche vers une autre les mouvement mimimaux possible sont 
// A -> 0, A -> 1, .., A -> 9
// 0 -> A, 0 -> 1, .., 0 -> 9
// 1 -> A, 1 -> 0, .., 1 -> 9

use core::str;
use std::collections::HashMap;
use crate::utils::common::parse_usize;

const NUMERIC_KEY_PAD:&str = "\
789
456
123
 0A";

const DIR_KEY_PAD:&str ="\x20^A
<v>";

const RIGHT:char = '>';
const LEFT:char = '<';
const UP:char = '^';
const DOWN:char = 'v';
const ACT:char = 'A';

#[derive(Debug)]
struct KeyPad {
    pad:Vec<String>,
    map:KeyMap
}

type KeyMap = HashMap<(char,char),(String, Option<String>)>;

impl KeyPad {
    fn build(input:&str) -> KeyPad {
        let mut pad = Vec::new();
        let map = HashMap::new();
        for l in input.lines() {
            pad.push(l.to_string());
        }
        let mut key_pad = KeyPad{pad, map};
        key_pad.init_map();
        return key_pad;
    }
    
    fn get_key(&self, x:usize,y:usize) -> char {
        self.pad.get(y).unwrap().chars().nth(x).unwrap()
    }

    fn build_seq(h:char,v:char,nbh:usize,nbv:usize,vf:bool) -> String {
        let (f,s, nbf) = if vf { (v,h,nbv) } else { (h,v,nbh) };
        let mut seq = vec![f;nbh+nbv+1];
        for e in seq[nbf..nbh+nbv].as_mut() {
            *e=s;
        }
        seq[nbh+nbv]=ACT;
        seq.into_iter().collect()
    }
    
    fn init_map(&mut self) {
        let height = self.pad.len();
        let width = self.pad.first().unwrap().len();
        for start_x in 0..width as usize {
            for start_y in 0..height {
                for to_x in 0..width {
                    for to_y in 0..height {
                        if width*to_y+to_x < width*start_y+start_x {
                            continue;
                        }
                        let from_key = self.get_key(start_x, start_y);
                        let to_key = self.get_key(to_x, to_y);
                        if from_key == ' ' || to_key == ' ' {
                            continue;
                        }
    
                        // define direct direction
                        let (direct_hor, rev_hor, vert_first) = if start_x > to_x { (LEFT,RIGHT, true) } else { (RIGHT,LEFT, false) };
                        let (direct_ver,rev_ver) = if start_y > to_y { (UP,DOWN) } else { (DOWN,UP) };
    
                        let direct_seq:String;
                        let revert_seq:String;
                        let direct_seq2:Option<String>;
                        let revert_seq2:Option<String>;
                        let nb_right = to_x.abs_diff(start_x);
                        let nb_down = to_y.abs_diff(start_y);
                        if nb_right > 0 && nb_down > 0 {
                            direct_seq = KeyPad::build_seq(direct_hor, direct_ver, nb_right,nb_down, vert_first);
                            revert_seq = KeyPad::build_seq(rev_hor, rev_ver, nb_right, nb_down, !vert_first);
                            if ((start_x,to_y) == (0,3)) || (from_key == '<' || to_key == '<') {
                                direct_seq2 = None;
                                revert_seq2 = None;
                            } else {
                                direct_seq2 = Some(KeyPad::build_seq(direct_hor, direct_ver, nb_right,nb_down, !vert_first));
                                revert_seq2 = Some(KeyPad::build_seq(rev_hor, rev_ver, nb_right, nb_down, vert_first));
                            }
                        } else if nb_right > 0 {
                            let mut seq = vec![direct_hor;nb_right+1];
                            seq[nb_right]=ACT;
                            direct_seq = seq.into_iter().collect();
                            let mut seq = vec![rev_hor;nb_right+1];
                            seq[nb_right]=ACT;
                            revert_seq = seq.into_iter().collect();
                            direct_seq2 = None;
                            revert_seq2 = None;
                        } else if nb_down > 0 {
                            let mut seq = vec![direct_ver;nb_down+1];
                            seq[nb_down]=ACT;
                            direct_seq = seq.into_iter().collect();
                            let mut seq = vec![rev_ver;nb_down+1];
                            seq[nb_down]=ACT;
                            revert_seq = seq.into_iter().collect();
                            direct_seq2 = None;
                            revert_seq2 = None;
                        } else {
                            direct_seq = "A".to_string();
                            revert_seq = "A".to_string();
                            direct_seq2 = None;
                            revert_seq2 = None;
                        }
                        self.map.insert((from_key,to_key), (direct_seq,direct_seq2));
                        self.map.insert((to_key,from_key), (revert_seq,revert_seq2));
                    }
                }
            }
        }
    }

}


fn compute_transition(tmap:&mut HashMap<(char,char,usize),usize>, cmd:&str, depth:usize, keypad:&KeyMap) -> usize {
    if depth == 0 {
        return cmd.len();
    }
    let mut from='A';
    let mut len = 0;
    //println!("searching Transition for seq {} on depth {}", cmd,depth);
    for to in cmd.chars() {
        //println!("searching Transition ({},{},{})", from,to,depth);
        len += tmap.get(&(from,to,depth)).copied().unwrap_or_else(|| {
            let (first_seq,opt_sec_seq) = keypad.get(&(from, to)).unwrap();
            let mut sublen = compute_transition(tmap, first_seq, depth - 1, keypad);
            if let Some(opt_cmd) = opt_sec_seq {
                let opt_len = compute_transition(tmap, &opt_cmd, depth - 1, keypad);
                if opt_len < sublen {
                    sublen = opt_len;
                }
            }
            //println!("updating tmap with ({},{},{}=>{})", from,to,depth, sublen);
            tmap.insert((from,to,depth), sublen);
            sublen
        });
        from = to;
    }
    //println!("Transition for seq {} on depth {} minimal cmd length is {}", cmd, depth, len);
    len
}

pub fn solve(part:usize, input:String) -> String {
    let num_key_pad = KeyPad::build(NUMERIC_KEY_PAD);
    let dir_keypad = KeyPad::build(DIR_KEY_PAD);
    let mut keypad = num_key_pad.map.clone();
    dir_keypad.map.iter().map(|(k,v)| keypad.insert(*k, v.clone())).count();
    //println!("num_key_pad map : {:?}", num_key_pad.map);
    //println!("dir_key_pad map : {:?}", dir_keypad.map);
    let depth = if part == 1 { 2 }  else { 25 };
    let mut tmap = HashMap::new();
    let mut result = 0;
    for code in input.lines() {
        result += parse_usize(code).first().unwrap() * compute_transition(&mut tmap, code, depth+1, &keypad);
    }
    result.to_string()
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