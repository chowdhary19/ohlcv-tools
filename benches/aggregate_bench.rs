use criterion::{Criterion, criterion_group, criterion_main};
use ohlcv_tools::candles::{Tick, aggregate};

fn synthetic_ticks(n: usize) -> Vec<Tick> {
    (0..n)
        .map(|i| Tick {
            timestamp: i as i64,
            price: 100.0 + (i % 50) as f64,
            volume: 1.0 + (i % 10) as f64,
        })
        .collect()
}

fn bench_aggregate(c: &mut Criterion) {
    let ticks = synthetic_ticks(100_000);
    c.bench_function("aggregate 100k ticks into 60s candles", |b| {
        b.iter(|| aggregate(&ticks, 60))
    });
}

criterion_group!(benches, bench_aggregate);
criterion_main!(benches);
