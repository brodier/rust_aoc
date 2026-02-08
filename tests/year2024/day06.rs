use aoc::year2024::day06::*;

const EXAMPLE:&str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";


#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE.to_string())), "41");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE.to_string())), "6");
}
