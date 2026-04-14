#![forbid(unsafe_code)]
#![doc = "Statistical summaries, distributions, inference, and regression."]

mod descriptive;
mod distributions;
mod inference;
mod special;

pub use descriptive::{
    SummaryStatistics, correlation, covariance, mean, median, mode, quantile, sample_std_dev,
    sample_variance, std_dev, summary, variance,
};
pub use distributions::{Binomial, Distribution, Normal, Poisson};
pub use inference::{
    ConfidenceInterval, HypothesisTestResult, RegressionResult, chi_square_goodness_of_fit,
    linear_regression, mean_confidence_interval, t_test_mean, z_test_mean,
};
