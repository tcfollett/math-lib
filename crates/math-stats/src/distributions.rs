use crate::special::{inverse_normal_cdf, log_factorial, normal_cdf};
use math_core::{MathError, MathResult};
use rand::Rng;

pub trait Distribution {
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Normal {
    mean: f64,
    std_dev: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Binomial {
    trials: u64,
    probability: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Poisson {
    lambda: f64,
}

impl Normal {
    pub fn new(mean: f64, std_dev: f64) -> MathResult<Self> {
        if !mean.is_finite() || !std_dev.is_finite() || std_dev <= 0.0 {
            return Err(MathError::InvalidDomain {
                context: "Normal::new",
                message: "mean must be finite and std_dev must be positive".to_string(),
            });
        }
        Ok(Self { mean, std_dev })
    }

    pub fn pdf(&self, x: f64) -> f64 {
        let z = (x - self.mean) / self.std_dev;
        (-0.5 * z * z).exp() / (self.std_dev * (2.0 * std::f64::consts::PI).sqrt())
    }

    pub fn cdf(&self, x: f64) -> f64 {
        normal_cdf((x - self.mean) / self.std_dev)
    }

    pub fn inverse_cdf(&self, probability: f64) -> MathResult<f64> {
        Ok(self.mean + self.std_dev * inverse_normal_cdf(probability)?)
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let u1 = rng
            .r#gen::<f64>()
            .clamp(f64::MIN_POSITIVE, 1.0 - f64::EPSILON);
        let u2 = rng.r#gen::<f64>();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        self.mean + self.std_dev * z0
    }
}

impl Distribution for Normal {
    fn mean(&self) -> f64 {
        self.mean
    }

    fn variance(&self) -> f64 {
        self.std_dev * self.std_dev
    }
}

impl Binomial {
    pub fn new(trials: u64, probability: f64) -> MathResult<Self> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(MathError::InvalidDomain {
                context: "Binomial::new",
                message: "probability must be in [0, 1]".to_string(),
            });
        }
        Ok(Self {
            trials,
            probability,
        })
    }

    pub fn pmf(&self, successes: u64) -> MathResult<f64> {
        if successes > self.trials {
            return Ok(0.0);
        }
        if self.probability == 0.0 {
            return Ok(if successes == 0 { 1.0 } else { 0.0 });
        }
        if self.probability == 1.0 {
            return Ok(if successes == self.trials { 1.0 } else { 0.0 });
        }

        let failures = self.trials - successes;
        let log_probability =
            log_factorial(self.trials) - log_factorial(successes) - log_factorial(failures)
                + successes as f64 * self.probability.ln()
                + failures as f64 * (1.0 - self.probability).ln();
        Ok(log_probability.exp())
    }

    pub fn cdf(&self, successes: u64) -> MathResult<f64> {
        let upper = successes.min(self.trials);
        let mut total = 0.0;
        for k in 0..=upper {
            total += self.pmf(k)?;
        }
        Ok(total.min(1.0))
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        (0..self.trials)
            .map(|_| (rng.r#gen::<f64>() < self.probability) as u64)
            .sum()
    }
}

impl Distribution for Binomial {
    fn mean(&self) -> f64 {
        self.trials as f64 * self.probability
    }

    fn variance(&self) -> f64 {
        self.trials as f64 * self.probability * (1.0 - self.probability)
    }
}

impl Poisson {
    pub fn new(lambda: f64) -> MathResult<Self> {
        if !lambda.is_finite() || lambda <= 0.0 {
            return Err(MathError::InvalidDomain {
                context: "Poisson::new",
                message: "lambda must be positive and finite".to_string(),
            });
        }
        Ok(Self { lambda })
    }

    pub fn pmf(&self, value: u64) -> f64 {
        (-self.lambda + value as f64 * self.lambda.ln() - log_factorial(value)).exp()
    }

    pub fn cdf(&self, value: u64) -> f64 {
        (0..=value).map(|k| self.pmf(k)).sum::<f64>().min(1.0)
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        let threshold = (-self.lambda).exp();
        let mut product = 1.0;
        let mut count = 0u64;
        loop {
            count += 1;
            product *= rng.r#gen::<f64>();
            if product <= threshold {
                return count - 1;
            }
        }
    }
}

impl Distribution for Poisson {
    fn mean(&self) -> f64 {
        self.lambda
    }

    fn variance(&self) -> f64 {
        self.lambda
    }
}

#[cfg(test)]
mod tests {
    use super::{Binomial, Distribution, Normal, Poisson};
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn distributions_match_reference_values() {
        let normal = Normal::new(0.0, 1.0).unwrap();
        assert!((normal.pdf(0.0) - 0.398_942_28).abs() < 1e-6);
        assert!((normal.cdf(0.0) - 0.5).abs() < 1e-12);
        assert!(normal.inverse_cdf(0.5).unwrap().abs() < 1e-12);
        assert_eq!(normal.mean(), 0.0);
        assert_eq!(normal.variance(), 1.0);

        let binomial = Binomial::new(10, 0.5).unwrap();
        assert!((binomial.pmf(5).unwrap() - 0.246_093_75).abs() < 1e-10);

        let poisson = Poisson::new(3.0).unwrap();
        assert!((poisson.pmf(0) - (-3.0_f64).exp()).abs() < 1e-12);
    }

    #[test]
    fn sampling_produces_reasonable_values() {
        let mut rng = StdRng::seed_from_u64(42);
        let normal = Normal::new(0.0, 1.0).unwrap();
        let binomial = Binomial::new(12, 0.3).unwrap();
        let poisson = Poisson::new(4.0).unwrap();

        let normal_sample = normal.sample(&mut rng);
        let binomial_sample = binomial.sample(&mut rng);
        let poisson_sample = poisson.sample(&mut rng);

        assert!(normal_sample.is_finite());
        assert!(binomial_sample <= 12);
        assert!(poisson_sample < 20);
    }
}
