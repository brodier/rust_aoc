use aoc::year2023::day14::*;

const EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "136");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "64");
}
