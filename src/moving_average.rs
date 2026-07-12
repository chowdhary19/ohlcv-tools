use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
struct PriceRow {
    price: f64,
}

pub fn load_prices(path: &Path) -> Result<Vec<f64>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    reader
        .deserialize()
        .map(|row: Result<PriceRow, csv::Error>| row.map(|r| r.price))
        .collect()
}

/// Simple moving average with the given window size. Returns one value
/// per window-sized slice; empty if there are fewer prices than `window`.
pub fn sma(prices: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || prices.len() < window {
        return Vec::new();
    }
    prices
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect()
}

/// Exponential moving average with the given period (smoothing factor
/// `2 / (period + 1)`). The first EMA value is seeded with the simple
/// average of the first `period` prices, matching common conventions.
pub fn ema(prices: &[f64], period: usize) -> Vec<f64> {
    if period == 0 || prices.len() < period {
        return Vec::new();
    }
    let alpha = 2.0 / (period as f64 + 1.0);
    let seed: f64 = prices[..period].iter().sum::<f64>() / period as f64;

    let mut result = Vec::with_capacity(prices.len() - period + 1);
    result.push(seed);

    for price in &prices[period..] {
        let prev = *result.last().unwrap();
        result.push(alpha * price + (1.0 - alpha) * prev);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sma_of_too_few_prices_is_empty() {
        assert_eq!(sma(&[1.0, 2.0], 3), Vec::<f64>::new());
    }

    #[test]
    fn sma_basic_window() {
        let prices = [1.0, 2.0, 3.0, 4.0];
        // windows of 2: (1+2)/2=1.5, (2+3)/2=2.5, (3+4)/2=3.5
        assert_eq!(sma(&prices, 2), vec![1.5, 2.5, 3.5]);
    }

    #[test]
    fn sma_window_equal_to_length_gives_one_value() {
        let prices = [2.0, 4.0, 6.0];
        assert_eq!(sma(&prices, 3), vec![4.0]);
    }

    #[test]
    fn ema_of_too_few_prices_is_empty() {
        assert_eq!(ema(&[1.0, 2.0], 3), Vec::<f64>::new());
    }

    #[test]
    fn ema_seeds_with_simple_average_then_smooths() {
        let prices = [1.0, 2.0, 3.0, 4.0];
        let result = ema(&prices, 2);
        // seed = (1+2)/2 = 1.5; alpha = 2/3
        // next = 2/3*3 + 1/3*1.5 = 2.5
        // next = 2/3*4 + 1/3*2.5 ≈ 3.5
        assert_eq!(result.len(), 3);
        assert!((result[0] - 1.5).abs() < 1e-9);
        assert!((result[1] - 2.5).abs() < 1e-9);
        assert!((result[2] - 3.5).abs() < 1e-9);
    }
}
