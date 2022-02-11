use criterion::{black_box, criterion_group, criterion_main, Criterion};

use utilities::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    for i in (1..TEST_SIZE).step_by(TEST_STEP) {
        c.bench_function(
            format!("for loops and hashmaps: {} edges", i * TEST_UNIT).as_ref(),
            |b| b.iter(|| for_loops_and_hashmaps::compute(black_box(example_input(i)))),
        );
    }

    let i = 10000;
    c.bench_function(
        format!("for loops and hashmaps: {} edges", i * TEST_UNIT).as_ref(),
        |b| b.iter(|| for_loops_and_hashmaps::compute(black_box(example_input(i)))),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
