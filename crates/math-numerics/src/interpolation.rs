use math_core::{MathError, MathResult, ensure_non_empty, ensure_same_len};

#[derive(Debug, Clone, PartialEq)]
pub struct LinearInterpolator {
    x: Vec<f64>,
    y: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CubicSpline {
    x: Vec<f64>,
    a: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    d: Vec<f64>,
}

impl LinearInterpolator {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> MathResult<Self> {
        validate_pairs(&x, &y, "linear interpolation")?;
        Ok(Self { x, y })
    }

    pub fn interpolate(&self, xq: f64) -> MathResult<f64> {
        let index = bracket_index(&self.x, xq, "linear interpolation")?;
        let x0 = self.x[index];
        let x1 = self.x[index + 1];
        let y0 = self.y[index];
        let y1 = self.y[index + 1];
        let t = (xq - x0) / (x1 - x0);
        Ok(y0 + t * (y1 - y0))
    }
}

impl CubicSpline {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> MathResult<Self> {
        validate_pairs(&x, &y, "cubic spline")?;
        if x.len() < 3 {
            return Err(MathError::InvalidInput {
                context: "cubic spline",
                message: "requires at least three points".to_string(),
            });
        }

        let n = x.len();
        let mut h = vec![0.0; n - 1];
        for index in 0..(n - 1) {
            h[index] = x[index + 1] - x[index];
        }

        let mut alpha = vec![0.0; n];
        for index in 1..(n - 1) {
            alpha[index] = (3.0 / h[index]) * (y[index + 1] - y[index])
                - (3.0 / h[index - 1]) * (y[index] - y[index - 1]);
        }

        let mut l = vec![1.0; n];
        let mut mu = vec![0.0; n];
        let mut z = vec![0.0; n];
        let mut c = vec![0.0; n];
        let mut b = vec![0.0; n - 1];
        let mut d = vec![0.0; n - 1];

        for index in 1..(n - 1) {
            l[index] = 2.0 * (x[index + 1] - x[index - 1]) - h[index - 1] * mu[index - 1];
            mu[index] = h[index] / l[index];
            z[index] = (alpha[index] - h[index - 1] * z[index - 1]) / l[index];
        }

        for index in (0..(n - 1)).rev() {
            c[index] = z[index] - mu[index] * c[index + 1];
            b[index] = (y[index + 1] - y[index]) / h[index]
                - h[index] * (c[index + 1] + 2.0 * c[index]) / 3.0;
            d[index] = (c[index + 1] - c[index]) / (3.0 * h[index]);
        }

        Ok(Self { x, a: y, b, c, d })
    }

    pub fn interpolate(&self, xq: f64) -> MathResult<f64> {
        let index = bracket_index(&self.x, xq, "cubic spline")?;
        let dx = xq - self.x[index];
        Ok(self.a[index]
            + self.b[index] * dx
            + self.c[index] * dx * dx
            + self.d[index] * dx * dx * dx)
    }
}

fn validate_pairs(x: &[f64], y: &[f64], context: &'static str) -> MathResult<()> {
    ensure_non_empty(x, context)?;
    ensure_same_len(x, y, context)?;
    if x.len() < 2 {
        return Err(MathError::InvalidInput {
            context,
            message: "requires at least two points".to_string(),
        });
    }

    for window in x.windows(2) {
        if window[0] >= window[1] {
            return Err(MathError::InvalidInput {
                context,
                message: "x coordinates must be strictly increasing".to_string(),
            });
        }
    }

    Ok(())
}

fn bracket_index(x: &[f64], query: f64, context: &'static str) -> MathResult<usize> {
    if query < x[0] || query > x[x.len() - 1] {
        return Err(MathError::InvalidRange {
            context,
            message: format!(
                "query {query} is outside the interpolation range [{}, {}]",
                x[0],
                x[x.len() - 1]
            ),
        });
    }

    let index = x.partition_point(|value| *value <= query);
    Ok(index.saturating_sub(1).min(x.len() - 2))
}

#[cfg(test)]
mod tests {
    use super::{CubicSpline, LinearInterpolator};

    #[test]
    fn interpolators_recover_simple_curves() {
        let linear = LinearInterpolator::new(vec![0.0, 1.0, 2.0], vec![0.0, 2.0, 4.0]).unwrap();
        assert!((linear.interpolate(1.5).unwrap() - 3.0).abs() < 1e-12);

        let spline = CubicSpline::new(vec![0.0, 1.0, 2.0, 3.0], vec![0.0, 1.0, 0.0, 1.0]).unwrap();
        assert!((spline.interpolate(1.5).unwrap() - 0.5).abs() < 0.25);
    }
}
