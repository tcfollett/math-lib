use math_lib::{Matrix, Vector};

fn main() -> Result<(), math_lib::MathError> {
    let system = Matrix::new(3, 3, vec![3.0, 2.0, -1.0, 2.0, -2.0, 4.0, -1.0, 0.5, -1.0])?;
    let rhs = Vector::new(vec![1.0, -2.0, 0.0]);
    let solution = system.solve(&rhs)?;

    println!("solution = {:?}", solution.as_slice());
    Ok(())
}
