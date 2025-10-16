# My (brodier) [Advent of Code] Solutions

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

* Day 22 Elapsed: 9433499 μs
* Day 23 Elapsed: 6148329 μs
* Day 06 Elapsed: 2176052 μs
* Day 14 Elapsed: 1536287 μs
* Day 18 Elapsed: 1447364 μs
* Day 09 Elapsed: 1385345 μs
* Day 13 Elapsed: 1201761 μs


# TIP FOR Adding new days 

* Adding new daynn.rs in src/yearaaaa where nn is the number of the day and aaaa the year
* updating lib.rs and main.rs macro for adding the new day in the corresponding year
* Adding new daynn.rs in test/yearaaaa/ importing required ressource "use aoc::yearaaaa::daynn::*; at the head of the file
* updating test.rs by updating corresponding macro with the new day.

Template for bootstrapping src/yearyyyy/daydd.rs
``` Rust
#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}


impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        Puzzle { step, input }
    }

    fn solve(&self) -> String {
        println!("{}", self.input);
        self.step.to_string()
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
```

Template for bootstrapping tests/yearyyyy/daydd.rs

``` Rust
use aoc::yearyyyy::daynn::*;

const EXAMPLE: &str = "\
";

#[test]
fn part1_test() {
    assert_eq!(solve(1, EXAMPLE.to_string()), "1");
}

#[test]
fn part2_test() {
    assert_eq!(solve(2,EXAMPLE.to_string()), "2");
}
```

[ManEatingApe AOC Rust Repository]: https://github.com/maneatingape/advent-of-code-rust
[checks-link]: https://github.com/brodier/rust_oac/actions/workflows/rust.yml
[Advent of Code]: https://adventofcode.com
