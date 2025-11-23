use aoc::year2023::day20::*;

const EXAMPLE_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

const EXAMPLE_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";


#[test]
fn part1_1_test() {
    assert_eq!(part1(&parse(EXAMPLE_1.to_string())), "32000000");
}

#[test]
fn part1_2_test() {
    assert_eq!(part1(&parse(EXAMPLE_2.to_string())), "11687500");
}
