use crate::Vector;
use math_core::{
    MathError, MathResult, RealScalar, Scalar, default_epsilon, ensure_bounds, ensure_shape,
    ensure_square,
};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LUDecomposition<T> {
    lu: Matrix<T>,
    pivots: Vec<usize>,
    permutation_sign: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QRDecomposition<T> {
    pub q: Matrix<T>,
    pub r: Matrix<T>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> MathResult<Self> {
        if rows * cols != data.len() {
            return Err(MathError::DimensionMismatch {
                expected: format!("{rows}x{cols} matrix payload"),
                found: format!("{} values", data.len()),
            });
        }

        Ok(Self { rows, cols, data })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    pub fn row_slice(&self, row: usize) -> MathResult<&[T]> {
        ensure_bounds(row, self.rows, "matrix row access")?;
        let start = row * self.cols;
        Ok(&self.data[start..start + self.cols])
    }

    pub fn row(&self, row: usize) -> MathResult<Vector<T>>
    where
        T: Clone,
    {
        Ok(Vector::new(self.row_slice(row)?.to_vec()))
    }

    pub fn column(&self, col: usize) -> MathResult<Vector<T>>
    where
        T: Copy,
    {
        ensure_bounds(col, self.cols, "matrix column access")?;
        Ok(Vector::new(
            (0..self.rows).map(|row| self[(row, col)]).collect(),
        ))
    }

    pub fn map<U, F>(&self, mut f: F) -> Matrix<U>
    where
        F: FnMut(T) -> U,
        T: Copy,
    {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().copied().map(&mut f).collect(),
        }
    }

    pub fn zip_map<U, V, F>(&self, other: &Matrix<U>, mut f: F) -> MathResult<Matrix<V>>
    where
        F: FnMut(T, U) -> V,
        T: Copy,
        U: Copy,
    {
        ensure_shape(
            other.rows,
            other.cols,
            self.rows,
            self.cols,
            "matrix zip_map",
        )?;

        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .iter()
                .copied()
                .zip(other.data.iter().copied())
                .map(|(lhs, rhs)| f(lhs, rhs))
                .collect(),
        })
    }

    pub fn transpose(&self) -> Matrix<T>
    where
        T: Copy,
    {
        let mut data = Vec::with_capacity(self.data.len());
        for col in 0..self.cols {
            for row in 0..self.rows {
                data.push(self[(row, col)]);
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }

    fn offset(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

impl<T: Scalar> Matrix<T> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::zero(); rows * cols],
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::zeros(size, size);
        for index in 0..size {
            matrix[(index, index)] = T::one();
        }
        matrix
    }

    pub fn trace(&self) -> MathResult<T> {
        ensure_square(self.rows, self.cols, "matrix trace")?;
        Ok((0..self.rows).map(|index| self[(index, index)]).sum())
    }

    pub fn mul_vector(&self, vector: &Vector<T>) -> MathResult<Vector<T>> {
        if self.cols != vector.len() {
            return Err(MathError::DimensionMismatch {
                expected: format!("vector of length {}", self.cols),
                found: format!("vector of length {}", vector.len()),
            });
        }

        let mut result = vec![T::zero(); self.rows];
        for row in 0..self.rows {
            let mut acc = T::zero();
            for col in 0..self.cols {
                acc += self[(row, col)] * vector[col];
            }
            result[row] = acc;
        }

        Ok(Vector::new(result))
    }

    pub fn mul_matrix(&self, other: &Matrix<T>) -> MathResult<Matrix<T>> {
        if self.cols != other.rows {
            return Err(MathError::DimensionMismatch {
                expected: format!("inner dimensions {} == {}", self.cols, other.rows),
                found: format!("{} vs {}", self.cols, other.rows),
            });
        }

        let mut data = vec![T::zero(); self.rows * other.cols];
        for row in 0..self.rows {
            for col in 0..other.cols {
                let mut acc = T::zero();
                for inner in 0..self.cols {
                    acc += self[(row, inner)] * other[(inner, col)];
                }
                data[row * other.cols + col] = acc;
            }
        }

        Ok(Matrix {
            rows: self.rows,
            cols: other.cols,
            data,
        })
    }
}

impl<T: RealScalar> Matrix<T> {
    pub fn determinant(&self) -> MathResult<T> {
        let decomposition = self.lu_decomposition()?;
        let diagonal_product = (0..self.rows).fold(T::one(), |acc, index| {
            acc * decomposition.lu[(index, index)]
        });

        if decomposition.permutation_sign > 0 {
            Ok(diagonal_product)
        } else {
            Ok(-diagonal_product)
        }
    }

