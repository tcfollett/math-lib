use crate::traits::RealScalar;

pub fn default_epsilon<T: RealScalar>() -> T {
    T::epsilon() * T::from_f64(16.0).expect("real scalar should support f64 conversion")
}

pub fn approx_eq<T: RealScalar>(lhs: T, rhs: T, epsilon: T) -> bool {
    (lhs - rhs).abs() <= epsilon
}

pub fn relative_eq<T: RealScalar>(lhs: T, rhs: T, epsilon: T) -> bool {
    let scale = T::one().max(lhs.abs()).max(rhs.abs());
    (lhs - rhs).abs() <= epsilon * scale
}
