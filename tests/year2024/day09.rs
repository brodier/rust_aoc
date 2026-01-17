use aoc::year2024::day09::*;


const EXAMPLE: &str = "2333133121414131402";

#[test]
fn part1_test() {
    assert_eq!(solve(1,EXAMPLE.to_string()), "1928");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "2858");
}
