use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn bench_generate(c: &mut Criterion) {
    let mut group = c.benchmark_group("snowflake");
    group.throughput(Throughput::Elements(1)).sample_size(1000);
    group.bench_function("generate", |b| {
        let mut generator = snowflake::SnowflakeGenerator::new(0, 0);
        b.iter(|| generator.generate())
    });
    group.bench_function("generate_fuzzy", |b| {
        let mut generator = snowflake::SnowflakeGenerator::new(0, 1);
        b.iter(|| generator.generate_fuzzy())
    });
    group.finish();
}

criterion_group!(benches, bench_generate);
criterion_main!(benches);
