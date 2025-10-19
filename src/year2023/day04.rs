use crate::utils::common::parse_usize;

const MAX_CARD_VAL: usize = 100;

#[derive(Debug)]
struct Puzzle {
   scores:Vec<usize>
}

#[derive(Debug)]
struct Card {
    winning_numbers:Vec<usize>,
    draw_numbers:Vec<usize>,
}

impl Card {
    fn eval(&self) -> usize {
        let mut founds = [false; MAX_CARD_VAL];
        self.winning_numbers.iter().for_each(|&card| founds[card] = true );
        self.draw_numbers.iter().filter(|&&card| founds[card]).count()
    }
}

impl Puzzle {
    fn build(lines: &str) -> Puzzle {
        let scores = lines.lines().map(|line| {
                let (_,rest) = line.split_once(":").unwrap();
                let (winning,draw) = rest.split_once("|").unwrap();
                let card = Card{winning_numbers: parse_usize(winning),draw_numbers: parse_usize(draw)};
                return card.eval();
            }).collect();
        Puzzle{scores}
    }

    fn result1(&self) -> usize {
        self.scores.iter().map(|&s| (1 << s) >> 1 ).sum()
    }

    fn result2(&self) -> usize {
        let mut copies = vec![1; self.scores.len()];
        for (i, &s) in self.scores.iter().enumerate() {
            (1..=s).for_each(|j| copies[i + j] += copies[i]);
        }
        copies.iter().sum()
    }
}

pub fn solve(step: usize, input: String) -> String {
    let puzzle = Puzzle::build(&input);
    if step == 1 {
        puzzle.result1().to_string()
    } else {
        puzzle.result2().to_string()
    }
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