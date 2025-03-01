use aoc::year2024::day22::*;

const EXAMPLE1:&str = "\
1
10
100
2024";

const EXAMPLE2:&str = "\
1
2
3
2024";


#[test]
fn next_secret_test() {
    assert_eq!(next_secret(123),15887950);
    assert_eq!(next_secret(15887950),16495136);
    assert_eq!(next_secret(16495136),527345);
    assert_eq!(next_secret(527345),704524);
}

#[test]
fn part1_test() {
    assert_eq!(solve(1,EXAMPLE1.to_string()),"37327623");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE2.to_string()),"23");
}