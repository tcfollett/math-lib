# `math_lib`

`math_lib` is a modular Rust mathematics library and workspace designed for scientific computing, numerical analysis, graph algorithms, statistics, and plotting. The public package is published as `math-lib` and imported in Rust code as `math_lib`.

This library was created using Codex GPT-5.4 and subsequently refined, tested, and organized into a robust multi-crate Rust workspace.

## Overview

The objective of `math_lib` is to provide a comprehensive and coherent mathematical toolkit rather than a single narrow subsystem. The architecture prioritizes:

- Shared error handling and scalar traits
- Dense linear algebra
- Graph theory algorithms
- Numerical methods
- Descriptive and inferential statistics
- Static plotting for mathematical and statistical output

The project is intentionally structured as a Cargo workspace, allowing each mathematical domain to evolve independently while maintaining a unified top-level API.

## Architecture

The repository is partitioned into focused crates:

| Crate | Purpose |
| --- | --- |
| `math-core` | Shared `MathError`, validation helpers, tolerance utilities, and scalar trait definitions. |
| `math-linalg` | Dense vectors and matrices, matrix arithmetic, decomposition routines, inversion, rank, determinant, and system solving. |
| `math-graph` | Adjacency-list graph representation with traversal, shortest paths, topological sorting, connectivity analysis, and minimum spanning trees. |
| `math-numerics` | Root finding, integration, differentiation, interpolation, and ordinary differential equation solvers. |
| `math-stats` | Summary statistics, distributions, confidence intervals, hypothesis testing, and linear regression. |
| `math-plot` | Static SVG plotting with optional PNG output. |
| `math-lib` | Umbrella crate that re-exports the public API behind feature flags. |

The root workspace utilizes Cargo resolver 2 and targets stable Rust with `std`.

## Feature Walkthrough

### Shared Core Abstractions

`math-core` defines the foundational vocabulary utilized across the workspace. Instead of each crate implementing redundant error handling or numeric assumptions, all crates build upon:

- `MathError` for consistent failure reporting.
- `MathResult<T>` for fallible operations.
- `Scalar` and `RealScalar` trait layers for generic numeric code.
- Validation macros and helpers such as `ensure_non_empty`, `ensure_same_len`, and `ensure_square`.
- Tolerance helpers for approximate and relative floating-point equality.

This shared layer guarantees structural preconditions essential for scientific computing. For instance, matrix inversion enforces square non-singular matrices, and correlation calculations enforce matching sample lengths with non-zero spread.

### Linear Algebra

`math-linalg` is built around dense row-major storage utilizing `Vector<T>` and `Matrix<T>`.

Capabilities include:

- Vector construction, indexing, iteration, mapping, and zip-style transforms.
- Dot products and norm calculations.
- Matrix construction and shape validation.
- Transpose, trace, matrix-vector multiplication, and matrix-matrix multiplication.
- Determinant calculation and inversion for square real-valued matrices.
- Rank estimation via Gaussian elimination.
- Linear system solving.
- LU decomposition.
- QR decomposition via Gram-Schmidt orthogonalization.

The API is structured to seamlessly handle standard linear algebra operations, such as solving systems of equations:

$$A \mathbf{x} = \mathbf{b}$$

And computing vector dot products:

$$\mathbf{x} \cdot \mathbf{y} = \sum_{i=1}^{n} x_i y_i$$

### Graph Theory

`math-graph` treats graph theory as a distinct, first-class subsystem. It provides:

- Directed and undirected graph constructors.
- Stable `NodeId` and `EdgeId` handlers.
- Weighted adjacency-list storage.
- Breadth-first search (BFS) and depth-first search (DFS).
- Connected component analysis.
- Topological sorting for directed acyclic graphs (DAGs).
- Dijkstra's algorithm for shortest paths.
- Minimum spanning trees for weighted undirected graphs.

### Numerical Methods

`math-numerics` implements core scientific computing algorithms:

- Root finding via bisection, Newton-Raphson, and secant methods.
- Numerical integration using trapezoidal, Simpson's, and adaptive Simpson's rules.
- Finite-difference approximations for first and second derivatives.
- Finite-difference gradients for multivariate scalar functions.
- Linear and cubic spline interpolation.
- Ordinary Differential Equation (ODE) solvers including Euler and Runge-Kutta 4 (RK4).

The Newton-Raphson solver implements the standard iterative approach:

