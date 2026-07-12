use criterion::{Criterion, criterion_group, criterion_main};
use ohlcv_tools::candles::{Tick, aggregate};
use ohlcv_tools::resample::resample;

fn synthetic_candles(n: usize) -> Vec<ohlcv_tools::candles::Candle> {
    let ticks: Vec<Tick> = (0..n)
        .map(|i| Tick {
            timestamp: i as i64,
            price: 100.0 + (i % 50) as f64,
            volume: 1.0 + (i % 10) as f64,
        })
        .collect();
    // one-minute candles, same shape `aggregate` produces for real input
    aggregate(&ticks, 60)
}

fn bench_resample(c: &mut Criterion) {
    let candles = synthetic_candles(100_000);
    c.bench_function("resample 1m candles into 1h candles", |b| {
        b.iter(|| resample(&candles, 3600))
    });
}

criterion_group!(benches, bench_resample);
criterion_main!(benches);
