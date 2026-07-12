use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Trade {
    pub price: f64,
    pub volume: f64,
}

pub fn load_trades(path: &Path) -> Result<Vec<Trade>, csv::Error> {
    let mut reader = crate::input::reader_for(path)?;
    reader.deserialize().collect()
}

/// Volume-weighted average price. Returns `None` if there are no trades
/// or total volume is zero (VWAP is undefined in that case).
pub fn vwap(trades: &[Trade]) -> Option<f64> {
    let total_volume: f64 = trades.iter().map(|t| t.volume).sum();
    if total_volume == 0.0 {
        return None;
    }
    let total_value: f64 = trades.iter().map(|t| t.price * t.volume).sum();
    Some(total_value / total_volume)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trade(price: f64, volume: f64) -> Trade {
        Trade { price, volume }
    }

    #[test]
    fn vwap_of_empty_is_none() {
        assert_eq!(vwap(&[]), None);
    }

    #[test]
    fn vwap_of_zero_volume_is_none() {
        let trades = vec![trade(100.0, 0.0), trade(200.0, 0.0)];
        assert_eq!(vwap(&trades), None);
    }

    #[test]
    fn vwap_weights_by_volume() {
        let trades = vec![trade(10.0, 1.0), trade(20.0, 3.0)];
        // (10*1 + 20*3) / (1+3) = 70/4 = 17.5
        assert_eq!(vwap(&trades), Some(17.5));
    }

    #[test]
    fn vwap_single_trade_equals_its_price() {
        let trades = vec![trade(42.0, 5.0)];
        assert_eq!(vwap(&trades), Some(42.0));
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// VWAP must fall within [min price, max price] for any set of
        /// trades with strictly positive volumes.
        #[test]
        fn vwap_is_bounded_by_min_and_max_price(
            trades in proptest::collection::vec(
                (1.0f64..10_000.0, 0.0001f64..1_000.0),
                1..50,
            )
        ) {
            let trades: Vec<Trade> = trades
                .into_iter()
                .map(|(price, volume)| Trade { price, volume })
                .collect();
            let min = trades.iter().map(|t| t.price).fold(f64::INFINITY, f64::min);
            let max = trades.iter().map(|t| t.price).fold(f64::NEG_INFINITY, f64::max);

            let result = vwap(&trades).expect("positive volumes always yield a VWAP");

            prop_assert!(result >= min - 1e-9);
            prop_assert!(result <= max + 1e-9);
        }
    }
}
