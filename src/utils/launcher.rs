use std::fs::read_to_string;
use std::iter::empty;
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};

use crate::utils::common::*;


struct Solution {
    year: usize,
    day: usize,
    path: PathBuf,
    wrapper: fn(String) -> (String, String, Duration, Duration, Duration),
}


pub fn launch(year:Option<usize>,day:Option<usize>) {
    // Pretty print output and timing for each solution
    let mut solved = 0;
    let mut duration = Duration::ZERO;
    // Filter solutions
    let solutions = empty()
        .chain(year2023())
        .chain(year2024())
        .filter(|solution| year.is_none_or(|y: usize| y == solution.year))
        .filter(|solution| day.is_none_or(|d: usize| d == solution.day));

    for Solution {
        year,
        day,
        path,
        wrapper,
    } in solutions
    {
        if let Ok(data) = read_to_string(&path) {
            let timer = Instant::now();
            let (part1, part2, init_time, part1_time, part2_time) = wrapper(data);
            duration += timer.elapsed();
            let (t1, t2, t3) = (
                init_time.as_micros(),
                part1_time.as_micros(),
                part2_time.as_micros(),
            );
            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Init... ({t1} us)");
            println!("    Part 1: {part1} ({t2} us)");
            println!("    Part 2: {part2} ({t3} us)");
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!(
                "    Place input file in {BOLD}{WHITE}{}{RESET}",
                path.display()
            );
        }
        solved += 1;
    }
    // Print totals
    println!("{BOLD}{RED}Solved: {solved}{RESET}");
    println!("{BOLD}{GREEN}Duration: {} ms{RESET}", duration.as_millis());
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("puzzle").join(year).join(day).with_extension("txt");

                let wrapper = |data: String| {
                    use crate::$year::$day::*;
                    let t0 = Instant::now();
                    let input = parse(data);
                    let t1 = t0.elapsed();
                    let part1 = part1(&input);
                    let t2 = t0.elapsed();
                    let part2 = part2(&input);
                    let t3 = t0.elapsed();

                    (part1.to_string(), part2.to_string(), t1, t2 - t1, t3 - t2)
                };

                Solution { year: *parse_usize(year).first().unwrap(), day: *parse_usize(day).first().unwrap(), path, wrapper }
            },)*]
        }
    }
}

run!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(year2023
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21
);
