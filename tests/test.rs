
macro_rules! test {
    ($year:tt:$($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

test!(year2024:
    day01, 
    day02, 
    day03, 
    day04,
    day07,
    day17,
    day18, 
    day19, 
    day20, 
    day21, 
    day22,
    day23,
    day24,
    day25
);
