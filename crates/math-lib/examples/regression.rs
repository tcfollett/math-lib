use math_lib::{linear_regression, mean_confidence_interval};

fn main() -> Result<(), math_lib::MathError> {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [2.1, 4.1, 6.0, 7.8, 10.2];

    let regression = linear_regression(&x, &y)?;
    let interval = mean_confidence_interval(&y, 0.95)?;

    println!(
        "slope = {:.3}, intercept = {:.3}",
        regression.slope, regression.intercept
    );
    println!(
        "95% CI for mean(y) = [{:.3}, {:.3}]",
        interval.lower, interval.upper
    );
    Ok(())
}
