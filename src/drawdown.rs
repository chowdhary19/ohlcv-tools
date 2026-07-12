/// Maximum peak-to-trough percentage drawdown, as a positive fraction
/// (e.g. 0.2 for a 20% drop). Returns `None` for fewer than 2 prices.
pub fn max_drawdown(prices: &[f64]) -> Option<f64> {
    if prices.len() < 2 {
        return None;
    }
    let mut peak = prices[0];
    let mut worst = 0.0f64;
    for &price in &prices[1..] {
        peak = peak.max(price);
        let drawdown = (peak - price) / peak;
        worst = worst.max(drawdown);
    }
    Some(worst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_few_prices_is_none() {
        assert_eq!(max_drawdown(&[]), None);
        assert_eq!(max_drawdown(&[100.0]), None);
    }

    #[test]
    fn monotonic_rise_has_zero_drawdown() {
        assert_eq!(max_drawdown(&[100.0, 110.0, 120.0]), Some(0.0));
    }

    #[test]
    fn finds_worst_peak_to_trough_drop() {
        // peak 100 -> trough 80 = 20% dd; then peak 120 -> trough 90 = 25% dd (worst)
        let prices = [100.0, 80.0, 120.0, 90.0];
        let dd = max_drawdown(&prices).unwrap();
        assert!((dd - 0.25).abs() < 1e-9);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Max drawdown is a peak-to-trough drop, so it can never be
        /// negative for any series of strictly positive prices.
        #[test]
        fn max_drawdown_is_never_negative(
            prices in proptest::collection::vec(1.0f64..10_000.0, 2..50)
        ) {
            let result = max_drawdown(&prices).expect("2+ prices always yield a drawdown");
            prop_assert!(result >= 0.0 - 1e-9);
        }
    }
}
