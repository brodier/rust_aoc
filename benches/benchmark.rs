#![allow(unstable_features)]
#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        mod $year {$(
            mod $day {
                use aoc::$year::$day::*;
                use aoc::utils::common::*;
                use std::fs::read_to_string;
                use std::sync::LazyLock;
                use test::Bencher;

                static DATA: LazyLock<String> = LazyLock::new(|| {
                    let year = stringify!($year);
                    let day = stringify!($day);
                    let path = format!("puzzle/{year}/{day}.txt");

                    read_to_string(&path).unwrap_or_else(|_| {
                        panic!("Missing input file {BOLD}{WHITE}{path}{RESET}");
                    })
                });

                #[bench]
                fn part1_bench(b: &mut Bencher) {
                    b.iter(|| solve(1, DATA.to_string()));
                }

                #[bench]
                fn part2_bench(b: &mut Bencher) {
                    b.iter(|| solve(2, DATA.to_string()));                }
            }
        )*}
    }
}

benchmark!(year2023
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15
);

benchmark!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);