use std::cmp::Ordering;

use crate::utils::common::parse_usize;

#[derive(Debug)]
struct Puzzle<'a> {
    hands:Vec<(&'a str, &'a str)>
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn card_value(card:u8) -> u8 {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        b'9' => 9,
        b'8' => 8,
        b'7' => 7, 
        b'6' => 6,
        b'5' => 5,
        b'4' => 4,
        b'3' => 3,
        b'2' => 2,
        _ => panic!("invalid card symbol")
    }
}

fn qualify(hand:&str) -> HandType {
    let hand = hand.as_bytes();
    let mut tmp:[u8;13] = [0;13];
    for i in 0..5 {
        tmp[card_value(hand[i]) as usize - 2 as usize] += 1;
    }
    if tmp.contains(&5) {
        return HandType::Five;
    } else if tmp.contains(&4) {
        return HandType::Four;
    } else if tmp.contains(&3) {
        if tmp.contains(&2) {
            return HandType::FullHouse;
        } else {
            return HandType::Three;
        }
    } else {
        let mut has_one_pair = false;
        for v in tmp {
            if v == 2 && has_one_pair {
                return HandType::TwoPair;
            } else if v==2 {
                has_one_pair = true;
            }
        }
        if has_one_pair {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    } 
}

fn cmp_on_same_type(left:&str, right:&str) -> Ordering { 
    let left_hand = left.as_bytes();
    let right_hand = right.as_bytes();
    for i in 0..5 {
        let order = card_value(left_hand[i]).cmp(& card_value(right_hand[i]));
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

fn cmp_hand(left:&str, right:&str) -> Ordering {
    let cmp_type = qualify(left).cmp(&qualify(right));
    if cmp_type == Ordering::Equal {
        return cmp_on_same_type(left,right);
    } else {
        return cmp_type;
    }
}


impl Puzzle<'_> {
    fn build<'a>(input: &'a str) -> Puzzle<'a> {
        let mut hands = Vec::new();
        for line in input.lines(){
            let hand  = line.split_once(" ").unwrap();
            hands.push(hand);
        }
        hands.sort_by(|a,b| cmp_hand(a.0, b.0));
        Puzzle {hands}
    }

    fn solve1(&self) -> usize {
        let mut result = 0;
        for (i, &hand)  in self.hands.iter().enumerate() {
            let bid = hand.1.parse::<usize>().unwrap();
            eprintln!("hand {} has rank {} and bid {}", hand.0, i + 1, bid);
            result += (i+1) * bid;
        }
        result
    }

    fn solve2(&self) -> usize {
        0
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(&input);
    if step == 1 {
        p.solve1().to_string()
    } else {
        p.solve2().to_string()
    }
}


