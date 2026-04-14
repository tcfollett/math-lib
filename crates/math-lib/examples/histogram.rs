use math_lib::{Color, Plot};

fn main() -> Result<(), math_lib::MathError> {
    let samples = vec![1.0, 1.2, 1.4, 1.8, 2.0, 2.2, 2.3, 2.8, 3.0, 3.1];
    let plot = Plot::new()
        .title("Sample Histogram")
        .x_label("value")
        .y_label("count")
        .add_histogram("samples", samples, 5, Color::ORANGE);

    let output = plot.render_svg()?;
    println!("generated {} SVG bytes", output.svg.len());
    Ok(())
}
