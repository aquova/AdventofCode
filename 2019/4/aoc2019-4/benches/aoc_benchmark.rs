use criterion::{criterion_group, criterion_main, Criterion};
use aoc2019_4::aoc;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("aoc_4", |b| b.iter(|| aoc()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
