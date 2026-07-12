#[derive(Debug, PartialEq, serde::Serialize)]
pub struct Stats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
}

/// Summary statistics for a price series. `None` if empty.
pub fn summarize(prices: &[f64]) -> Option<Stats> {
    if prices.is_empty() {
        return None;
    }
    let mut sorted = prices.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let mean = prices.iter().sum::<f64>() / prices.len() as f64;
    let mid = sorted.len() / 2;
    let median = if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    };

    Some(Stats {
        min,
        max,
        mean,
        median,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_none() {
        assert_eq!(summarize(&[]), None);
    }

    #[test]
    fn odd_length_series() {
        let s = summarize(&[3.0, 1.0, 2.0]).unwrap();
        assert_eq!(
            s,
            Stats {
                min: 1.0,
                max: 3.0,
                mean: 2.0,
                median: 2.0
            }
        );
    }

    #[test]
    fn even_length_series_averages_middle_two() {
        let s = summarize(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(s.median, 2.5);
    }

    #[test]
    fn unsorted_input_does_not_affect_result() {
        let s = summarize(&[5.0, 1.0, 3.0, 2.0, 4.0]).unwrap();
        assert_eq!(
            s,
            Stats {
                min: 1.0,
                max: 5.0,
                mean: 3.0,
                median: 3.0
            }
        );
    }
}
