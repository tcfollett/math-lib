# `math_lib`

`math_lib` is a modular Rust mathematics library and workspace for scientific computing, numerical analysis, graph algorithms, statistics, and plotting. The public package is published as `math-lib` and imported in Rust code as `math_lib`.

This library was created using Codex GPT-5.4 and then refined, tested, and organized as a multi-crate Rust workspace.

## Overview

The goal of `math_lib` is to provide a broad, coherent math toolkit instead of a single narrow subsystem. The current architecture focuses on:

- shared error handling and scalar traits
- dense linear algebra
- graph theory algorithms
- numerical methods
- descriptive and inferential statistics
- static plotting for mathematical and statistical output

The project is intentionally structured as a Cargo workspace so each mathematical domain can evolve independently while still feeling unified at the top-level API.

## Architecture

The repository is split into focused crates:

| Crate | Purpose |
| --- | --- |
| `math-core` | Shared `MathError`, validation helpers, tolerance utilities, and scalar trait definitions. |
| `math-linalg` | Dense vectors and matrices, matrix arithmetic, decomposition routines, inversion, rank, determinant, and system solving. |
| `math-graph` | Adjacency-list graph representation with traversal, shortest paths, topological sorting, connectivity analysis, and minimum spanning trees. |
| `math-numerics` | Root finding, integration, differentiation, interpolation, and ordinary differential equation solvers. |
| `math-stats` | Summary statistics, distributions, confidence intervals, hypothesis testing, and linear regression. |
| `math-plot` | Static SVG plotting with optional PNG output. |
| `math-lib` | Umbrella crate that re-exports the public API behind feature flags. |

The root workspace uses Cargo resolver 2 and targets stable Rust with `std`.

## Feature Walkthrough

### Shared core abstractions

`math-core` defines the vocabulary that the rest of the workspace uses. Instead of each crate inventing its own error style or numeric assumptions, they all build on:

- `MathError` for consistent failure reporting
- `MathResult<T>` for fallible operations
- `Scalar` and `RealScalar` trait layers for generic numeric code
- validation helpers like `ensure_non_empty`, `ensure_same_len`, and `ensure_square`
- tolerance helpers such as approximate and relative floating-point equality

This shared layer is especially important for scientific code because most operations are only meaningful under structural preconditions. For example, a matrix inverse only exists for square non-singular matrices, and correlation only makes sense when both samples have matching length and non-zero spread.

### Linear algebra

`math-linalg` is currently built around dense row-major storage with `Vector<T>` and `Matrix<T>`.

Implemented capabilities include:

- vector construction, indexing, iteration, mapping, and zip-style transforms
- dot products and norm calculations
- matrix construction and shape validation
- transpose, trace, matrix-vector multiplication, and matrix-matrix multiplication
- determinant and inverse for square real-valued matrices
- rank estimation by elimination
- linear system solving
- LU decomposition
- QR decomposition via Gram-Schmidt style orthogonalization

The library is designed around familiar linear algebra problems such as:

$$
A \mathbf{x} = \mathbf{b}
$$

and

$$
\mathbf{x} \cdot \mathbf{y} = \sum_{i=1}^{n} x_i y_i
$$

This is the foundation for future work such as sparse matrices, eigenvalue routines, optimization, and tensor-oriented computation.

### Graph theory

`math-graph` treats graph theory as a first-class subsystem, separate from plotting. It currently provides:

- directed and undirected graph constructors
- stable `NodeId` and `EdgeId` handles
- weighted adjacency-list storage
- breadth-first search
- depth-first search
- connected components
- topological sort for DAG-style workflows
- Dijkstra shortest paths
- minimum spanning trees for weighted undirected graphs

This makes the crate useful for routing, dependency analysis, workflow scheduling, and algorithm education in addition to more traditional mathematical graph applications.

### Numerical methods

`math-numerics` covers a range of scientific computing tasks:

- root finding with bisection, Newton-Raphson, and secant methods
- numerical integration with trapezoidal, Simpson, and adaptive Simpson rules
- finite-difference first and second derivatives
- finite-difference gradients for multivariate scalar functions
- linear interpolation
- cubic spline interpolation
- Euler and RK4 ODE solvers

The Newton-Raphson solver follows the classic iteration:

