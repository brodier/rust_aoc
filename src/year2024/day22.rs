use std::{collections::HashMap, sync::atomic::AtomicUsize, thread::scope};
use crate::utils::common::parse_usize;

const BUYER_PROC:[fn(usize)->usize;3] = [|u| u << 6, |u| u >> 5, |u| u << 11];
const MODULO_VAL:usize = 16777216;

#[inline]
fn mix_and_prune_on(op:&fn(usize)->usize, secret:usize) -> usize {
    (op(secret) ^ secret) % MODULO_VAL
}

pub fn next_secret(mut secret:usize) -> usize {
    BUYER_PROC.iter().for_each(|op| secret = mix_and_prune_on(op, secret));
    secret
}

fn parallel_solving_part1(id_thread:usize, nb_threads:usize, total:&AtomicUsize, input:&Vec<usize>) {
    let mut result = 0;
    for initial_secret in (id_thread..input.len()).step_by(nb_threads) {
        let mut secret = input[initial_secret];
        for _ in 0..2000 {
            secret = next_secret(secret);
        }
        result += secret;
    }
    total.fetch_add(result, std::sync::atomic::Ordering::Relaxed);
}

fn solve_part1(input:&Vec<usize>) -> usize {
    let nb_threads = std::thread::available_parallelism().unwrap().get();
    let result = AtomicUsize::new(0);
    let total = &result;
    scope(|scope|{
        for i in 0..nb_threads {
            scope.spawn(move || parallel_solving_part1(i, nb_threads, total, input));
        }
    });
    result.into_inner()
}

fn update_seq(from:usize, with:usize) -> usize {
    (from << 8) & 0xffffff00 | (with & 0xff)
}

pub fn solve(part:usize, input:String) -> String {
    let init_secrets = parse_usize(&input);
    if part==1 {
        return solve_part1(&init_secrets).to_string();
    }

    let mut total_map:HashMap<usize,usize> = HashMap::new();
    for init_secret in init_secrets {
        let mut seq_map:HashMap<usize,usize> = HashMap::new();
        let mut cur_seq = 0;
        let mut previous_price:i8;
        let mut secret = init_secret;
        let mut price:i8 = (secret % 10) as i8;
        for i in 0..2000 {
            previous_price = price;
            secret = next_secret(secret);
            price = (secret % 10) as i8;
            let change = price - previous_price;
            cur_seq = update_seq(cur_seq, change as usize);
            if i > 2 {
                if !seq_map.contains_key(&cur_seq) {
                    seq_map.insert(cur_seq, price as usize);
                    let global_val = total_map.get_mut(&cur_seq);
                    if global_val.is_some() {
                        *(global_val.unwrap()) += price as usize;
                    } else {
                        total_map.insert(cur_seq, price as usize);
                    }
                } 
            }
        }
    }
    let (&_best_seq,&total) = total_map.iter().max_by(|seq1,seq2| seq1.1.cmp(seq2.1)).unwrap();
    total.to_string()
}