use criterion::{criterion_group, criterion_main, Criterion};
use aoc::utils::launcher::launch;

fn bench_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("AOC Bench");
    for day in 1..26 {
        let name = format!("year2024::day{:02}", day);
        group.bench_function(&name, |b| b.iter(|| launch(Some(2024), Some(day)) ));
    }
    for day in 1..20 {
        let name = format!("year2023::day{:02}", day);
        group.bench_function(&name, |b| b.iter(|| launch(Some(2023), Some(day)) ));
    }
    group.finish();
}

criterion_group!(benches, bench_all);
criterion_main!(benches);