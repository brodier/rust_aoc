
macro_rules! test {
    ($year:tt:$($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

test!(year2024:
    day01, day02, day03, day04, day17, day18, day19, day20
);
