use math_core::{MathError, MathResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegrationResult {
    pub value: f64,
    pub evaluations: usize,
}

pub fn trapezoidal<F>(f: F, lower: f64, upper: f64, steps: usize) -> MathResult<IntegrationResult>
where
    F: Fn(f64) -> f64,
{
    if steps == 0 {
        return Err(MathError::InvalidInput {
            context: "trapezoidal",
            message: "steps must be greater than zero".to_string(),
        });
    }

    let h = (upper - lower) / steps as f64;
    let mut total = 0.5 * (f(lower) + f(upper));
    for step in 1..steps {
        total += f(lower + step as f64 * h);
    }

    Ok(IntegrationResult {
        value: total * h,
        evaluations: steps + 1,
    })
}

pub fn simpson<F>(f: F, lower: f64, upper: f64, steps: usize) -> MathResult<IntegrationResult>
where
    F: Fn(f64) -> f64,
{
    if steps == 0 || steps % 2 != 0 {
        return Err(MathError::InvalidInput {
            context: "simpson",
            message: "steps must be a positive even number".to_string(),
        });
    }

    let h = (upper - lower) / steps as f64;
    let mut total = f(lower) + f(upper);
    for step in 1..steps {
        let x = lower + step as f64 * h;
        total += if step % 2 == 0 {
            2.0 * f(x)
        } else {
            4.0 * f(x)
        };
    }

    Ok(IntegrationResult {
        value: total * h / 3.0,
        evaluations: steps + 1,
    })
}

pub fn adaptive_simpson<F>(
    f: F,
    lower: f64,
    upper: f64,
    tolerance: f64,
    max_depth: usize,
) -> MathResult<IntegrationResult>
where
    F: Fn(f64) -> f64,
{
    if tolerance <= 0.0 {
        return Err(MathError::InvalidInput {
            context: "adaptive_simpson",
            message: "tolerance must be positive".to_string(),
        });
    }

    let fa = f(lower);
    let fb = f(upper);
    let midpoint = 0.5 * (lower + upper);
    let fm = f(midpoint);
    let initial = simpson_segment(lower, upper, fa, fm, fb);

    let (value, evaluations) =
        adaptive_simpson_recursive(&f, lower, upper, fa, fm, fb, initial, tolerance, max_depth)?;

    Ok(IntegrationResult { value, evaluations })
}

fn simpson_segment(lower: f64, upper: f64, f_lower: f64, f_mid: f64, f_upper: f64) -> f64 {
    (upper - lower) * (f_lower + 4.0 * f_mid + f_upper) / 6.0
}

#[allow(clippy::too_many_arguments)]
fn adaptive_simpson_recursive<F>(
    f: &F,
    lower: f64,
    upper: f64,
    f_lower: f64,
    f_mid: f64,
    f_upper: f64,
    whole: f64,
    tolerance: f64,
    depth: usize,
) -> MathResult<(f64, usize)>
where
    F: Fn(f64) -> f64,
{
    let midpoint = 0.5 * (lower + upper);
    let left_mid = 0.5 * (lower + midpoint);
    let right_mid = 0.5 * (midpoint + upper);

    let f_left_mid = f(left_mid);
    let f_right_mid = f(right_mid);
    let left = simpson_segment(lower, midpoint, f_lower, f_left_mid, f_mid);
    let right = simpson_segment(midpoint, upper, f_mid, f_right_mid, f_upper);
    let delta = left + right - whole;

    if depth == 0 {
        return Err(MathError::NonConvergence {
            context: "adaptive_simpson",
            iterations: 0,
            tolerance,
        });
    }

    if delta.abs() <= 15.0 * tolerance {
        return Ok((left + right + delta / 15.0, 2));
    }

    let (left_value, left_evals) = adaptive_simpson_recursive(
        f,
        lower,
        midpoint,
        f_lower,
        f_left_mid,
        f_mid,
        left,
        tolerance / 2.0,
        depth - 1,
    )?;
    let (right_value, right_evals) = adaptive_simpson_recursive(
        f,
        midpoint,
        upper,
        f_mid,
        f_right_mid,
        f_upper,
        right,
        tolerance / 2.0,
        depth - 1,
    )?;

    Ok((left_value + right_value, left_evals + right_evals + 2))
}

#[cfg(test)]
mod tests {
    use super::{adaptive_simpson, simpson, trapezoidal};

    #[test]
    fn integration_methods_match_known_integral() {
        let f = |x: f64| x.sin();

        let trap = trapezoidal(f, 0.0, std::f64::consts::PI, 2_000).unwrap();
        let simp = simpson(f, 0.0, std::f64::consts::PI, 2_000).unwrap();
        let adaptive = adaptive_simpson(f, 0.0, std::f64::consts::PI, 1e-10, 20).unwrap();

        assert!((trap.value - 2.0).abs() < 1e-4);
        assert!((simp.value - 2.0).abs() < 1e-8);
        assert!((adaptive.value - 2.0).abs() < 1e-8);
    }
}
