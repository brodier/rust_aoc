use aoc::year2023::day01::*;

const EXAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";


const EXAMPLE_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "142");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE_2.to_string()), "281");
}