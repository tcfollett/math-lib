#![forbid(unsafe_code)]
#![doc = "Numerical methods for roots, integration, differentiation, interpolation, and ODEs."]

mod differentiation;
mod integration;
mod interpolation;
mod ode;
mod root;

pub use differentiation::{derivative, gradient, second_derivative};
pub use integration::{IntegrationResult, adaptive_simpson, simpson, trapezoidal};
pub use interpolation::{CubicSpline, LinearInterpolator};
pub use ode::{OdeStep, euler, rk4};
pub use root::{RootResult, SolverOptions, bisection, newton_raphson, secant};
