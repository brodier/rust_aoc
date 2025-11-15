use aoc::year2023::day17::*;

const EXAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

const EXAMPLE2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991";


#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE.to_string())), "102");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE.to_string())), "94");
}


#[test]
fn part2bis_test() {
    assert_eq!(part2(&parse(EXAMPLE2.to_string())), "71");
}