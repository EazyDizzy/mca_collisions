use criterion::{criterion_group, criterion_main, Criterion};
use mca_collisions::read::read_level;
use pprof::criterion::{Output, PProfProfiler};

fn bench_read_lvl(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_lvl");
    group.sample_size(10);

    group.bench_function("read_level", |b| {
        b.iter(|| read_level("./assets/simple_lvl"))
    });

    group.finish()
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_read_lvl
}
criterion_main!(benches);
