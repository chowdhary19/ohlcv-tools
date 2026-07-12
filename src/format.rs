/// Round `value` to `decimals` decimal places for display.
pub fn round_to(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_down_correctly() {
        assert_eq!(round_to(1.23449, 4), 1.2345);
    }

    #[test]
    fn rounds_up_correctly() {
        assert_eq!(round_to(1.23456, 4), 1.2346);
    }

    #[test]
    fn zero_decimals_rounds_to_whole_number() {
        assert_eq!(round_to(2.6, 0), 3.0);
    }

    #[test]
    fn handles_repeating_decimals() {
        assert_eq!(round_to(99.57142857142857, 4), 99.5714);
    }

    #[test]
    fn negative_values_round_correctly() {
        assert_eq!(round_to(-1.23456, 4), -1.2346);
    }
}
