use math_lib::Graph;

fn main() -> Result<(), math_lib::MathError> {
    let mut graph = Graph::<&str, ()>::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b, 1.0, ())?;
    graph.add_edge(a, c, 4.0, ())?;
    graph.add_edge(b, c, 2.0, ())?;
    graph.add_edge(c, d, 1.0, ())?;

    let shortest = graph.dijkstra(a)?;
    println!("distance(A -> D) = {:?}", shortest.distance_to(d));
    println!("path(A -> D) = {:?}", shortest.path_to(d));
    Ok(())
}
