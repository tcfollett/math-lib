use math_core::{MathError, MathResult, ensure_non_empty};

#[derive(Debug, Clone, PartialEq)]
pub struct OdeStep {
    pub t: f64,
    pub y: Vec<f64>,
}

pub fn euler<F>(f: F, t0: f64, y0: Vec<f64>, t_end: f64, step: f64) -> MathResult<Vec<OdeStep>>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    integrate_ode(f, t0, y0, t_end, step, Stepper::Euler)
}

pub fn rk4<F>(f: F, t0: f64, y0: Vec<f64>, t_end: f64, step: f64) -> MathResult<Vec<OdeStep>>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    integrate_ode(f, t0, y0, t_end, step, Stepper::Rk4)
}

#[derive(Debug, Clone, Copy)]
enum Stepper {
    Euler,
    Rk4,
}

fn integrate_ode<F>(
    f: F,
    t0: f64,
    y0: Vec<f64>,
    t_end: f64,
    step: f64,
    stepper: Stepper,
) -> MathResult<Vec<OdeStep>>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    ensure_non_empty(&y0, "ode solver")?;
    if step <= 0.0 {
        return Err(MathError::InvalidInput {
            context: "ode solver",
            message: "step size must be positive".to_string(),
        });
    }
    if t_end < t0 {
        return Err(MathError::InvalidRange {
            context: "ode solver",
            message: "t_end must be greater than or equal to t0".to_string(),
        });
    }

    let mut steps = vec![OdeStep {
        t: t0,
        y: y0.clone(),
    }];
    let mut current_t = t0;
    let mut current_y = y0;

    while current_t < t_end {
        let actual_step = (t_end - current_t).min(step);
        current_y = match stepper {
            Stepper::Euler => euler_step(&f, current_t, &current_y, actual_step)?,
            Stepper::Rk4 => rk4_step(&f, current_t, &current_y, actual_step)?,
        };
        current_t += actual_step;
        steps.push(OdeStep {
            t: current_t,
            y: current_y.clone(),
        });
    }

    Ok(steps)
}

fn euler_step<F>(f: &F, t: f64, y: &[f64], step: f64) -> MathResult<Vec<f64>>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    let slope = f(t, y);
    ensure_same_dimension(y, &slope, "Euler step")?;
    Ok(y.iter()
        .zip(slope.iter())
        .map(|(value, delta)| value + step * delta)
        .collect())
}

fn rk4_step<F>(f: &F, t: f64, y: &[f64], step: f64) -> MathResult<Vec<f64>>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    let k1 = f(t, y);
    ensure_same_dimension(y, &k1, "RK4 step")?;

    let y2: Vec<f64> = y
        .iter()
        .zip(k1.iter())
        .map(|(value, delta)| value + 0.5 * step * delta)
        .collect();
    let k2 = f(t + 0.5 * step, &y2);
    ensure_same_dimension(y, &k2, "RK4 step")?;

    let y3: Vec<f64> = y
        .iter()
        .zip(k2.iter())
        .map(|(value, delta)| value + 0.5 * step * delta)
        .collect();
    let k3 = f(t + 0.5 * step, &y3);
    ensure_same_dimension(y, &k3, "RK4 step")?;

    let y4: Vec<f64> = y
        .iter()
        .zip(k3.iter())
        .map(|(value, delta)| value + step * delta)
        .collect();
    let k4 = f(t + step, &y4);
    ensure_same_dimension(y, &k4, "RK4 step")?;

    Ok((0..y.len())
        .map(|index| {
            y[index] + step * (k1[index] + 2.0 * k2[index] + 2.0 * k3[index] + k4[index]) / 6.0
        })
        .collect())
}

fn ensure_same_dimension(
    reference: &[f64],
    candidate: &[f64],
    context: &'static str,
) -> MathResult<()> {
    if reference.len() != candidate.len() {
        return Err(MathError::DimensionMismatch {
            expected: format!("{} values in {context}", reference.len()),
            found: format!("{} values", candidate.len()),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{euler, rk4};

    #[test]
    fn ode_solvers_track_exponential_growth() {
        let f = |_t: f64, y: &[f64]| vec![y[0]];
        let euler_solution = euler(f, 0.0, vec![1.0], 1.0, 0.01).unwrap();
        let rk4_solution = rk4(f, 0.0, vec![1.0], 1.0, 0.05).unwrap();

        let expected = std::f64::consts::E;
        assert!((euler_solution.last().unwrap().y[0] - expected).abs() < 0.05);
        assert!((rk4_solution.last().unwrap().y[0] - expected).abs() < 1e-4);
    }
}
