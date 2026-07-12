/// Reject non-finite values (NaN, +/-Infinity) which would otherwise
/// silently poison downstream calculations (e.g. a single NaN price
/// makes VWAP, SMA, and drawdown all return NaN with no error).
pub fn check_finite(values: &[f64], label: &str) -> Result<(), String> {
    for (i, &v) in values.iter().enumerate() {
        if !v.is_finite() {
            return Err(format!("{label}: non-finite value {v} at row {}", i + 1));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_finite_is_ok() {
        assert!(check_finite(&[1.0, 2.0, 3.0], "price").is_ok());
    }

    #[test]
    fn nan_is_rejected() {
        let err = check_finite(&[1.0, f64::NAN], "price").unwrap_err();
        assert!(err.contains("row 2"));
    }

    #[test]
    fn infinity_is_rejected() {
        assert!(check_finite(&[f64::INFINITY], "price").is_err());
        assert!(check_finite(&[f64::NEG_INFINITY], "price").is_err());
    }
}
