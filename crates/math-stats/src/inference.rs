use crate::descriptive::{correlation, mean, sample_std_dev};
use crate::special::{chi_square_cdf, normal_cdf, student_t_cdf, student_t_quantile};
use math_core::{MathError, MathResult, ensure_non_empty, ensure_same_len};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    pub lower: f64,
    pub upper: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HypothesisTestResult {
    pub statistic: f64,
    pub p_value: f64,
    pub degrees_of_freedom: Option<f64>,
    pub alpha: f64,
    pub reject_null: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegressionResult {
    pub slope: f64,
    pub intercept: f64,
    pub correlation: f64,
    pub r_squared: f64,
    pub residual_std_error: f64,
}

pub fn mean_confidence_interval(sample: &[f64], confidence: f64) -> MathResult<ConfidenceInterval> {
    validate_probability(confidence, "mean_confidence_interval")?;
    ensure_non_empty(sample, "mean_confidence_interval")?;
    if sample.len() < 2 {
        return Err(MathError::Statistics {
            context: "mean_confidence_interval",
            message: "requires at least two observations".to_string(),
        });
    }

    let sample_mean = mean(sample)?;
    let std_dev = sample_std_dev(sample)?;
    let dof = (sample.len() - 1) as f64;
    let critical = student_t_quantile(0.5 + confidence / 2.0, dof)?;
    let margin = critical * std_dev / (sample.len() as f64).sqrt();

    Ok(ConfidenceInterval {
        lower: sample_mean - margin,
        upper: sample_mean + margin,
        confidence,
    })
}

pub fn z_test_mean(
    sample: &[f64],
    hypothesized_mean: f64,
    population_std_dev: f64,
    alpha: f64,
) -> MathResult<HypothesisTestResult> {
    validate_probability(1.0 - alpha, "z_test_mean")?;
    ensure_non_empty(sample, "z_test_mean")?;
    if population_std_dev <= 0.0 {
        return Err(MathError::Statistics {
            context: "z_test_mean",
            message: "population standard deviation must be positive".to_string(),
        });
    }

    let sample_mean = mean(sample)?;
    let z = (sample_mean - hypothesized_mean) / (population_std_dev / (sample.len() as f64).sqrt());
    let p_value = 2.0 * (1.0 - normal_cdf(z.abs()));
    Ok(HypothesisTestResult {
        statistic: z,
        p_value,
        degrees_of_freedom: None,
        alpha,
        reject_null: p_value < alpha,
    })
}

pub fn t_test_mean(
    sample: &[f64],
    hypothesized_mean: f64,
    alpha: f64,
) -> MathResult<HypothesisTestResult> {
    validate_probability(1.0 - alpha, "t_test_mean")?;
    ensure_non_empty(sample, "t_test_mean")?;
    if sample.len() < 2 {
        return Err(MathError::Statistics {
            context: "t_test_mean",
            message: "requires at least two observations".to_string(),
        });
    }

    let sample_mean = mean(sample)?;
    let std_dev = sample_std_dev(sample)?;
    let dof = (sample.len() - 1) as f64;
    let t_stat = (sample_mean - hypothesized_mean) / (std_dev / (sample.len() as f64).sqrt());
    let p_value = 2.0 * (1.0 - student_t_cdf(t_stat.abs(), dof)?);

    Ok(HypothesisTestResult {
        statistic: t_stat,
        p_value,
        degrees_of_freedom: Some(dof),
        alpha,
        reject_null: p_value < alpha,
    })
}

pub fn chi_square_goodness_of_fit(
    observed: &[u64],
    expected_probabilities: &[f64],
    alpha: f64,
) -> MathResult<HypothesisTestResult> {
    validate_probability(1.0 - alpha, "chi_square_goodness_of_fit")?;
    ensure_non_empty(observed, "chi_square_goodness_of_fit")?;
    ensure_same_len(
        observed,
        expected_probabilities,
        "chi_square_goodness_of_fit",
    )?;
    if expected_probabilities
        .iter()
        .any(|value| *value <= 0.0 || !value.is_finite())
    {
        return Err(MathError::Statistics {
            context: "chi_square_goodness_of_fit",
            message: "expected probabilities must be finite and positive".to_string(),
        });
    }

    let total_probability: f64 = expected_probabilities.iter().sum();
    if (total_probability - 1.0).abs() > 1e-6 {
        return Err(MathError::Statistics {
            context: "chi_square_goodness_of_fit",
            message: "expected probabilities must sum to 1".to_string(),
        });
    }

    let total_count: u64 = observed.iter().sum();
    let statistic = observed
        .iter()
        .zip(expected_probabilities.iter())
        .map(|(count, probability)| {
            let expected = total_count as f64 * probability;
            let delta = *count as f64 - expected;
            delta * delta / expected
        })
        .sum::<f64>();

    let dof = (observed.len() - 1) as f64;
    let p_value = 1.0 - chi_square_cdf(statistic, dof)?;
    Ok(HypothesisTestResult {
        statistic,
        p_value,
        degrees_of_freedom: Some(dof),
        alpha,
        reject_null: p_value < alpha,
    })
}

pub fn linear_regression(x: &[f64], y: &[f64]) -> MathResult<RegressionResult> {
    ensure_non_empty(x, "linear_regression")?;
    ensure_same_len(x, y, "linear_regression")?;
    if x.len() < 2 {
        return Err(MathError::Statistics {
            context: "linear_regression",
            message: "requires at least two observations".to_string(),
        });
    }

    let x_mean = mean(x)?;
    let y_mean = mean(y)?;
    let ss_xx = x.iter().map(|value| (value - x_mean).powi(2)).sum::<f64>();
    if ss_xx <= f64::EPSILON {
        return Err(MathError::Statistics {
            context: "linear_regression",
            message: "x values must not all be identical".to_string(),
        });
    }
    let ss_xy = x
        .iter()
        .zip(y.iter())
        .map(|(lhs, rhs)| (lhs - x_mean) * (rhs - y_mean))
        .sum::<f64>();
    let slope = ss_xy / ss_xx;
    let intercept = y_mean - slope * x_mean;
    let correlation = correlation(x, y)?;

    let rss = x
        .iter()
        .zip(y.iter())
        .map(|(lhs, rhs)| {
            let predicted = intercept + slope * lhs;
            (rhs - predicted).powi(2)
        })
        .sum::<f64>();

    Ok(RegressionResult {
        slope,
        intercept,
        correlation,
        r_squared: correlation * correlation,
        residual_std_error: (rss / (x.len() - 2) as f64).sqrt(),
    })
}

fn validate_probability(probability: f64, context: &'static str) -> MathResult<()> {
    if !(0.0..1.0).contains(&probability) {
        return Err(MathError::InvalidRange {
            context,
            message: "probability must be between 0 and 1".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        chi_square_goodness_of_fit, linear_regression, mean_confidence_interval, t_test_mean,
        z_test_mean,
    };

    #[test]
    fn inference_helpers_return_reasonable_results() {
        let sample = [10.2, 9.9, 10.4, 10.1, 10.3];
        let ci = mean_confidence_interval(&sample, 0.95).unwrap();
        assert!(ci.lower < 10.0 && ci.upper > 10.0);

        let z_test = z_test_mean(&sample, 9.0, 0.5, 0.05).unwrap();
        assert!(z_test.reject_null);

        let t_test = t_test_mean(&sample, 10.0, 0.05).unwrap();
        assert!(!t_test.reject_null);

        let chi_square = chi_square_goodness_of_fit(&[20, 30, 50], &[0.2, 0.3, 0.5], 0.05).unwrap();
        assert!(!chi_square.reject_null);
    }

    #[test]
    fn linear_regression_matches_simple_line() {
        let result = linear_regression(&[1.0, 2.0, 3.0, 4.0], &[2.0, 4.0, 6.0, 8.0]).unwrap();
        assert!((result.slope - 2.0).abs() < 1e-12);
        assert!(result.intercept.abs() < 1e-12);
        assert!((result.r_squared - 1.0).abs() < 1e-12);
    }
}
