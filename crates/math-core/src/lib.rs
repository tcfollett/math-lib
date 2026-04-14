#![forbid(unsafe_code)]
#![doc = "Shared traits, validation, and error types used throughout the math workspace."]

pub mod error;
pub mod tolerance;
pub mod traits;
pub mod validate;

pub use error::{MathError, MathResult};
pub use tolerance::{approx_eq, default_epsilon, relative_eq};
pub use traits::{RealScalar, Scalar};
pub use validate::{ensure_bounds, ensure_non_empty, ensure_same_len, ensure_shape, ensure_square};
