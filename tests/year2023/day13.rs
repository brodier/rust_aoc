use aoc::year2023::day13::*;

const EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "405");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "2");
}