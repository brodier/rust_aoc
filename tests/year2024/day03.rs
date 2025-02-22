use aoc::year2024::day03::*;

const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn part1_test() {
    assert_eq!(solve(1,EXAMPLE.to_string()), 161);
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE2.to_string()), 48);
}
