use aoc::year2024::day09::*;


const EXAMPLE: &str = "2333133121414131402";

#[test]
fn part1_test() {
    let file_map = parse(EXAMPLE.to_string());
    assert_eq!(part1(&file_map), "1928");
}

#[test]
fn part2_test() {
    let file_map = parse(EXAMPLE.to_string());
    assert_eq!(part2(&file_map), "2858");
}
