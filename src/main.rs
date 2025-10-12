//! [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
//!
//! These codes allow command line applications to show colored or styled text in most terminals.
//! Advanced commands can move the cursor or clear the screen.

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[94m";
pub const WHITE: &str = "\x1b[97m";
pub const HOME: &str = "\x1b[H";
pub const CLEAR: &str = "\x1b[J";


use std::env::args;
use std::iter::empty;
use std::time::{Duration, Instant};
use aoc::utils::common::load_puzzle;
use aoc::utils::common::parse_usize;
use aoc::*;


fn main() {
        // Parse command line options
        let (year, day) = match args().nth(1) {
            Some(arg) => {
                let args = parse_usize(&arg);
                if args.len() == 0 {
                    (None,None)
                } else if args.len()==1 {
                    (Some(args[0]),None)    
                } else {
                    (Some(args[0]),Some(args[1]))
                }
            }
            None => (None, None),
        };
        // Pretty print output and timing for each solution
        let mut solved = 0;
        let mut duration = Duration::ZERO;
        // Filter solutions
        let solutions = empty()
            .chain(year2023())
            .chain(year2024())
            .filter(|solution| year.is_none_or(|y: usize| y == solution.year))
            .filter(|solution| day.is_none_or(|d: usize| d == solution.day));
    
            for Solution { year, day, wrapper } in solutions {
                if let Ok(data) = load_puzzle(year, day) {
                    let instant = Instant::now();
                    let part1 = wrapper(1, data.clone());
                    let elapsed1 = instant.elapsed();
                    let part2 =  wrapper(2,data);
                    let elapsed = instant.elapsed();
                    let elapsed2 = elapsed - elapsed1;
                    solved += 1;
                    duration += elapsed;
        
                    println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
                    println!("    Part 1: {part1}");
                    println!("    Part 2: {part2}");
                    println!("    Elapsed: {} μs ( part1 {} µs, part2 {} µs ) ", elapsed.as_micros(), elapsed1.as_micros(), elapsed2.as_micros());
                } else {
                    eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
                    eprintln!("    Missing input!");
                    eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", format!("puzzle/year{}/day{:02}.txt",year,day));
                }
            }
    // Print totals
    println!("{BOLD}{RED}Solved: {solved}{RESET}");
    println!("{BOLD}{GREEN}Duration: {} ms{RESET}", duration.as_millis());
}


struct Solution {
    year: usize,
    day: usize,
    wrapper: fn(usize,String) -> String,
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = parse_usize(stringify!($year))[0];
                let day = parse_usize(stringify!($day))[0];
                let wrapper = $year::$day::solve;

                Solution { year: year, day: day, wrapper }
            },)*]
        }
    }
}

run!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2023
    day01, day02, day03, day04, day05, day06, day07
);