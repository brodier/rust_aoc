use aoc::year2024::day20::*;

const EXAMPLE:&str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";


#[test]
fn part1_test() {
    let puzzle = Puzzle::build(EXAMPLE.to_string());
    assert_eq!(puzzle.solve(20, 2).to_string(), "5");
}

#[test]
fn part2_test() {
    let puzzle = Puzzle::build(EXAMPLE.to_string());
    assert_eq!(puzzle.solve(74, 20).to_string(), "7");
    assert_eq!(puzzle.solve(72, 20).to_string(), "29");
}
