#![forbid(unsafe_code)]
#![doc = include_str!("../../../README.md")]

pub use math_core::{
    MathError, MathResult, RealScalar, Scalar, approx_eq, default_epsilon, relative_eq,
};

#[cfg(feature = "linalg")]
pub mod linalg {
    pub use math_linalg::*;
}

#[cfg(feature = "graph")]
pub mod graph {
    pub use math_graph::*;
}

#[cfg(feature = "numerics")]
pub mod numerics {
    pub use math_numerics::*;
}

#[cfg(feature = "stats")]
pub mod stats {
    pub use math_stats::*;
}

#[cfg(feature = "plot")]
pub mod plot {
    pub use math_plot::*;
}

#[cfg(feature = "linalg")]
pub use math_linalg::{LUDecomposition, Matrix, QRDecomposition, Vector};

#[cfg(feature = "graph")]
pub use math_graph::{EdgeId, Graph, MinimumSpanningTree, NodeId, ShortestPaths};

#[cfg(feature = "numerics")]
pub use math_numerics::{
    CubicSpline, IntegrationResult, LinearInterpolator, OdeStep, RootResult, SolverOptions,
    adaptive_simpson, bisection, derivative, euler, gradient, newton_raphson, rk4, secant,
    second_derivative, simpson, trapezoidal,
};

#[cfg(feature = "stats")]
pub use math_stats::{
    Binomial, ConfidenceInterval, Distribution, HypothesisTestResult, Normal, Poisson,
    RegressionResult, SummaryStatistics, chi_square_goodness_of_fit, correlation, covariance,
    linear_regression, mean, mean_confidence_interval, median, mode, quantile, sample_std_dev,
    sample_variance, std_dev, summary, t_test_mean, variance, z_test_mean,
};

#[cfg(feature = "plot")]
pub use math_plot::{Color, Plot, PlotOutput};
