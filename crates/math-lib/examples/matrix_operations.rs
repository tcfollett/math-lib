use math_lib::{Matrix, Vector};

fn main() -> Result<(), math_lib::MathError> {
    let matrix = Matrix::new(2, 2, vec![2.0, 1.0, 1.0, 3.0])?;
    let vector = Vector::new(vec![3.0, 4.0]);
    let projected = matrix.mul_vector(&vector)?;

    println!("A * x = {:?}", projected.as_slice());
    println!("det(A) = {}", matrix.determinant()?);
    println!("trace(A) = {}", matrix.trace()?);
    Ok(())
}
