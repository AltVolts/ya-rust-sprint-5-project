use broken_app::{algo, sum_even, leak_buffer, normalize, average_positive};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

fn bench_sum_even(c: &mut Criterion) {
    let data: Vec<i64> = (0..50_000).collect();
    c.bench_function("sum_even_broken", |b| b.iter(|| sum_even(&data)));
}

fn bench_leak_buffer(c: &mut Criterion) {
    let data: Vec<u8> = (0..10_000).map(|x| x as u8).collect();
    c.bench_function("leak_buffer_broken", |b| b.iter(|| leak_buffer(&data)));
}

fn bench_normalize(c: &mut Criterion) {
    let text = "  Hello   \t World \n  ".repeat(100);
    c.bench_function("normalize_broken", |b| b.iter(|| normalize(&text)));
}

fn bench_average_positive(c: &mut Criterion) {
    let nums: Vec<i64> = (-10..10_000).collect();
    c.bench_function("average_positive_broken", |b| b.iter(|| average_positive(&nums)));
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("slow_fib_broken", |b| b.iter(|| algo::slow_fib(32)));
}

fn bench_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    c.bench_function("slow_dedup_broken", |b| {
        b.iter_batched(
            || data.clone(),
            |v| {
                let _ = algo::slow_dedup(&v);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    bench_sum_even,
    bench_leak_buffer,
    bench_normalize,
    bench_average_positive,
    bench_fib,
    bench_dedup
);
criterion_main!(benches);