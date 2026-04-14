use math_lib::{Color, Plot};

fn main() -> Result<(), math_lib::MathError> {
    let plot = Plot::new()
        .title("Sine Wave")
        .x_label("x")
        .y_label("sin(x)")
        .add_function(
            "sin(x)",
            |x| x.sin(),
            0.0,
            std::f64::consts::TAU,
            200,
            Color::BLUE,
        )?;

    let output = plot.render_svg()?;
    println!("generated {} SVG bytes", output.svg.len());
    Ok(())
}