    pub fn inverse(&self) -> MathResult<Self> {
        ensure_square(self.rows, self.cols, "matrix inverse")?;
        let decomposition = self.lu_decomposition()?;
        let mut inverse = Self::zeros(self.rows, self.cols);

        for col in 0..self.cols {
            let mut basis = vec![T::zero(); self.rows];
            basis[col] = T::one();
            let solution = decomposition.solve(&Vector::new(basis))?;
            for row in 0..self.rows {
                inverse[(row, col)] = solution[row];
            }
        }

        Ok(inverse)
    }

    pub fn solve(&self, rhs: &Vector<T>) -> MathResult<Vector<T>> {
        self.lu_decomposition()?.solve(rhs)
    }

    pub fn rank(&self) -> usize {
        let epsilon = default_epsilon::<T>();
        let mut reduced = self.clone();
        let mut rank = 0;
        let mut pivot_row = 0;

        for col in 0..reduced.cols {
            if pivot_row >= reduced.rows {
                break;
            }

            let mut best_row = pivot_row;
            let mut best_value = reduced[(best_row, col)].abs();
            for row in (pivot_row + 1)..reduced.rows {
                let candidate = reduced[(row, col)].abs();
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row;
                }
            }

            if best_value <= epsilon {
                continue;
            }

            reduced.swap_rows(best_row, pivot_row);
            let pivot = reduced[(pivot_row, col)];
            for row in (pivot_row + 1)..reduced.rows {
                let factor = reduced[(row, col)] / pivot;
                reduced[(row, col)] = T::zero();
                for inner in (col + 1)..reduced.cols {
                    reduced[(row, inner)] =
                        reduced[(row, inner)] - factor * reduced[(pivot_row, inner)];
                }
            }

            rank += 1;
            pivot_row += 1;
        }

        rank
    }

    pub fn lu_decomposition(&self) -> MathResult<LUDecomposition<T>> {
        ensure_square(self.rows, self.cols, "LU decomposition")?;
        let epsilon = default_epsilon::<T>();
        let mut lu = self.clone();
        let mut pivots: Vec<usize> = (0..self.rows).collect();
        let mut sign = 1;

        for col in 0..self.cols {
            let mut pivot_row = col;
            let mut pivot_value = lu[(pivot_row, col)].abs();
            for row in (col + 1)..self.rows {
                let candidate = lu[(row, col)].abs();
                if candidate > pivot_value {
                    pivot_row = row;
                    pivot_value = candidate;
                }
            }

            if pivot_value <= epsilon {
                return Err(MathError::SingularMatrix);
            }

            if pivot_row != col {
                lu.swap_rows(pivot_row, col);
                pivots.swap(pivot_row, col);
                sign *= -1;
            }

            for row in (col + 1)..self.rows {
                let factor = lu[(row, col)] / lu[(col, col)];
                lu[(row, col)] = factor;
                for inner in (col + 1)..self.cols {
                    lu[(row, inner)] = lu[(row, inner)] - factor * lu[(col, inner)];
                }
            }
        }

        Ok(LUDecomposition {
            lu,
            pivots,
            permutation_sign: sign,
        })
    }

    pub fn qr_decomposition(&self) -> MathResult<QRDecomposition<T>> {
        if self.rows < self.cols {
            return Err(MathError::InvalidInput {
                context: "QR decomposition",
                message: "requires rows >= cols for the thin QR implementation".to_string(),
            });
        }

        let epsilon = default_epsilon::<T>();
        let mut q_columns: Vec<Vec<T>> = Vec::with_capacity(self.cols);
        let mut r = Matrix::zeros(self.cols, self.cols);

        for col in 0..self.cols {
            let a_col = self.column(col)?.into_vec();
            let mut v = a_col.clone();

            for prev in 0..col {
                let q_prev = &q_columns[prev];
                let projection = q_prev
                    .iter()
                    .copied()
                    .zip(a_col.iter().copied())
                    .map(|(lhs, rhs)| lhs * rhs)
                    .sum();
                r[(prev, col)] = projection;
                for row in 0..self.rows {
                    v[row] -= projection * q_prev[row];
                }
            }

            let norm = v
                .iter()
                .copied()
                .map(|value| value * value)
                .sum::<T>()
                .sqrt();
            if norm <= epsilon {
                return Err(MathError::InvalidInput {
                    context: "QR decomposition",
                    message: "encountered a linearly dependent column".to_string(),
                });
            }

            r[(col, col)] = norm;
            for value in &mut v {
                *value /= norm;
            }
            q_columns.push(v);
        }

        let mut q = Matrix::zeros(self.rows, self.cols);
        for col in 0..self.cols {
            for row in 0..self.rows {
                q[(row, col)] = q_columns[col][row];
            }
        }

        Ok(QRDecomposition { q, r })
    }

    fn swap_rows(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        for col in 0..self.cols {
            let lhs = self.offset(a, col);
            let rhs = self.offset(b, col);
            self.data.swap(lhs, rhs);
        }
    }
}

