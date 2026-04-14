use crate::{MathError, MathResult};

pub fn ensure_non_empty<T>(values: &[T], context: &'static str) -> MathResult<()> {
    if values.is_empty() {
        return Err(MathError::EmptyInput { context });
    }

    Ok(())
}

pub fn ensure_same_len<T, U>(left: &[T], right: &[U], context: &'static str) -> MathResult<()> {
    if left.len() != right.len() {
        return Err(MathError::DimensionMismatch {
            expected: format!("matching lengths in {context}"),
            found: format!("{} vs {}", left.len(), right.len()),
        });
    }

    Ok(())
}

pub fn ensure_shape(
    rows: usize,
    cols: usize,
    expected_rows: usize,
    expected_cols: usize,
    context: &'static str,
) -> MathResult<()> {
    if rows != expected_rows || cols != expected_cols {
        return Err(MathError::DimensionMismatch {
            expected: format!("{expected_rows}x{expected_cols} in {context}"),
            found: format!("{rows}x{cols}"),
        });
    }

    Ok(())
}

pub fn ensure_square(rows: usize, cols: usize, context: &'static str) -> MathResult<()> {
    if rows != cols {
        return Err(MathError::DimensionMismatch {
            expected: format!("square matrix in {context}"),
            found: format!("{rows}x{cols}"),
        });
    }

    Ok(())
}

pub fn ensure_bounds(index: usize, len: usize, context: &'static str) -> MathResult<()> {
    if index >= len {
        return Err(MathError::InvalidInput {
            context,
            message: format!("index {index} out of bounds for length {len}"),
        });
    }

    Ok(())
}
