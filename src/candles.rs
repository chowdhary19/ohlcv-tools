use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Tick {
    pub timestamp: i64,
    pub price: f64,
    pub volume: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

pub fn load_ticks(path: &Path) -> Result<Vec<Tick>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    reader.deserialize().collect()
}

/// Aggregate ticks into OHLCV candles bucketed by `interval_secs`.
/// Ticks do not need to be pre-sorted by timestamp. Non-positive
/// intervals produce no candles.
pub fn aggregate(ticks: &[Tick], interval_secs: i64) -> Vec<Candle> {
    if interval_secs <= 0 || ticks.is_empty() {
        return Vec::new();
    }

    let mut sorted: Vec<&Tick> = ticks.iter().collect();
    sorted.sort_by_key(|t| t.timestamp);

    let mut candles: Vec<Candle> = Vec::new();
    let mut current_bucket: Option<i64> = None;

    for tick in sorted {
        let bucket = tick.timestamp.div_euclid(interval_secs) * interval_secs;

        if current_bucket == Some(bucket) {
            let candle = candles
                .last_mut()
                .expect("current_bucket implies a candle exists");
            candle.high = candle.high.max(tick.price);
            candle.low = candle.low.min(tick.price);
            candle.close = tick.price;
            candle.volume += tick.volume;
        } else {
            candles.push(Candle {
                timestamp: bucket,
                open: tick.price,
                high: tick.price,
                low: tick.price,
                close: tick.price,
                volume: tick.volume,
            });
            current_bucket = Some(bucket);
        }
    }

    candles
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tick(timestamp: i64, price: f64, volume: f64) -> Tick {
        Tick {
            timestamp,
            price,
            volume,
        }
    }

    #[test]
    fn aggregate_of_empty_is_empty() {
        assert_eq!(aggregate(&[], 60), Vec::new());
    }

    #[test]
    fn non_positive_interval_is_empty() {
        let ticks = vec![tick(0, 100.0, 1.0)];
        assert_eq!(aggregate(&ticks, 0), Vec::new());
        assert_eq!(aggregate(&ticks, -60), Vec::new());
    }

    #[test]
    fn single_tick_makes_one_flat_candle() {
        let ticks = vec![tick(10, 100.0, 5.0)];
        let candles = aggregate(&ticks, 60);
        assert_eq!(
            candles,
            vec![Candle {
                timestamp: 0,
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
                volume: 5.0
            }]
        );
    }

    #[test]
    fn multiple_ticks_in_one_bucket_aggregate_ohlc() {
        let ticks = vec![
            tick(0, 100.0, 1.0),
            tick(10, 110.0, 2.0),
            tick(20, 90.0, 3.0),
            tick(30, 105.0, 4.0),
        ];
        let candles = aggregate(&ticks, 60);
        assert_eq!(candles.len(), 1);
        let c = &candles[0];
        assert_eq!(c.open, 100.0);
        assert_eq!(c.high, 110.0);
        assert_eq!(c.low, 90.0);
        assert_eq!(c.close, 105.0);
        assert_eq!(c.volume, 10.0);
    }

    #[test]
    fn ticks_spanning_multiple_buckets_make_multiple_candles() {
        let ticks = vec![
            tick(0, 100.0, 1.0),
            tick(65, 200.0, 1.0),
            tick(130, 300.0, 1.0),
        ];
        let candles = aggregate(&ticks, 60);
        let timestamps: Vec<i64> = candles.iter().map(|c| c.timestamp).collect();
        assert_eq!(timestamps, vec![0, 60, 120]);
    }

    #[test]
    fn unsorted_input_is_aggregated_correctly() {
        let ticks = vec![
            tick(20, 90.0, 1.0),
            tick(0, 100.0, 1.0),
            tick(10, 110.0, 1.0),
        ];
        let candles = aggregate(&ticks, 60);
        assert_eq!(candles.len(), 1);
        assert_eq!(candles[0].open, 100.0); // earliest timestamp, not input order
        assert_eq!(candles[0].close, 90.0); // latest timestamp, not input order
    }
}
