/// Simple percentage returns between consecutive prices.
pub fn simple_returns(prices: &[f64]) -> Vec<f64> {
    prices.windows(2).map(|w| (w[1] - w[0]) / w[0]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_or_single_price_has_no_returns() {
        assert_eq!(simple_returns(&[]), Vec::<f64>::new());
        assert_eq!(simple_returns(&[100.0]), Vec::<f64>::new());
    }

    #[test]
    fn computes_percentage_change() {
        let prices = [100.0, 110.0, 99.0];
        // (110-100)/100 = 0.10, (99-110)/110 = -0.1
        assert_eq!(simple_returns(&prices), vec![0.10, -0.1]);
    }
}
