use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("aoc_7p1", |b| b.iter(|| day7::day7p1()));
    c.bench_function("aoc_7p2", |b| b.iter(|| day7::day7p2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
