use aoc::year2024::day18::*;

const EXAMPLE:&str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";


#[test]
fn part1_test() {
    assert_eq!(solve(1,EXAMPLE.to_string()), "22");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "6,1");
}
