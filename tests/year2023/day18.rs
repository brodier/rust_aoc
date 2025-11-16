use aoc::year2023::day18::*;

const EXAMPLE: &str = "\
";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE.to_string())), "1");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE.to_string())), "2");
}
