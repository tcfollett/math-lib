use math_core::{MathError, MathResult, ensure_non_empty, ensure_same_len};

#[derive(Debug, Clone, PartialEq)]
pub struct SummaryStatistics {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub modes: Vec<f64>,
    pub variance: f64,
    pub sample_variance: f64,
    pub std_dev: f64,
    pub sample_std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub q1: f64,
    pub q3: f64,
    pub iqr: f64,
}

pub fn summary(values: &[f64]) -> MathResult<SummaryStatistics> {
    validate_sample(values, "summary")?;

    let q1 = quantile(values, 0.25)?;
    let q3 = quantile(values, 0.75)?;

    Ok(SummaryStatistics {
        count: values.len(),
        mean: mean(values)?,
        median: median(values)?,
        modes: mode(values)?,
        variance: variance(values)?,
        sample_variance: sample_variance(values)?,
        std_dev: std_dev(values)?,
        sample_std_dev: sample_std_dev(values)?,
        min: values.iter().copied().fold(f64::INFINITY, f64::min),
        max: values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        q1,
        q3,
        iqr: q3 - q1,
    })
}

pub fn mean(values: &[f64]) -> MathResult<f64> {
    validate_sample(values, "mean")?;
    Ok(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn median(values: &[f64]) -> MathResult<f64> {
    validate_sample(values, "median")?;
    let sorted = sorted(values);
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Ok(sorted[mid])
    }
}

pub fn mode(values: &[f64]) -> MathResult<Vec<f64>> {
    validate_sample(values, "mode")?;
    let sorted = sorted(values);
    let mut modes = Vec::new();
    let mut best_count = 1usize;
    let mut current_value = sorted[0];
    let mut current_count = 1usize;

    for value in sorted.iter().copied().skip(1) {
        if value == current_value {
            current_count += 1;
        } else {
            update_modes(current_value, current_count, &mut best_count, &mut modes);
            current_value = value;
            current_count = 1;
        }
    }
    update_modes(current_value, current_count, &mut best_count, &mut modes);

    if best_count == 1 {
        Ok(Vec::new())
    } else {
        Ok(modes)
    }
}

pub fn variance(values: &[f64]) -> MathResult<f64> {
    validate_sample(values, "variance")?;
    let mean = mean(values)?;
    Ok(values
        .iter()
        .map(|value| {
            let delta = *value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64)
}

pub fn sample_variance(values: &[f64]) -> MathResult<f64> {
    validate_sample_size(values, 2, "sample_variance")?;
    let mean = mean(values)?;
    Ok(values
        .iter()
        .map(|value| {
            let delta = *value - mean;
            delta * delta
        })
        .sum::<f64>()
        / (values.len() - 1) as f64)
}

pub fn std_dev(values: &[f64]) -> MathResult<f64> {
    Ok(variance(values)?.sqrt())
}

pub fn sample_std_dev(values: &[f64]) -> MathResult<f64> {
    Ok(sample_variance(values)?.sqrt())
}

pub fn covariance(left: &[f64], right: &[f64]) -> MathResult<f64> {
    validate_sample_size(left, 2, "covariance")?;
    validate_finite(right, "covariance")?;
    ensure_same_len(left, right, "covariance")?;
    let left_mean = mean(left)?;
    let right_mean = mean(right)?;
    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(lhs, rhs)| (lhs - left_mean) * (rhs - right_mean))
        .sum::<f64>()
        / (left.len() - 1) as f64)
}

pub fn correlation(left: &[f64], right: &[f64]) -> MathResult<f64> {
    let left_std = sample_std_dev(left)?;
    let right_std = sample_std_dev(right)?;
    if left_std <= f64::EPSILON || right_std <= f64::EPSILON {
        return Err(MathError::Statistics {
            context: "correlation",
            message: "standard deviation must be non-zero".to_string(),
        });
    }

    Ok(covariance(left, right)? / (left_std * right_std))
}

pub fn quantile(values: &[f64], probability: f64) -> MathResult<f64> {
    validate_sample(values, "quantile")?;
    if !(0.0..=1.0).contains(&probability) {
        return Err(MathError::InvalidRange {
            context: "quantile",
            message: "probability must be in [0, 1]".to_string(),
        });
    }

    let sorted = sorted(values);
    if sorted.len() == 1 {
        return Ok(sorted[0]);
    }

    let position = probability * (sorted.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    if lower == upper {
        return Ok(sorted[lower]);
    }

    let weight = position - lower as f64;
    Ok(sorted[lower] * (1.0 - weight) + sorted[upper] * weight)
}

fn validate_sample(values: &[f64], context: &'static str) -> MathResult<()> {
    ensure_non_empty(values, context)?;
    validate_finite(values, context)
}

fn validate_sample_size(values: &[f64], min_size: usize, context: &'static str) -> MathResult<()> {
    validate_sample(values, context)?;
    if values.len() < min_size {
        return Err(MathError::Statistics {
            context,
            message: format!("requires at least {min_size} observations"),
        });
    }
    Ok(())
}

fn validate_finite(values: &[f64], context: &'static str) -> MathResult<()> {
    if values.iter().any(|value| !value.is_finite()) {
        return Err(MathError::Statistics {
            context,
            message: "all sample values must be finite".to_string(),
        });
    }
    Ok(())
}

fn sorted(values: &[f64]) -> Vec<f64> {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted
}

fn update_modes(value: f64, count: usize, best_count: &mut usize, modes: &mut Vec<f64>) {
    match count.cmp(best_count) {
        std::cmp::Ordering::Greater => {
            *best_count = count;
            modes.clear();
            modes.push(value);
        }
        std::cmp::Ordering::Equal if count > 1 => modes.push(value),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{correlation, mean, mode, quantile, sample_variance, summary};

    #[test]
    fn descriptive_statistics_match_known_values() {
        let values = [1.0, 2.0, 2.0, 3.0, 4.0];
        let stats = summary(&values).unwrap();

        assert_eq!(mean(&values).unwrap(), 2.4);
        assert_eq!(mode(&values).unwrap(), vec![2.0]);
        assert!((sample_variance(&values).unwrap() - 1.3).abs() < 1e-10);
        assert_eq!(quantile(&values, 0.5).unwrap(), 2.0);
        assert_eq!(stats.count, 5);
        assert_eq!(stats.median, 2.0);
    }

    #[test]
    fn correlation_is_one_for_perfect_linear_relationship() {
        let left = [1.0, 2.0, 3.0, 4.0];
        let right = [2.0, 4.0, 6.0, 8.0];
        assert!((correlation(&left, &right).unwrap() - 1.0).abs() < 1e-12);
    }
}
