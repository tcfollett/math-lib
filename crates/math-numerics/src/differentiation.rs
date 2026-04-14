use math_core::{MathError, MathResult, ensure_non_empty};

pub fn derivative<F>(f: F, x: f64, h: f64) -> MathResult<f64>
where
    F: Fn(f64) -> f64,
{
    if h <= 0.0 {
        return Err(MathError::InvalidInput {
            context: "derivative",
            message: "step size must be positive".to_string(),
        });
    }

    Ok((f(x + h) - f(x - h)) / (2.0 * h))
}

pub fn second_derivative<F>(f: F, x: f64, h: f64) -> MathResult<f64>
where
    F: Fn(f64) -> f64,
{
    if h <= 0.0 {
        return Err(MathError::InvalidInput {
            context: "second_derivative",
            message: "step size must be positive".to_string(),
        });
    }

    Ok((f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h))
}

pub fn gradient<F>(f: F, point: &[f64], h: f64) -> MathResult<Vec<f64>>
where
    F: Fn(&[f64]) -> f64,
{
    ensure_non_empty(point, "gradient")?;
    if h <= 0.0 {
        return Err(MathError::InvalidInput {
            context: "gradient",
            message: "step size must be positive".to_string(),
        });
    }

    let mut gradient = Vec::with_capacity(point.len());
    for index in 0..point.len() {
        let mut forward = point.to_vec();
        let mut backward = point.to_vec();
        forward[index] += h;
        backward[index] -= h;
        gradient.push((f(&forward) - f(&backward)) / (2.0 * h));
    }

    Ok(gradient)
}

#[cfg(test)]
mod tests {
    use super::{derivative, gradient, second_derivative};

    #[test]
    fn finite_difference_derivatives_are_reasonable() {
        let h = 1e-5;
        let f = |x: f64| x.powi(3);
        assert!((derivative(f, 2.0, h).unwrap() - 12.0).abs() < 1e-4);
        assert!((second_derivative(f, 2.0, h).unwrap() - 12.0).abs() < 1e-3);

        let g = |point: &[f64]| point[0] * point[0] + 3.0 * point[1];
        let grad = gradient(g, &[2.0, 1.0], h).unwrap();
        assert!((grad[0] - 4.0).abs() < 1e-4);
        assert!((grad[1] - 3.0).abs() < 1e-4);
    }
}
