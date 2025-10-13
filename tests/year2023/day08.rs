use aoc::year2023::day08::*;

const EXAMPLE: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const EXAMPLE2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";


#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "2");
    assert_eq!(solve(1, EXAMPLE2.to_string()), "6");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "2");
}