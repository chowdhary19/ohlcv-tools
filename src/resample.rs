use crate::candles::Candle;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct CandleRow {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub fn load_candles(path: &Path) -> Result<Vec<Candle>, csv::Error> {
    let mut reader = crate::input::reader_for(path)?;
    reader
        .deserialize()
        .map(|row: Result<CandleRow, csv::Error>| {
            row.map(|r| Candle {
                timestamp: r.timestamp,
                open: r.open,
                high: r.high,
                low: r.low,
                close: r.close,
                volume: r.volume,
            })
        })
        .collect()
}

/// Re-bucket already-aggregated OHLCV candles into a larger interval
/// (candle-of-candles). Input candles do not need to be pre-sorted by
/// timestamp. Within each new bucket, `open` is taken from the
/// earliest-timestamped candle, `close` from the latest, `high`/`low`
/// from the extremes across all candles in the bucket, and `volume` is
/// summed. Non-positive intervals produce no candles.
pub fn resample(candles: &[Candle], interval_secs: i64) -> Vec<Candle> {
    if interval_secs <= 0 || candles.is_empty() {
        return Vec::new();
    }

    let mut sorted: Vec<&Candle> = candles.iter().collect();
    sorted.sort_by_key(|c| c.timestamp);

    let mut result: Vec<Candle> = Vec::new();
    let mut current_bucket: Option<i64> = None;

    for candle in sorted {
        let bucket = candle.timestamp.div_euclid(interval_secs) * interval_secs;

        if current_bucket == Some(bucket) {
            let merged = result
                .last_mut()
                .expect("current_bucket implies a candle exists");
            merged.high = merged.high.max(candle.high);
            merged.low = merged.low.min(candle.low);
            merged.close = candle.close;
            merged.volume += candle.volume;
        } else {
            result.push(Candle {
                timestamp: bucket,
                open: candle.open,
                high: candle.high,
                low: candle.low,
                close: candle.close,
                volume: candle.volume,
            });
            current_bucket = Some(bucket);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candle(timestamp: i64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Candle {
        Candle {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }

    #[test]
    fn resample_of_empty_is_empty() {
        assert_eq!(resample(&[], 300), Vec::new());
    }

    #[test]
    fn non_positive_interval_is_empty() {
        let candles = vec![candle(0, 100.0, 105.0, 95.0, 102.0, 10.0)];
        assert_eq!(resample(&candles, 0), Vec::new());
        assert_eq!(resample(&candles, -60), Vec::new());
    }

    #[test]
    fn single_candle_smaller_than_interval_passes_through() {
        let candles = vec![candle(10, 100.0, 110.0, 90.0, 105.0, 5.0)];
        let result = resample(&candles, 300);
        assert_eq!(result, vec![candle(0, 100.0, 110.0, 90.0, 105.0, 5.0)]);
    }

    #[test]
    fn multiple_candles_in_one_bucket_merge_correctly() {
        // four 1-minute candles resampled into one 5-minute bucket
        let candles = vec![
            candle(0, 100.0, 105.0, 98.0, 102.0, 10.0),
            candle(60, 102.0, 108.0, 101.0, 107.0, 20.0),
            candle(120, 107.0, 107.0, 90.0, 95.0, 15.0),
            candle(180, 95.0, 100.0, 94.0, 99.0, 5.0),
        ];
        let result = resample(&candles, 300);
        assert_eq!(result.len(), 1);
        let c = &result[0];
        assert_eq!(c.timestamp, 0);
        assert_eq!(c.open, 100.0); // from earliest candle
        assert_eq!(c.high, 108.0); // max high across all
        assert_eq!(c.low, 90.0); // min low across all
        assert_eq!(c.close, 99.0); // from latest candle
        assert_eq!(c.volume, 50.0); // summed
    }

    #[test]
    fn candles_spanning_multiple_buckets_stay_separate() {
        let candles = vec![
            candle(0, 100.0, 100.0, 100.0, 100.0, 1.0),
            candle(300, 200.0, 200.0, 200.0, 200.0, 1.0),
            candle(600, 300.0, 300.0, 300.0, 300.0, 1.0),
        ];
        let result = resample(&candles, 300);
        let timestamps: Vec<i64> = result.iter().map(|c| c.timestamp).collect();
        assert_eq!(timestamps, vec![0, 300, 600]);
    }

    #[test]
    fn unsorted_input_is_resampled_correctly() {
        let candles = vec![
            candle(120, 107.0, 107.0, 90.0, 95.0, 1.0),
            candle(0, 100.0, 105.0, 98.0, 102.0, 1.0),
            candle(60, 102.0, 108.0, 101.0, 107.0, 1.0),
        ];
        let result = resample(&candles, 300);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].open, 100.0); // earliest timestamp, not input order
        assert_eq!(result[0].close, 95.0); // latest timestamp, not input order
    }

    #[test]
    fn upsampling_to_smaller_interval_keeps_each_candle_separate() {
        // resampling to an interval smaller than the candle spacing is a
        // no-op re-bucketing: each candle lands in its own bucket.
        let candles = vec![
            candle(0, 100.0, 105.0, 95.0, 102.0, 10.0),
            candle(60, 102.0, 108.0, 101.0, 107.0, 20.0),
        ];
        let result = resample(&candles, 30);
        assert_eq!(result.len(), 2);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::candles::{Tick, aggregate};
    use proptest::prelude::*;

    proptest! {
        /// Resampling real, internally-consistent candles (produced by
        /// `aggregate` from random ticks, same as the aggregate OHLC
        /// invariant proptest) into a larger interval must preserve
        /// basic OHLC consistency: low is the minimum, high is the
        /// maximum, and open/close both fall within [low, high].
        /// Volume must stay non-negative.
        #[test]
        fn resample_preserves_ohlc_invariants(
            ticks in proptest::collection::vec(
                (0i64..100_000, 1.0f64..10_000.0, 0.0f64..1_000.0),
                1..200,
            ),
            source_interval in 1i64..300,
            resample_interval in 1i64..3600,
        ) {
            let ticks: Vec<Tick> = ticks
                .into_iter()
                .map(|(timestamp, price, volume)| Tick { timestamp, price, volume })
                .collect();

            let source_candles = aggregate(&ticks, source_interval);
            for candle in resample(&source_candles, resample_interval) {
                prop_assert!(candle.low <= candle.high);
                prop_assert!(candle.open >= candle.low && candle.open <= candle.high);
                prop_assert!(candle.close >= candle.low && candle.close <= candle.high);
                prop_assert!(candle.volume >= 0.0);
            }
        }
    }
}
