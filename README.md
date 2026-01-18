# [Advent of Code] Solutions [![checks-badge]][checks-link]

The wrapping for launching solver is largely inspired 
from [ManEatingApe AOC Rust Repository]

# Advent of Code 

The year 2024 is complete 
Currently playing with year 2023 [checks-link]

**Run**
* Everything `cargo run`
* Specific year `cargo run year2024`
* Specific day `cargo run year2024::day16`


# Year 2024 Work in Progress I need to optimize following puzzles : 

(time is estimate through criterion benchmark)
* Day 07 (243 ms)
* Day 06 ( 88 ms)
* Day 16 ( 67 ms) 
* Day 18 ( 65 ms)
* Day 23 ( 48 ms)
* Day 11 ( 38 ms)

# TIP FOR Adding new days 

* Adding new daynn.rs in src/yearaaaa where nn is the number of the day and aaaa the year
* updating lib.rs and main.rs macro for adding the new day in the corresponding year
* Adding new daynn.rs in test/yearaaaa/ importing required ressource "use aoc::yearaaaa::daynn::*; at the head of the file
* updating test.rs by updating corresponding macro with the new day.

Template for bootstrapping src/yearyyyy/daydd.rs
``` Rust
type ParseResult = String;

pub fn parse(input:String) -> ParseResult {
    input
}

pub fn part1(_:&ParseResult) -> String {
    "1".to_string()
}

pub fn part2(_:&ParseResult) -> String {
    "2".to_string()
}
```

Template for bootstrapping tests/yearyyyy/daydd.rs

``` Rust
use aoc::yearyyyy::daynn::*;

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
```

[ManEatingApe AOC Rust Repository]: https://github.com/maneatingape/advent-of-code-rust
[checks-badge]: https://img.shields.io/github/actions/workflow/status/brodier/rust_aoc/rust.yml?label=checks
[checks-link]: https://github.com/brodier/rust_aoc/actions/workflows/rust.yml
[Advent of Code]: https://adventofcode.com
