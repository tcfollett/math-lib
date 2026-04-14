use math_core::{MathError, MathResult};

const LANCZOS_COEFFS: [f64; 9] = [
    0.999_999_999_999_809_9,
    676.520_368_121_885_1,
    -1_259.139_216_722_402_8,
    771.323_428_777_653_1,
    -176.615_029_162_140_6,
    12.507_343_278_686_905,
    -0.138_571_095_265_720_12,
    9.984_369_578_019_572e-6,
    1.505_632_735_149_311_6e-7,
];

pub fn erf(x: f64) -> f64 {
    let sign = x.signum();
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let a1 = 0.254_829_592;
    let a2 = -0.284_496_736;
    let a3 = 1.421_413_741;
    let a4 = -1.453_152_027;
    let a5 = 1.061_405_429;
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
}

pub fn normal_cdf(x: f64) -> f64 {
    if x == 0.0 {
        return 0.5;
    }
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

pub fn inverse_normal_cdf(probability: f64) -> MathResult<f64> {
    if !(0.0..=1.0).contains(&probability) || probability == 0.0 || probability == 1.0 {
        return Err(MathError::InvalidRange {
            context: "inverse_normal_cdf",
            message: "probability must be strictly between 0 and 1".to_string(),
        });
    }

    let a = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];
    let b = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];
    let c = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];
    let d = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];
    let p_low = 0.024_25;
    let p_high = 1.0 - p_low;

    let value = if probability < p_low {
        let q = (-2.0 * probability.ln()).sqrt();
        (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    } else if probability <= p_high {
        let q = probability - 0.5;
        let r = q * q;
        (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
            / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - probability).ln()).sqrt();
        -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    };

    Ok(value)
}

pub fn log_gamma(z: f64) -> f64 {
    if z < 0.5 {
        return std::f64::consts::PI.ln()
            - (std::f64::consts::PI * z).sin().ln()
            - log_gamma(1.0 - z);
    }

    let z = z - 1.0;
    let mut x = LANCZOS_COEFFS[0];
    for (index, coefficient) in LANCZOS_COEFFS.iter().enumerate().skip(1) {
        x += coefficient / (z + index as f64);
    }
    let t = z + 7.5;
    0.5 * (2.0 * std::f64::consts::PI).ln() + (z + 0.5) * t.ln() - t + x.ln()
}

pub fn regularized_gamma_p(shape: f64, x: f64) -> MathResult<f64> {
    if shape <= 0.0 || x < 0.0 {
        return Err(MathError::InvalidDomain {
            context: "regularized_gamma_p",
            message: "shape must be positive and x must be non-negative".to_string(),
        });
    }
    if x == 0.0 {
        return Ok(0.0);
    }

    if x < shape + 1.0 {
        let mut term = 1.0 / shape;
        let mut sum = term;
        let mut n = 1.0;
        while term.abs() > 1e-15 * sum.abs() {
            term *= x / (shape + n);
            sum += term;
            n += 1.0;
        }
        Ok(sum * (-x + shape * x.ln() - log_gamma(shape)).exp())
    } else {
        Ok(1.0 - gamma_continued_fraction(shape, x)?)
    }
}

