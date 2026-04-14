# `math_lib`

`math_lib` is a modular Rust math workspace that aims to cover the core building blocks of scientific computing:

- dense linear algebra with vectors, matrices, LU, and QR
- graph algorithms with traversal, shortest paths, topological sorting, and MSTs
- numerical methods for roots, integration, differentiation, interpolation, and ODEs
- descriptive statistics, probability distributions, inference, and regression
- static plotting for functions, datasets, and histograms

The public entrypoint is the `math-lib` crate, imported in code as `math_lib`.

## Quick start

```rust
use math_lib::{Matrix, Plot, Color, SolverOptions, bisection};

fn main() -> Result<(), math_lib::MathError> {
    let matrix = Matrix::new(2, 2, vec![2.0, 1.0, 1.0, 3.0])?;
    let det = matrix.determinant()?;

    let root = bisection(|x| x.cos() - x, 0.0, 1.0, SolverOptions::default())?;

    let plot = Plot::new()
        .title("sin(x)")
        .x_label("x")
        .y_label("sin(x)")
        .add_function("sin(x)", |x| x.sin(), 0.0, std::f64::consts::TAU, 128, Color::BLUE)?;

    let svg = plot.render_svg()?;
    println!("det = {det}, root = {}, svg bytes = {}", root.root, svg.svg.len());
    Ok(())
}
```

## Workspace layout

- `math-core`: shared errors, scalar traits, tolerance helpers, validation
- `math-linalg`: dense vectors, matrices, LU/QR, solve/inverse/rank
- `math-graph`: adjacency-list graphs and graph algorithms
- `math-numerics`: root finding, integration, differentiation, interpolation, ODEs
- `math-stats`: summaries, distributions, inference, regression
- `math-plot`: static SVG plotting with optional PNG output
- `math-lib`: umbrella crate with feature-gated re-exports
