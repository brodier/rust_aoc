use aoc::year2024::day21::*;

const EXAMPLE:&str = "\
029A
980A
179A
456A
379A";




#[test]
fn part1_test() {
    assert_eq!(solve(1,EXAMPLE.to_string()),"126384");
}