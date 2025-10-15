use aoc::year2023::day10::*;

const EXAMPLE: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

const EXAMPLE2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

const EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

const EXAMPLE4: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

const EXAMPLE5: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";


#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "4");
    assert_eq!(solve(1, EXAMPLE2.to_string()), "8");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "1");
    assert_eq!(solve(2,EXAMPLE2.to_string()), "1");
    assert_eq!(solve(2,EXAMPLE3.to_string()), "4");
    assert_eq!(solve(2,EXAMPLE4.to_string()), "4");
    assert_eq!(solve(2,EXAMPLE5.to_string()), "8");
}