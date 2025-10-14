use aoc::year2023::day10::*;

const EXAMPLE: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

const EXAMPLE2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";


#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "4");
    assert_eq!(solve(1, EXAMPLE2.to_string()), "8");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "4");
}