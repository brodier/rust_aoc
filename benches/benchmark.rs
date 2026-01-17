use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc::utils::launcher::*   ;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("year2024::day09", |b| b.iter(|| launch(black_box(Some(2024)),black_box(Some(9)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);