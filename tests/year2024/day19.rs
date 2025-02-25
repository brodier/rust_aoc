use aoc::year2024::day19::*;

const EXAMPLE:&str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";


#[test]
fn part1_test() {
    assert_eq!(solve(1,EXAMPLE.to_string()), "6");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "16");
}