$$
x_{n+1} = x_n - \frac{f(x_n)}{f'(x_n)}
$$

The Simpson rule implementation targets approximations of the form:

$$
\int_a^b f(x)\,dx \approx \frac{h}{3}
\left[
f(x_0)
+ 4 \sum_{k=1}^{n/2} f(x_{2k-1})
+ 2 \sum_{k=1}^{n/2-1} f(x_{2k})
+ f(x_n)
\right]
$$

The crate returns structured convergence information where appropriate and uses shared `MathError` variants for invalid domains, invalid ranges, and non-convergence.

### Statistical methods

`math-stats` is designed to support both exploratory analysis and classical statistical workflows.

Current descriptive functionality includes:

- mean
- median
- mode
- quantiles
- population and sample variance
- population and sample standard deviation
- covariance
- correlation
- aggregate summary statistics

The statistical core includes formulas aligned with standard notation:

$$
\mu = \frac{1}{n}\sum_{i=1}^{n} x_i
\qquad
s^2 = \frac{1}{n-1}\sum_{i=1}^{n}(x_i - \bar{x})^2
$$

Distribution support currently includes:

- normal distribution
- binomial distribution
- Poisson distribution
- PDF/PMF evaluation
- CDF evaluation
- inverse CDF where practical
- random sampling

Inferential statistics currently include:

- mean confidence intervals
- z-tests
- one-sample t-tests
- chi-square goodness-of-fit testing
- simple linear regression

The linear regression implementation follows the standard least-squares slope:

$$
\hat{y} = \beta_0 + \beta_1 x
\qquad
\beta_1 =
\frac{\sum_{i=1}^{n}(x_i-\bar{x})(y_i-\bar{y})}
{\sum_{i=1}^{n}(x_i-\bar{x})^2}
$$

Supporting special functions are implemented locally where needed, including approximations related to the error function, gamma function, beta function, normal quantiles, Student's t distribution, and chi-square probabilities.

### Plotting

`math-plot` provides static plotting aimed at scientific and statistical use cases. It currently supports:

- line series
- scatter series
- histograms
- sampled function plots
- configurable axis labels
- configurable titles
- configurable plot sizes
- SVG rendering by default
- optional PNG output through a feature flag

The plotting API is deliberately backend-light at the surface. Consumers work with `Plot`, `Color`, and `PlotOutput` rather than low-level rendering primitives.

This makes it practical to:

- visualize sampled numerical functions
- inspect datasets or regression points
- render histograms for exploratory analysis
- generate artifact files that can be viewed outside Rust

### Umbrella crate and feature flags

`math-lib` re-exports the public API so users can start with a single dependency while keeping subsystem boundaries clean.

Available feature flags:

| Feature | Meaning |
| --- | --- |
| `linalg` | Enables dense linear algebra APIs. |
| `graph` | Enables graph theory data structures and algorithms. |
| `numerics` | Enables numerical methods. |
| `stats` | Enables the statistics subsystem. |
| `plot` | Enables static plotting. |
| `png` | Enables PNG output through the plotting crate. |

Default features enable `linalg`, `graph`, `numerics`, `stats`, and `plot`.

## Public API shape

The primary user-facing types currently include:

- `MathError`
- `Vector<T>`
- `Matrix<T>`
- `Graph<N, E>`
- `NodeId`
- `EdgeId`
- `SolverOptions`
- `RootResult`
- `SummaryStatistics`
- `ConfidenceInterval`
- `HypothesisTestResult`
- `RegressionResult`
- `Plot`
- `PlotOutput`
- `Color`

The design philosophy is:

- use generic numeric traits where reasonable
- use concrete `f64`-oriented routines where scientific methods fundamentally require real-valued floating-point arithmetic
- keep fallible operations explicit through `Result`
- preserve a broad surface area while keeping the API readable

## Technologies and tooling

The project currently uses the following core technologies:

| Technology | Role |
| --- | --- |
| Rust | Primary implementation language. |
| Rust edition 2024 | Language edition used across the workspace. |
| Cargo workspace | Multi-crate project organization and dependency management. |
| Cargo resolver 2 | Workspace dependency resolution behavior. |
| Stable Rust `std` | Baseline execution model; `no_std` is not currently targeted. |
| rustfmt | Formatting standard used in CI and local development. |
| Clippy | Lint enforcement for code quality. |
| rustdoc / `cargo doc` | API documentation generation. |
| GitHub Actions | Continuous integration for formatting, linting, tests, and docs. |
| SVG | Default plot serialization format. |
| PNG | Optional raster output path via a feature flag. |

## Dependencies

### First-party workspace crates

These are the internal dependencies that make up the library itself:

- `math-core`
- `math-linalg`
- `math-graph`
- `math-numerics`
- `math-stats`
- `math-plot`
- `math-lib`

### Direct third-party dependencies

These are the explicit external crates referenced by the workspace manifests:

| Crate | Version | Why it is used |
| --- | --- | --- |
| `num-traits` | `0.2` | Numeric trait support for generic scalar abstractions and real-valued operations. |
| `rand` | `0.8` | Random sampling for probability distributions. |
| `image` | `0.25` | Optional PNG output backend for plots when the `png` feature is enabled. |

### Current transitive third-party dependency set

The current `Cargo.lock` includes the following non-workspace third-party crates:

- `adler2`
- `autocfg`
- `bitflags`
- `bytemuck`
- `byteorder-lite`
- `cfg-if`
- `crc32fast`
- `fdeflate`
- `flate2`
- `getrandom`
- `image`
- `libc`
- `miniz_oxide`
- `moxcms`
- `num-traits`
- `png`
- `ppv-lite86`
- `proc-macro2`
- `pxfm`
- `quote`
- `rand`
- `rand_chacha`
- `rand_core`
- `simd-adler32`
- `syn`
- `unicode-ident`
- `wasi`
- `zerocopy`
- `zerocopy-derive`

In practice, these fall into a few functional groups:

- numeric traits and compile-time numeric support
- randomness and OS entropy access
- optional PNG encoding and image writing
- supporting macro and derive infrastructure required by some transitive crates

## Quality and verification

The workspace is set up to validate the library through:

- unit tests in each crate
- cross-crate integration tests in the umbrella crate
- doctest coverage through the generated crate docs
- formatting checks
- lint checks
- docs builds in CI

The main CI workflow currently runs:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo doc --workspace --no-deps`

## Examples and exploration

Instead of a short quick-start snippet, the repository includes dedicated examples under `crates/math-lib/examples` for:

- matrix operations
- solving linear systems
- Dijkstra shortest paths
- root finding
- regression
- function plotting
- histogram plotting

These examples are useful both as usage references and as smoke tests for the high-level API.

## Current scope and future direction

The current version is intentionally broad rather than deeply specialized. It already covers a meaningful portion of a scientific computing workflow, but it is also a foundation for future work such as:

- sparse linear algebra
- eigenvalue and SVD routines
- optimization methods
- more probability distributions
- richer statistical modeling
- advanced graph algorithms
- more plotting styles and export options

The existing implementation favors clarity, modularity, and testability so those future additions can be layered on without rewriting the public structure of the workspace.
