use aoc::year2024::day04::*;


const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn part1_test() {
    assert_eq!(solve(1,EXAMPLE.to_string()), "18");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "9");
}
