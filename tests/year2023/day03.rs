use aoc::year2023::day03::*;

const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "4361");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "467835");
}