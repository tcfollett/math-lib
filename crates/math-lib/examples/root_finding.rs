use math_lib::{SolverOptions, bisection, newton_raphson, secant};

fn main() -> Result<(), math_lib::MathError> {
    let options = SolverOptions::default();
    let f = |x: f64| x.cos() - x;
    let df = |x: f64| -x.sin() - 1.0;

    println!("bisection root = {}", bisection(f, 0.0, 1.0, options)?.root);
    println!(
        "newton root = {}",
        newton_raphson(f, df, 0.5, options)?.root
    );
    println!("secant root = {}", secant(f, 0.0, 1.0, options)?.root);
    Ok(())
}
