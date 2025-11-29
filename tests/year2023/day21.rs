use aoc::year2023::day21::*;

const EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[test]
fn part1_test() {
    let mut p = parse(EXAMPLE.to_string());
    p.set_nb_step(6);
    assert_eq!(part1(&p), "16");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE.to_string())), "2");
}