fn gamma_continued_fraction(shape: f64, x: f64) -> MathResult<f64> {
    let mut b = x + 1.0 - shape;
    let mut c = 1.0 / f64::MIN_POSITIVE;
    let mut d = 1.0 / b;
    let mut h = d;

    for iteration in 1..=200 {
        let i = iteration as f64;
        let an = -i * (i - shape);
        b += 2.0;
        d = an * d + b;
        if d.abs() < f64::MIN_POSITIVE {
            d = f64::MIN_POSITIVE;
        }
        c = b + an / c;
        if c.abs() < f64::MIN_POSITIVE {
            c = f64::MIN_POSITIVE;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < 1e-14 {
            return Ok(((-x + shape * x.ln() - log_gamma(shape)).exp()) * h);
        }
    }

    Err(MathError::NonConvergence {
        context: "regularized_gamma_p",
        iterations: 200,
        tolerance: 1e-14,
    })
}

pub fn regularized_beta(a: f64, b: f64, x: f64) -> MathResult<f64> {
    if a <= 0.0 || b <= 0.0 || !(0.0..=1.0).contains(&x) {
        return Err(MathError::InvalidDomain {
            context: "regularized_beta",
            message: "a and b must be positive and x must be in [0, 1]".to_string(),
        });
    }
    if x == 0.0 {
        return Ok(0.0);
    }
    if x == 1.0 {
        return Ok(1.0);
    }

    let bt =
        (log_gamma(a + b) - log_gamma(a) - log_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        Ok(bt * beta_continued_fraction(a, b, x)? / a)
    } else {
        Ok(1.0 - bt * beta_continued_fraction(b, a, 1.0 - x)? / b)
    }
}

fn beta_continued_fraction(a: f64, b: f64, x: f64) -> MathResult<f64> {
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < f64::MIN_POSITIVE {
        d = f64::MIN_POSITIVE;
    }
    d = 1.0 / d;
    let mut h = d;

    for m in 1..=200 {
        let m2 = 2 * m;
        let aa = m as f64 * (b - m as f64) * x / ((qam + m2 as f64) * (a + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < f64::MIN_POSITIVE {
            d = f64::MIN_POSITIVE;
        }
        c = 1.0 + aa / c;
        if c.abs() < f64::MIN_POSITIVE {
            c = f64::MIN_POSITIVE;
        }
        d = 1.0 / d;
        h *= d * c;

        let aa = -(a + m as f64) * (qab + m as f64) * x / ((a + m2 as f64) * (qap + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < f64::MIN_POSITIVE {
            d = f64::MIN_POSITIVE;
        }
        c = 1.0 + aa / c;
        if c.abs() < f64::MIN_POSITIVE {
            c = f64::MIN_POSITIVE;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;

        if (delta - 1.0).abs() < 1e-14 {
            return Ok(h);
        }
    }

    Err(MathError::NonConvergence {
        context: "regularized_beta",
        iterations: 200,
        tolerance: 1e-14,
    })
}

pub fn student_t_cdf(t: f64, degrees_of_freedom: f64) -> MathResult<f64> {
    if degrees_of_freedom <= 0.0 {
        return Err(MathError::InvalidDomain {
            context: "student_t_cdf",
            message: "degrees of freedom must be positive".to_string(),
        });
    }

    if t == 0.0 {
        return Ok(0.5);
    }

    let x = degrees_of_freedom / (degrees_of_freedom + t * t);
    let beta = regularized_beta(degrees_of_freedom / 2.0, 0.5, x)?;
    if t > 0.0 {
        Ok(1.0 - 0.5 * beta)
    } else {
        Ok(0.5 * beta)
    }
}

pub fn student_t_quantile(probability: f64, degrees_of_freedom: f64) -> MathResult<f64> {
    let z = inverse_normal_cdf(probability)?;
    let v = degrees_of_freedom;
    if v <= 0.0 {
        return Err(MathError::InvalidDomain {
            context: "student_t_quantile",
            message: "degrees of freedom must be positive".to_string(),
        });
    }

    let z2 = z * z;
    let z3 = z2 * z;
    let z5 = z3 * z2;
    let z7 = z5 * z2;
    Ok(z + (z3 + z) / (4.0 * v)
        + (5.0 * z5 + 16.0 * z3 + 3.0 * z) / (96.0 * v * v)
        + (3.0 * z7 + 19.0 * z5 + 17.0 * z3 - 15.0 * z) / (384.0 * v * v * v))
}

pub fn chi_square_cdf(x: f64, degrees_of_freedom: f64) -> MathResult<f64> {
    if degrees_of_freedom <= 0.0 {
        return Err(MathError::InvalidDomain {
            context: "chi_square_cdf",
            message: "degrees of freedom must be positive".to_string(),
        });
    }
    regularized_gamma_p(degrees_of_freedom / 2.0, x / 2.0)
}

pub fn log_factorial(value: u64) -> f64 {
    log_gamma(value as f64 + 1.0)
}

#[cfg(test)]
mod tests {
    use super::{chi_square_cdf, inverse_normal_cdf, normal_cdf, student_t_cdf};

    #[test]
    fn special_functions_hit_known_reference_points() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-12);
        assert!(inverse_normal_cdf(0.5).unwrap().abs() < 1e-12);
        assert!((student_t_cdf(0.0, 10.0).unwrap() - 0.5).abs() < 1e-12);
        assert!((chi_square_cdf(2.0, 2.0).unwrap() - (1.0 - (-1.0_f64).exp())).abs() < 1e-8);
    }
}
