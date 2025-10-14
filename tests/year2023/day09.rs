use aoc::year2023::day09::*;

const EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "114");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "2");
}