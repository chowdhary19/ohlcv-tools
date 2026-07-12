use criterion::{Criterion, criterion_group, criterion_main};
use ohlcv_tools::moving_average::sma;

fn synthetic_prices(n: usize) -> Vec<f64> {
    (0..n).map(|i| 100.0 + (i % 50) as f64).collect()
}

fn bench_sma(c: &mut Criterion) {
    let prices = synthetic_prices(100_000);
    c.bench_function("sma over 100k prices, window 14", |b| {
        b.iter(|| sma(&prices, 14))
    });
}

criterion_group!(benches, bench_sma);
criterion_main!(benches);