impl<T: RealScalar> LUDecomposition<T> {
    pub fn lower(&self) -> Matrix<T> {
        let mut matrix = Matrix::identity(self.lu.rows);
        for row in 1..self.lu.rows {
            for col in 0..row {
                matrix[(row, col)] = self.lu[(row, col)];
            }
        }
        matrix
    }

    pub fn upper(&self) -> Matrix<T> {
        let mut matrix = Matrix::zeros(self.lu.rows, self.lu.cols);
        for row in 0..self.lu.rows {
            for col in row..self.lu.cols {
                matrix[(row, col)] = self.lu[(row, col)];
            }
        }
        matrix
    }

    pub fn pivots(&self) -> &[usize] {
        &self.pivots
    }

    pub fn solve(&self, rhs: &Vector<T>) -> MathResult<Vector<T>> {
        if rhs.len() != self.lu.rows {
            return Err(MathError::DimensionMismatch {
                expected: format!("vector of length {}", self.lu.rows),
                found: format!("vector of length {}", rhs.len()),
            });
        }

        let mut y = vec![T::zero(); self.lu.rows];
        for row in 0..self.lu.rows {
            let mut value = rhs[self.pivots[row]];
            for (col, y_value) in y.iter().enumerate().take(row) {
                value -= self.lu[(row, col)] * *y_value;
            }
            y[row] = value;
        }

        let mut x = vec![T::zero(); self.lu.rows];
        for row in (0..self.lu.rows).rev() {
            let mut value = y[row];
            for (col, x_value) in x.iter().enumerate().take(self.lu.cols).skip(row + 1) {
                value -= self.lu[(row, col)] * *x_value;
            }
            let pivot = self.lu[(row, row)];
            if pivot.abs() <= default_epsilon::<T>() {
                return Err(MathError::SingularMatrix);
            }
            x[row] = value / pivot;
        }

        Ok(Vector::new(x))
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[self.offset(row, col)]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        let offset = self.offset(row, col);
        &mut self.data[offset]
    }
}

#[cfg(test)]
mod tests {
    use super::{Matrix, Vector};

    #[test]
    fn matrix_vector_multiplication_matches_expected_values() {
        let matrix = Matrix::<f64>::new(2, 2, vec![2.0, 1.0, 1.0, 3.0]).unwrap();
        let vector = Vector::new(vec![3.0_f64, 4.0]);

        let result = matrix.mul_vector(&vector).unwrap();
        assert_eq!(result, Vector::new(vec![10.0, 15.0]));
    }

    #[test]
    fn determinant_inverse_and_rank_work_for_dense_matrix() {
        let matrix =
            Matrix::<f64>::new(3, 3, vec![4.0, 7.0, 2.0, 3.0, 6.0, 1.0, 2.0, 5.0, 1.0]).unwrap();

        let determinant: f64 = matrix.determinant().unwrap();
        assert!((determinant - 3.0_f64).abs() < 1e-10);
        assert_eq!(matrix.rank(), 3);

        let inverse = matrix.inverse().unwrap();
        let identity = matrix.mul_matrix(&inverse).unwrap();
        for row in 0..3 {
            for col in 0..3 {
                let expected = if row == col { 1.0_f64 } else { 0.0_f64 };
                assert!((identity[(row, col)] - expected).abs() < 1e-8);
            }
        }
    }

    #[test]
    fn lu_and_qr_decompositions_round_trip_matrix() {
        let matrix = Matrix::<f64>::new(3, 2, vec![1.0, 1.0, 1.0, 0.0, 1.0, 2.0]).unwrap();
        let qr = matrix.qr_decomposition().unwrap();
        let reconstructed = qr.q.mul_matrix(&qr.r).unwrap();
        for row in 0..matrix.rows() {
            for col in 0..matrix.cols() {
                assert!((reconstructed[(row, col)] - matrix[(row, col)]).abs() < 1e-8);
            }
        }

        let square = Matrix::<f64>::new(2, 2, vec![3.0, 1.0, 1.0, 2.0]).unwrap();
        let lu = square.lu_decomposition().unwrap();
        let solution = lu.solve(&Vector::new(vec![9.0_f64, 8.0])).unwrap();
        assert!((solution[0] - 2.0_f64).abs() < 1e-8);
        assert!((solution[1] - 3.0_f64).abs() < 1e-8);
    }
}
