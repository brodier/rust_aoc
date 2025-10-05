use crate::utils::common::parse_usize;

const MAX_CARD: usize = 201;

#[derive(Debug)]
struct Puzzle {
   cards:Vec<Card>,
   result1:usize,
   result2:usize,
}

#[derive(Debug)]
struct Card {
    winning_numbers:Vec<usize>,
    draw_numbers:Vec<usize>,
}

impl Puzzle {
    fn build(lines: &str) -> Puzzle {
        let mut cards:Vec<Card> = Vec::new();
        for line in lines.lines() {
            let (_,rest) = line.split_once(":").unwrap();
            let (winning,draw) = rest.split_once("|").unwrap();
            let mut winning_numbers = parse_usize(winning);
            winning_numbers.sort();
            let mut draw_numbers = parse_usize(draw);
            draw_numbers.sort();
            let card = Card{winning_numbers,draw_numbers};
            cards.push(card);
        }
        Puzzle{cards,result1:0,result2:0}
    }

    fn eval(&mut self) {
        let mut score = 0;
        let mut nb_copies:[usize;MAX_CARD] = [0; MAX_CARD];

        for i in 0..self.cards.len() {
            let nb_cards_i = nb_copies[i] + 1;
            let card = &self.cards[i];
            let mut w = 0;
            let mut d = 0;
            let mut card_score:u32 = 0;
            while w < card.winning_numbers.len() && d < card.draw_numbers.len() {
                if card.winning_numbers[w] == card.draw_numbers[d] {
                    w += 1;
                    d += 1;
                    card_score += 1;
                } else if card.winning_numbers[w] < card.draw_numbers[d] {
                    w += 1;
                } else {
                    d += 1;
                }
            }
            if card_score > 0 {
                let from = i+1;
                let to = from+card_score as usize;
                for j in from..to {
                    nb_copies[j] += nb_cards_i;
                }
            }
            // eprintln!("Card {:?} score {}", card, card_score);
            if card_score > 0{
                score += 2_usize.pow(card_score - 1);
            }
            
        }
        self.result1 = score;
        self.result2 = nb_copies.iter().sum::<usize>() + self.cards.len();
    }
}

pub fn solve(step: usize, input: String) -> String {
    let mut puzzle = Puzzle::build(&input);
    puzzle.eval();
    if step == 1 {
        puzzle.result1.to_string()
    } else {
        puzzle.result2.to_string()
    }
}
