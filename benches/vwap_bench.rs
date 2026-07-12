use criterion::{Criterion, criterion_group, criterion_main};
use ohlcv_tools::vwap::{Trade, vwap};

fn synthetic_trades(n: usize) -> Vec<Trade> {
    (0..n)
        .map(|i| Trade {
            price: 100.0 + (i % 50) as f64,
            volume: 1.0 + (i % 10) as f64,
        })
        .collect()
}

fn bench_vwap(c: &mut Criterion) {
    let trades = synthetic_trades(100_000);
    c.bench_function("vwap over 100k trades", |b| b.iter(|| vwap(&trades)));
}

criterion_group!(benches, bench_vwap);
criterion_main!(benches);
