use math_lib::{Color, Graph, Matrix, Plot, SolverOptions, Vector, bisection, linear_regression};

#[test]
fn solves_linear_system_and_checks_residual() {
    let matrix =
        Matrix::<f64>::new(3, 3, vec![4.0, 2.0, 1.0, 0.0, 5.0, 2.0, 1.0, 0.0, 3.0]).unwrap();
    let rhs = Vector::new(vec![4.0_f64, 3.0, 7.0]);
    let solution = matrix.solve(&rhs).unwrap();
    let residual = matrix.mul_vector(&solution).unwrap();

    for index in 0..rhs.len() {
        assert!((residual[index] - rhs[index]).abs() < 1e-8);
    }
}

#[test]
fn regression_matches_positive_trend() {
    let regression = linear_regression(&[1.0, 2.0, 3.0, 4.0], &[1.8, 4.1, 6.0, 8.2]).unwrap();
    assert!(regression.slope > 1.9 && regression.slope < 2.2);
    assert!(regression.r_squared > 0.99);
}

#[test]
fn dijkstra_finds_shortest_path() {
    let mut graph = Graph::<&str, ()>::new_directed();
    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");
    graph.add_edge(a, b, 1.0, ()).unwrap();
    graph.add_edge(a, c, 5.0, ()).unwrap();
    graph.add_edge(b, c, 2.0, ()).unwrap();
    graph.add_edge(c, d, 1.0, ()).unwrap();
    graph.add_edge(b, d, 5.0, ()).unwrap();

    let shortest = graph.dijkstra(a).unwrap();
    assert_eq!(shortest.path_to(d).unwrap(), vec![a, b, c, d]);
}

#[test]
fn root_finder_converges() {
    let result = bisection(|x| x * x - 2.0, 0.0, 2.0, SolverOptions::default()).unwrap();
    assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-6);
}

#[test]
fn svg_plot_output_contains_expected_markup() {
    let plot = Plot::new()
        .title("Deterministic Plot")
        .x_label("x")
        .y_label("y")
        .add_line_series("line", vec![(0.0, 0.0), (1.0, 1.0)], Color::BLUE)
        .add_scatter_series("points", vec![(0.25, 0.25), (0.75, 0.75)], Color::RED);

    let output = plot.render_svg().unwrap();
    assert!(output.svg.contains("<svg"));
    assert!(output.svg.contains("Deterministic Plot"));
    assert!(output.svg.contains("polyline"));
    assert!(output.svg.contains("circle"));
}
