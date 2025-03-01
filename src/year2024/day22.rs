use std::collections::HashMap;
use crate::utils::common::parse_usize;

const BUYER_PROC:[fn(usize)->usize;3] = [|u| u << 6, |u| u >> 5, |u| u << 11];
const MODULO_VAL:usize = 16777216;

fn mix_and_prune_on(op:&fn(usize)->usize, secret:usize) -> usize {
    (op(secret) ^ secret) % MODULO_VAL
}

pub fn next_secret(mut secret:usize) -> usize {
    BUYER_PROC.iter().for_each(|op| secret = mix_and_prune_on(op, secret));
    secret
}

fn solve_part1(input:&Vec<usize>) -> usize {
    let mut result = 0;
    for initial_secret in input {
        let mut secret = *initial_secret;
        for _ in 0..2000 {
            secret = next_secret(secret);
        }
        result += secret;
    }
    result
}

fn sub(a:u8,b:u8)-> i8 {
    a as i8 - b as i8
}

type MonkeySeq = (i8,i8,i8,i8);

fn updat_seq((_,b,c,d):MonkeySeq, e:i8) -> MonkeySeq {
    (b,c,d,e)
}

fn merge_hashmap_by_sum(from:&HashMap<MonkeySeq,usize>, to:& mut HashMap<MonkeySeq,usize>) -> (usize,MonkeySeq) {
    let mut max = (0,(0,0,0,0));
    from.iter().for_each( |(&k,&v)|             {
        let old_val = to.remove(&k).unwrap_or_default();
        let new_val = v + old_val;
        to.insert(k, v+old_val);
        if new_val > max.0 {
            max = (new_val,k);
        }
    });
    max
}

pub fn solve(part:usize, input:String) -> String {
    let init_secrets = parse_usize(&input);
    if part==1 {
        return solve_part1(&init_secrets).to_string();
    }

    let mut global_seq_eval:HashMap<MonkeySeq,usize> = HashMap::new();
    let mut max= (0,(0,0,0,0));
    for init_secret in init_secrets {
        let mut seq_eval:HashMap<MonkeySeq,usize> = HashMap::new();
        let mut cur_seq:MonkeySeq = (0,0,0,0);
        let mut previous_price:u8;
        let mut secret = init_secret;
        let mut price:u8 = (secret % 10) as u8;
        for i in 0..2000 {
            previous_price = price;
            secret = next_secret(secret);
            price = (secret % 10) as u8;
            let change = sub(price,previous_price);
            cur_seq = updat_seq(cur_seq, change);
            if i > 2 {
                if !seq_eval.contains_key(&cur_seq) {
                    seq_eval.insert(cur_seq, price as usize);
                }
            }
        }
        if global_seq_eval.len() == 0 {
            global_seq_eval = seq_eval;
        } else {
            let new_max = merge_hashmap_by_sum(&seq_eval, &mut global_seq_eval);
            if new_max.0 > max.0 {
                max = new_max;
            }
        }
    }
    max.0.to_string()
}