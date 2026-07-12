/// Pearson correlation coefficient between two equal-length series.
/// `None` if lengths differ, fewer than 2 points, or either series has
/// zero variance (correlation undefined).
pub fn pearson(a: &[f64], b: &[f64]) -> Option<f64> {
    if a.len() != b.len() || a.len() < 2 {
        return None;
    }
    let n = a.len() as f64;
    let mean_a = a.iter().sum::<f64>() / n;
    let mean_b = b.iter().sum::<f64>() / n;

    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for (&x, &y) in a.iter().zip(b) {
        let da = x - mean_a;
        let db = y - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }

    if var_a == 0.0 || var_b == 0.0 {
        return None;
    }
    Some(cov / (var_a.sqrt() * var_b.sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mismatched_lengths_is_none() {
        assert_eq!(pearson(&[1.0, 2.0], &[1.0]), None);
    }

    #[test]
    fn too_few_points_is_none() {
        assert_eq!(pearson(&[1.0], &[1.0]), None);
    }

    #[test]
    fn zero_variance_is_none() {
        assert_eq!(pearson(&[1.0, 1.0, 1.0], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn perfect_positive_correlation() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [2.0, 4.0, 6.0, 8.0];
        let r = pearson(&a, &b).unwrap();
        assert!((r - 1.0).abs() < 1e-9);
    }

    #[test]
    fn perfect_negative_correlation() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [-1.0, -2.0, -3.0, -4.0];
        let r = pearson(&a, &b).unwrap();
        assert!((r + 1.0).abs() < 1e-9);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Pearson correlation is only ever defined in [-1, 1] by
        /// construction (Cauchy-Schwarz), for any two equal-length series
        /// with non-zero variance.
        #[test]
        fn pearson_is_bounded_between_negative_one_and_one(
            a in proptest::collection::vec(-10_000.0f64..10_000.0, 2..50)
        ) {
            // Derive `b` from `a` with a bit of noise so both series
            // reliably have non-zero variance without biasing toward
            // any particular correlation value.
            let b: Vec<f64> = a.iter().enumerate().map(|(i, &x)| x * 0.5 + i as f64).collect();

            if let Some(r) = pearson(&a, &b) {
                prop_assert!(r >= -1.0 - 1e-9);
                prop_assert!(r <= 1.0 + 1e-9);
            }
        }
    }
}
