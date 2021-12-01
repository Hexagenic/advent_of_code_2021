extern crate advent_of_code_2021;

#[macro_use]
extern crate criterion;

use criterion::BenchmarkId;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day");
    for day in 1..=advent_of_code_2021::solutions::MAX_SOLVED_DAY {
        group.bench_with_input(BenchmarkId::from_parameter(day), &day, |b, &day| {
            b.iter(|| advent_of_code_2021::solutions::get_solution(day)());
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
