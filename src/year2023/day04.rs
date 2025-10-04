use crate::utils::common::parse_usize;


#[derive(Debug)]
struct Puzzle {
   cards:Vec<Card>,
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
        Puzzle{cards}
    }

    fn eval(&mut self) -> usize {
        let mut score = 0;
        for card in self.cards.iter() {
            let mut w = 0;
            let mut d = 0;
            let mut card_score = 0;
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
            eprintln!("Card {:?} score {}", card, card_score);
            if card_score > 0 {
                score += 2_usize.pow(card_score - 1) as usize;
            }
        }
        score
    }
}

pub fn solve(_step: usize, input: String) -> String {
    let mut puzzle = Puzzle::build(&input);
    puzzle.eval().to_string()
}
