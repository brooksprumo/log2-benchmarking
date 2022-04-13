use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use log2_benchmarking::*;
use std::num::NonZeroU64;

fn bench_log2_ceil(c: &mut Criterion) {
    let mut group = c.benchmark_group("log2_ceil");
    for ref i in [1, (1 << 32) - 1, (1 << 32), (1 << 32) + 1, (1 << 63) - 1] {
        group.bench_with_input(BenchmarkId::new("baseline", i), i, |b, i| {
            b.iter(|| log2_ceil_baseline(*i))
        });
        group.bench_with_input(BenchmarkId::new("v1", i), i, |b, i| {
            b.iter(|| log2_ceil_v1(*i))
        });
        group.bench_with_input(BenchmarkId::new("v2", i), i, |b, i| {
            b.iter(|| log2_ceil_v2(unsafe { NonZeroU64::new_unchecked(*i) }))
        });
        group.bench_with_input(BenchmarkId::new("v3", i), i, |b, i| {
            b.iter(|| log2_ceil_v3(unsafe { NonZeroU63::new_unchecked(*i) }))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_log2_ceil);
criterion_main!(benches);
