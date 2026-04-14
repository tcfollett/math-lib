use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    DimensionMismatch {
        expected: String,
        found: String,
    },
    InvalidInput {
        context: &'static str,
        message: String,
    },
    InvalidDomain {
        context: &'static str,
        message: String,
    },
    InvalidRange {
        context: &'static str,
        message: String,
    },
    SingularMatrix,
    NonConvergence {
        context: &'static str,
        iterations: usize,
        tolerance: f64,
    },
    InvalidGraph {
        message: String,
    },
    EmptyInput {
        context: &'static str,
    },
    Statistics {
        context: &'static str,
        message: String,
    },
    Io(String),
}

pub type MathResult<T> = Result<T, MathError>;

impl Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch { expected, found } => {
                write!(f, "dimension mismatch: expected {expected}, found {found}")
            }
            Self::InvalidInput { context, message } => {
                write!(f, "invalid input in {context}: {message}")
            }
            Self::InvalidDomain { context, message } => {
                write!(f, "domain error in {context}: {message}")
            }
            Self::InvalidRange { context, message } => {
                write!(f, "range error in {context}: {message}")
            }
            Self::SingularMatrix => write!(f, "matrix is singular"),
            Self::NonConvergence {
                context,
                iterations,
                tolerance,
            } => write!(
                f,
                "{context} did not converge after {iterations} iterations (tolerance {tolerance})"
            ),
            Self::InvalidGraph { message } => write!(f, "invalid graph: {message}"),
            Self::EmptyInput { context } => write!(f, "{context} requires at least one value"),
            Self::Statistics { context, message } => {
                write!(f, "statistics error in {context}: {message}")
            }
            Self::Io(message) => write!(f, "i/o error: {message}"),
        }
    }
}

impl Error for MathError {}

impl From<std::io::Error> for MathError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}
