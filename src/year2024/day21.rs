
// 1 construire la map des mouvement mimaux en fonction des transition pour numerical key pad
// 2 construire la map des mouvement mimaux en fonction des transition pour le dir key pad
// 3 pour chaque transition numérique minal deterimner les transition directionnal minimal
// pour chaque transition d'une touche vers une autre les mouvement mimimaux possible sont 
// A -> 0, A -> 1, .., A -> 9
// 0 -> A, 0 -> 1, .., 0 -> 9
// 1 -> A, 1 -> 0, .., 1 -> 9

use core::{num, str};
use std::collections::HashMap;

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
    map:HashMap<(char,char),Vec<String>>
}

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
    
    fn init_map(&mut self) {
        let height = self.pad.len();
        let width = self.pad.first().unwrap().len();
        for start_x in 0..width as usize {
            for start_y in 0..height {
                for to_x in 0..width {
                    for to_y in 0..height {
                        if width*to_y+to_x < width*start_y+start_y {
                            continue;
                        }
                        let from_key = self.get_key(start_x, start_y);
                        let to_key = self.get_key(to_x, to_y);
                        if from_key == ' ' || to_key == ' ' {
                            continue;
                        }
    
                        // define horizontal direct direction
                        let direct_hor = if start_x > to_x { LEFT } else { RIGHT };
                        let rev_hor = if start_x > to_x { RIGHT } else { LEFT };
                        let direct_ver = if start_y > to_y { UP } else { DOWN };
                        let rev_ver = if start_y > to_y { DOWN } else { UP };
                        // define vertical direct direction
    
                        let mut direct_seq = Vec::new();
                        let mut revert_seq = Vec::new();
                        let nb_right = to_x.abs_diff(start_x);
                        let nb_down = to_y.abs_diff(start_y);
                        if nb_right > 0 && nb_down > 0 {
                            // only considere two option starting with right then down or starting with down then right
                            let mut seq: Vec<char> = vec![direct_hor;nb_right+nb_down+1];
                            for e in seq[nb_right..nb_right+nb_down].as_mut() {
                                *e=direct_ver;
                            }
                            seq[nb_right+nb_down]=ACT;

                            direct_seq.push(seq.into_iter().collect());
                            let mut seq = vec![direct_ver;nb_right+nb_down+1];
                            for e in seq[nb_down..nb_right+nb_down].as_mut() {
                                *e=direct_hor;
                            }
                            seq[nb_right+nb_down]=ACT;
                            direct_seq.push(seq.into_iter().collect());
                            let mut seq = vec![rev_hor;nb_right+nb_down+1];
                            for e in seq[nb_right..nb_right+nb_down].as_mut() {
                                *e=rev_ver;
                            }
                            seq[nb_right+nb_down]=ACT;
                            revert_seq.push(seq.into_iter().collect());
                            let mut seq = vec![rev_ver;nb_right+nb_down+1];
                            for e in seq[nb_down..nb_right+nb_down].as_mut() {
                                *e=rev_hor;
                            }
                            seq[nb_right+nb_down]=ACT;
                            revert_seq.push(seq.into_iter().collect());
                        } else if nb_right > 0 {
                            let mut seq = vec![direct_hor;nb_right+1];
                            seq[nb_right]=ACT;
                            direct_seq.push(seq.into_iter().collect());
                            let mut seq = vec![rev_hor;nb_right+1];
                            seq[nb_right]=ACT;
                            revert_seq.push(seq.into_iter().collect());
                        } else if nb_down > 0 {
                            let mut seq = vec![direct_ver;nb_down+1];
                            seq[nb_down]=ACT;
                            direct_seq.push(seq.into_iter().collect());
                            let mut seq = vec![rev_ver;nb_down+1];
                            seq[nb_down]=ACT;
                            revert_seq.push(seq.into_iter().collect());
                        } else {
                            revert_seq.push("A".to_string());
                        }
                        self.map.insert((from_key,to_key), direct_seq);
                        self.map.insert((to_key,from_key), revert_seq);
                    }
                }
            }
        }
    }
}

fn get_num_key(x:usize, y:usize) -> u8 {
        NUMERIC_KEY_PAD.lines().nth(y).unwrap().as_bytes()[x] as u8
}

fn init_dir_trans() -> HashMap<(u8,u8),&'static Vec<u8>> {
    HashMap::new()
}

pub fn solve(part:usize, input:String) -> String {
    let num_key_pad = KeyPad::build(NUMERIC_KEY_PAD);
    let dir_key_pad = KeyPad::build(DIR_KEY_PAD);
    println!("{:?}", num_key_pad);
    println!("{:?}", dir_key_pad);
    "123456".to_string()
}
