use aoc::year2024::day01::*;

const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "11");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "31");
}