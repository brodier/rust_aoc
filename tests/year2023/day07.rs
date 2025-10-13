use aoc::year2023::day07::*;

const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "6440");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "5905");
}