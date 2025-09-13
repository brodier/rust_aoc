# My (brodier) Advent of Code Solutions

The wrapping for launching solver is largely inspired 
from maneatingape/advent-of-code-rust.git repository on GitHub

# Advent of Code [![checks-badge]][checks-link] [![docs-badge]][docs-link]

Currently playing with year 2024 

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

* Adding new daynn.rs in src/yearaaaa.rs where nn is the number of the day and aaaa the year
* updating lib.rs and main.rs macro for adding the new day in the corresponding year
* creating Adding new daynn.rs in src/yearaaaa.rs importing required ressource "use aoc::yearaaaa::daynn::*; at the head of the file
* updating test.rs by updating corresponding macro with the new day.
