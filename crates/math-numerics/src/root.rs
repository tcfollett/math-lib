use math_core::{MathError, MathResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolverOptions {
    pub tolerance: f64,
    pub max_iterations: usize,
}

impl Default for SolverOptions {
    fn default() -> Self {
        Self {
            tolerance: 1e-8,
            max_iterations: 128,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootResult {
    pub root: f64,
    pub value: f64,
    pub iterations: usize,
    pub converged: bool,
}

pub fn bisection<F>(
    f: F,
    mut lower: f64,
    mut upper: f64,
    options: SolverOptions,
) -> MathResult<RootResult>
where
    F: Fn(f64) -> f64,
{
    if lower >= upper {
        return Err(MathError::InvalidRange {
            context: "bisection",
            message: "lower bound must be strictly less than upper bound".to_string(),
        });
    }

    let mut f_lower = f(lower);
    let f_upper = f(upper);
    if f_lower * f_upper > 0.0 {
        return Err(MathError::InvalidDomain {
            context: "bisection",
            message: "interval does not bracket a root".to_string(),
        });
    }

    for iteration in 1..=options.max_iterations {
        let midpoint = 0.5 * (lower + upper);
        let f_mid = f(midpoint);

        if f_mid.abs() <= options.tolerance || 0.5 * (upper - lower) <= options.tolerance {
            return Ok(RootResult {
                root: midpoint,
                value: f_mid,
                iterations: iteration,
                converged: true,
            });
        }

        if f_lower * f_mid < 0.0 {
            upper = midpoint;
        } else {
            lower = midpoint;
            f_lower = f_mid;
        }
    }

    let root = 0.5 * (lower + upper);
    Err(MathError::NonConvergence {
        context: "bisection",
        iterations: options.max_iterations,
        tolerance: options.tolerance,
    }
    .into_non_convergence_with_root(root, f(root)))
}

pub fn newton_raphson<F, G>(
    f: F,
    derivative: G,
    mut guess: f64,
    options: SolverOptions,
) -> MathResult<RootResult>
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    for iteration in 1..=options.max_iterations {
        let value = f(guess);
        if value.abs() <= options.tolerance {
            return Ok(RootResult {
                root: guess,
                value,
                iterations: iteration,
                converged: true,
            });
        }

        let slope = derivative(guess);
        if slope.abs() <= f64::EPSILON {
            return Err(MathError::InvalidDomain {
                context: "newton_raphson",
                message: "derivative evaluated to zero".to_string(),
            });
        }

        let next = guess - value / slope;
        if (next - guess).abs() <= options.tolerance {
            let next_value = f(next);
            return Ok(RootResult {
                root: next,
                value: next_value,
                iterations: iteration,
                converged: true,
            });
        }

        guess = next;
    }

    Err(MathError::NonConvergence {
        context: "newton_raphson",
        iterations: options.max_iterations,
        tolerance: options.tolerance,
    })
}

pub fn secant<F>(f: F, mut x0: f64, mut x1: f64, options: SolverOptions) -> MathResult<RootResult>
where
    F: Fn(f64) -> f64,
{
    let mut f0 = f(x0);
    let mut f1 = f(x1);

    for iteration in 1..=options.max_iterations {
        if f1.abs() <= options.tolerance {
            return Ok(RootResult {
                root: x1,
                value: f1,
                iterations: iteration,
                converged: true,
            });
        }

        let denominator = f1 - f0;
        if denominator.abs() <= f64::EPSILON {
            return Err(MathError::InvalidDomain {
                context: "secant",
                message: "consecutive points produced the same function value".to_string(),
            });
        }

        let next = x1 - f1 * (x1 - x0) / denominator;
        if (next - x1).abs() <= options.tolerance {
            let next_value = f(next);
            return Ok(RootResult {
                root: next,
                value: next_value,
                iterations: iteration,
                converged: true,
            });
        }

        x0 = x1;
        f0 = f1;
        x1 = next;
        f1 = f(x1);
    }

    Err(MathError::NonConvergence {
        context: "secant",
        iterations: options.max_iterations,
        tolerance: options.tolerance,
    })
}

trait NonConvergenceRoot {
    fn into_non_convergence_with_root(self, _root: f64, _value: f64) -> Self;
}

impl NonConvergenceRoot for MathError {
    fn into_non_convergence_with_root(self, _root: f64, _value: f64) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{SolverOptions, bisection, newton_raphson, secant};

    #[test]
    fn root_finders_converge_for_simple_polynomial() {
        let options = SolverOptions::default();
        let f = |x: f64| x * x - 2.0;

        let bisected = bisection(f, 0.0, 2.0, options).unwrap();
        let newton = newton_raphson(f, |x| 2.0 * x, 1.0, options).unwrap();
        let secant_root = secant(f, 0.0, 2.0, options).unwrap();

        let expected = 2.0_f64.sqrt();
        assert!((bisected.root - expected).abs() < 1e-6);
        assert!((newton.root - expected).abs() < 1e-6);
        assert!((secant_root.root - expected).abs() < 1e-6);
    }
}
