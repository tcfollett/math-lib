use math_core::{MathError, MathResult, RealScalar, Scalar, ensure_same_len};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn map<U, F>(&self, mut f: F) -> Vector<U>
    where
        F: FnMut(T) -> U,
        T: Copy,
    {
        Vector::new(self.data.iter().copied().map(&mut f).collect())
    }

    pub fn zip_map<U, V, F>(&self, other: &Vector<U>, mut f: F) -> MathResult<Vector<V>>
    where
        F: FnMut(T, U) -> V,
        T: Copy,
        U: Copy,
    {
        ensure_same_len(self.as_slice(), other.as_slice(), "vector zip_map")?;

        Ok(Vector::new(
            self.data
                .iter()
                .copied()
                .zip(other.data.iter().copied())
                .map(|(lhs, rhs)| f(lhs, rhs))
                .collect(),
        ))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn zeros(len: usize) -> Self {
        Self {
            data: vec![T::zero(); len],
        }
    }

    pub fn dot(&self, other: &Self) -> MathResult<T> {
        ensure_same_len(self.as_slice(), other.as_slice(), "vector dot product")?;

        Ok(self
            .data
            .iter()
            .copied()
            .zip(other.data.iter().copied())
            .map(|(lhs, rhs)| lhs * rhs)
            .sum())
    }
}

impl<T: RealScalar> Vector<T> {
    pub fn l1_norm(&self) -> T {
        self.data.iter().copied().map(|value| value.abs()).sum()
    }

    pub fn l2_norm(&self) -> T {
        self.data
            .iter()
            .copied()
            .map(|value| value * value)
            .sum::<T>()
            .sqrt()
    }

    pub fn inf_norm(&self) -> T {
        self.data
            .iter()
            .copied()
            .map(|value| value.abs())
            .fold(T::zero(), T::max)
    }

    pub fn normalize(&self) -> MathResult<Self> {
        let norm = self.l2_norm();
        if norm == T::zero() {
            return Err(MathError::InvalidDomain {
                context: "vector normalize",
                message: "cannot normalize the zero vector".to_string(),
            });
        }

        Ok(self.map(|value| value / norm))
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn vector_norms_match_expected_values() {
        let vector = Vector::new(vec![3.0, 4.0]);

        assert_eq!(vector.l1_norm(), 7.0);
        assert_eq!(vector.l2_norm(), 5.0);
        assert_eq!(vector.inf_norm(), 4.0);
    }

    #[test]
    fn dot_product_computes_expected_result() {
        let lhs = Vector::new(vec![1.0, 2.0, 3.0]);
        let rhs = Vector::new(vec![4.0, 5.0, 6.0]);

        assert_eq!(lhs.dot(&rhs).unwrap(), 32.0);
    }
}
