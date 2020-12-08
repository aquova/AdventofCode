use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("aoc_8p1", |b| b.iter(|| day8::day8p1()));
    c.bench_function("aoc_8p2", |b| b.iter(|| day8::day8p2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
