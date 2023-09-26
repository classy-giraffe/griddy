use criterion::{black_box, criterion_group, criterion_main, Criterion};
use griddy::prelude::*;

#[inline]
fn img_bench(path: &str) -> Png {
    Png::new(path).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("big_sample", |b| {
        b.iter(|| img_bench(black_box(r"./samples/big_sample.png")))
    });
    c.bench_function("small_sample", |b| {
        b.iter(|| img_bench(black_box(r"./samples/sample.png")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
