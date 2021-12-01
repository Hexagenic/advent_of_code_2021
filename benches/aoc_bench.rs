extern crate advent_of_code_2021;

#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| advent_of_code_2021::solutions::day1())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
