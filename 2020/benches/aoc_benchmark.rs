use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("aoc_1p1", |b| b.iter(|| day1::day1p1()));
    c.bench_function("aoc_1p2", |b| b.iter(|| day1::day1p2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
