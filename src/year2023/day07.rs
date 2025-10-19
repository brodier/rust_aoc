use std::cmp::Ordering;

#[derive(Debug)]
struct Puzzle<'a> {
    hands: Vec<(&'a str, &'a str)>,
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

fn qualify(step: usize, hand: &str) -> HandType {
    let hand = hand.as_bytes();
    let mut tmp: [u8; 14] = [0; 14];
    for i in 0..5 {
        tmp[card_value(step, hand[i]) as usize] += 1;
    }
    if step == 2 && tmp[0]  <  5 && tmp[0] > 0 {
        // increment the max same symbol with the number of joker tmp[0]
        let mut max = 0;
        let mut max_i = 0;
        for (i, v) in tmp[1..].iter().enumerate() {
            if *v > max {
                max_i = i + 1;
                max = *v;
            }
        }
        tmp[max_i] += tmp[0];
        tmp[0] = 0;
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
            } else if v == 2 {
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

fn cmp_on_same_type(step: usize, left: &str, right: &str) -> Ordering {
    let left_hand = left.as_bytes();
    let right_hand = right.as_bytes();
    for i in 0..5 {
        let order = card_value(step, left_hand[i]).cmp(&card_value(step, right_hand[i]));
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

fn cmp_hand(step: usize, left: &str, right: &str) -> Ordering {
    let cmp_type = qualify(step, left).cmp(&qualify(step, right));
    if cmp_type == Ordering::Equal {
        return cmp_on_same_type(step, left, right);
    } else {
        return cmp_type;
    }
}

fn card_value(step: usize, card: u8) -> u8 {
    match card {
        b'A' => 13,
        b'K' => 12,
        b'Q' => 11,
        b'J' => {
            if step == 1 {
                10
            } else {
                0
            }
        }
        b'T' => 9,
        b'9' => 8,
        b'8' => 7,
        b'7' => 6,
        b'6' => 5,
        b'5' => 4,
        b'4' => 3,
        b'3' => 2,
        b'2' => 1,
        _ => panic!("invalid card symbol"),
    }
}

impl Puzzle<'_> {
    fn build<'a>(step: usize, input: &'a str) -> Puzzle<'a> {
        let mut hands = Vec::new();
        for line in input.lines() {
            let hand = line.split_once(" ").unwrap();
            hands.push(hand);
        }
        hands.sort_by(|a, b| cmp_hand(step, a.0, b.0));
        Puzzle { hands }
    }

    fn solve(&self) -> String {
        let mut result = 0;
        for (i, &hand) in self.hands.iter().enumerate() {
            let bid = hand.1.parse::<usize>().unwrap();
            result += (i + 1) * bid;
        }
        result.to_string()
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
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