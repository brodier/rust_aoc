use aoc::year2023::day15::*;

const EXAMPLE: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "1320");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "145");
}