$$x_{n+1} = x_n - \frac{f(x_n)}{f'(x_n)}$$

Numerical integration via Simpson's rule targets approximations structured as:

$$\int_a^b f(x)\,dx \approx \frac{h}{3} \left[ f(x_0) + 4 \sum_{k=1}^{n/2} f(x_{2k-1}) + 2 \sum_{k=1}^{n/2-1} f(x_{2k}) + f(x_n) \right]$$

### Statistical Methods

`math-stats` supports both exploratory data analysis and classical statistical inference.

Descriptive statistics include:

- Mean, median, and mode.
- Quantiles.
- Population and sample variance / standard deviation.
- Covariance and correlation.

Core statistical implementations align with standard notation:

$$\mu = \frac{1}{n}\sum_{i=1}^{n} x_i \qquad s^2 = \frac{1}{n-1}\sum_{i=1}^{n}(x_i - \bar{x})^2$$

Probability distribution support covers:

- Normal, Binomial, and Poisson distributions.
- PDF/PMF and CDF evaluation.
- Inverse CDF (quantile function) approximations.
- Random sampling.

Inferential statistics cover:

- Confidence intervals for the mean.
- Z-tests and one-sample T-tests.
- Chi-square goodness-of-fit tests.
- Simple linear regression.

The simple linear regression fits the standard least-squares slope:

$$\hat{y} = \beta_0 + \beta_1 x \qquad \beta_1 = \frac{\sum_{i=1}^{n}(x_i-\bar{x})(y_i-\bar{y})}{\sum_{i=1}^{n}(x_i-\bar{x})^2}$$

### Plotting

`math-plot` delivers static visualization tools optimized for scientific output. Features include:

- Line and scatter series.
- Histograms.
- Sampled function visualization.
- Configurable axes, labels, titles, and dimensions.
- Native SVG rendering.
- Optional PNG output.

The API relies on high-level constructs (`Plot`, `Color`, `PlotOutput`) rather than low-level rendering context, ensuring datasets and analytical models can be visualized with minimal boilerplate.

### Feature Flags

`math-lib` serves as the primary entry point, re-exporting internal crates based on configured feature flags.

| Feature | Description |
| --- | --- |
| `linalg` | Enables dense linear algebra APIs. |
| `graph` | Enables graph theory data structures and algorithms. |
| `numerics` | Enables numerical methods. |
| `stats` | Enables the statistics subsystem. |
| `plot` | Enables static plotting. |
| `png` | Enables rasterized PNG output via the plotting crate. |

*Note: Default features enable `linalg`, `graph`, `numerics`, `stats`, and `plot`.*

## Public API Shape

Primary user-facing structures include:

- `MathError`
- `Vector<T>`, `Matrix<T>`
- `Graph<N, E>`, `NodeId`, `EdgeId`
- `SolverOptions`, `RootResult`
- `SummaryStatistics`, `ConfidenceInterval`, `HypothesisTestResult`, `RegressionResult`
- `Plot`, `PlotOutput`, `Color`

The design philosophy emphasizes generic numeric traits where applicable, concrete `f64` boundaries for real-valued scientific functions, and explicit error handling via `Result`.

## Technologies and Tooling

| Technology | Role |
| --- | --- |
| Rust | Primary implementation language. |
| Rust edition 2024 | Language edition utilized across the workspace. |
| Cargo workspace | Multi-crate project organization and dependency resolution. |
| Stable Rust `std` | Baseline execution environment (`no_std` is not currently targeted). |
| rustfmt & Clippy | Linting, formatting, and static analysis enforcement. |
| rustdoc | API documentation generation. |
| GitHub Actions | CI pipeline handling formatting, testing, and documentation builds. |
| SVG & PNG | Output formats for the plotting engine. |

## Dependencies

### Internal Workspace Crates

- `math-core`
- `math-linalg`
- `math-graph`
- `math-numerics`
- `math-stats`
- `math-plot`
- `math-lib`

### Direct External Dependencies

| Crate | Version | Justification |
| --- | --- | --- |
| `num-traits` | `0.2` | Numeric trait support for generic scalar abstractions. |
| `rand` | `0.8` | Random sampling capabilities for probability distributions. |
| `image` | `0.25` | PNG output backend when the `png` feature flag is active. |

### Transitive Dependencies

The `Cargo.lock` resolves the following foundational and transitive dependencies to support the core functionality:

`adler2`, `autocfg`, `bitflags`, `bytemuck`, `byteorder-lite`, `cfg-if`, `crc32fast`, `fdeflate`, `flate2`, `getrandom`, `image`, `libc`, `miniz_oxide`, `moxcms`, `num-traits`, `png`, `ppv-lite86`, `proc-macro2`, `pxfm`, `quote`, `rand`, `rand_chacha`, `rand_core`, `simd-adler32`, `syn`, `unicode-ident`, `wasi`, `zerocopy`, `zerocopy-derive`.

## Quality Assurance

The library enforces correctness via:

- Isolated unit tests per crate.
- Cross-crate integration tests within `math-lib`.
- Doctests embedded in rustdoc comments.
- CI validation running:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-features`
  - `cargo doc --workspace --no-deps`

## Examples and Exploration

Detailed implementation examples are located in `crates/math-lib/examples/`, demonstrating:

- Matrix operations and system solving.
- Shortest path routing via Dijkstra's algorithm.
- Root finding and curve fitting.
- Function and dataset visualization.

## Future Scope

The existing architecture provides a highly modular foundation intended for future expansion into:

- Sparse matrix representations.
- Advanced decomposition (Eigenvalue, SVD).
- Continuous and discrete optimization methods.
- Advanced statistical modeling.
- Expanded graph clustering algorithms.
