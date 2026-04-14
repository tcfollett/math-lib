#![forbid(unsafe_code)]
#![doc = "Graph data structures and classical graph algorithms."]

use math_core::{MathError, MathResult};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeId(pub usize);

#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    directed: bool,
    nodes: Vec<N>,
    edges: Vec<Edge<E>>,
    adjacency: Vec<Vec<AdjacencyEdge>>,
}

#[derive(Debug, Clone)]
struct Edge<E> {
    id: EdgeId,
    source: NodeId,
    target: NodeId,
    weight: f64,
    data: E,
}

#[derive(Debug, Clone, Copy)]
struct AdjacencyEdge {
    edge_id: EdgeId,
    neighbor: NodeId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShortestPaths {
    pub source: NodeId,
    pub distances: Vec<Option<f64>>,
    pub previous: Vec<Option<NodeId>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MinimumSpanningTree {
    pub total_weight: f64,
    pub edges: Vec<EdgeId>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct QueueState {
    cost: f64,
    node: NodeId,
}

impl Eq for QueueState {}

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N, E> Graph<N, E> {
    pub fn new_directed() -> Self {
        Self {
            directed: true,
            nodes: Vec::new(),
            edges: Vec::new(),
            adjacency: Vec::new(),
        }
    }

    pub fn new_undirected() -> Self {
        Self {
            directed: false,
            nodes: Vec::new(),
            edges: Vec::new(),
            adjacency: Vec::new(),
        }
    }

    pub fn is_directed(&self) -> bool {
        self.directed
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn add_node(&mut self, value: N) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(value);
        self.adjacency.push(Vec::new());
        id
    }

    pub fn add_edge(
        &mut self,
        source: NodeId,
        target: NodeId,
        weight: f64,
        data: E,
    ) -> MathResult<EdgeId> {
        self.ensure_node(source)?;
        self.ensure_node(target)?;

        if weight.is_sign_negative() {
            return Err(MathError::InvalidGraph {
                message: "negative edge weights are not supported".to_string(),
            });
        }

        let id = EdgeId(self.edges.len());
        self.edges.push(Edge {
            id,
            source,
            target,
            weight,
            data,
        });

        self.adjacency[source.0].push(AdjacencyEdge {
            edge_id: id,
            neighbor: target,
        });
        if !self.directed {
            self.adjacency[target.0].push(AdjacencyEdge {
                edge_id: id,
                neighbor: source,
            });
        }

        Ok(id)
    }

    pub fn node(&self, id: NodeId) -> MathResult<&N> {
        self.nodes.get(id.0).ok_or_else(|| MathError::InvalidGraph {
            message: format!("node {:?} does not exist", id),
        })
    }

    pub fn edge(&self, id: EdgeId) -> MathResult<(&N, &N, f64, &E)> {
        let edge = self
            .edges
            .get(id.0)
            .ok_or_else(|| MathError::InvalidGraph {
                message: format!("edge {:?} does not exist", id),
            })?;

        Ok((
            self.node(edge.source)?,
            self.node(edge.target)?,
            edge.weight,
            &edge.data,
        ))
    }

    pub fn bfs(&self, start: NodeId) -> MathResult<Vec<NodeId>> {
        self.ensure_node(start)?;
        let mut visited = vec![false; self.node_count()];
        let mut queue = VecDeque::from([start]);
        let mut order = Vec::new();

        visited[start.0] = true;
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for neighbor in self.outgoing_neighbors(node) {
                if !visited[neighbor.0] {
                    visited[neighbor.0] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        Ok(order)
    }

    pub fn dfs(&self, start: NodeId) -> MathResult<Vec<NodeId>> {
        self.ensure_node(start)?;
        let mut visited = vec![false; self.node_count()];
        let mut stack = vec![start];
        let mut order = Vec::new();

        while let Some(node) = stack.pop() {
            if visited[node.0] {
                continue;
            }
            visited[node.0] = true;
            order.push(node);

            let mut neighbors = self.outgoing_neighbors(node);
            neighbors.reverse();
            for neighbor in neighbors {
                if !visited[neighbor.0] {
                    stack.push(neighbor);
                }
            }
        }

        Ok(order)
    }

    pub fn connected_components(&self) -> Vec<Vec<NodeId>> {
        let mut visited = vec![false; self.node_count()];
        let mut components = Vec::new();

        for start in 0..self.node_count() {
            if visited[start] {
                continue;
            }

            let mut queue = VecDeque::from([NodeId(start)]);
            let mut component = Vec::new();
            visited[start] = true;

            while let Some(node) = queue.pop_front() {
                component.push(node);
                for neighbor in self.weak_neighbors(node) {
                    if !visited[neighbor.0] {
                        visited[neighbor.0] = true;
                        queue.push_back(neighbor);
                    }
                }
            }

            components.push(component);
        }

        components
    }

    pub fn topological_sort(&self) -> MathResult<Vec<NodeId>> {
        if !self.directed {
            return Err(MathError::InvalidGraph {
                message: "topological sort requires a directed graph".to_string(),
            });
        }

        let mut indegree = vec![0usize; self.node_count()];
        for edge in &self.edges {
            indegree[edge.target.0] += 1;
        }

        let mut queue = VecDeque::new();
        for (index, degree) in indegree.iter().enumerate() {
            if *degree == 0 {
                queue.push_back(NodeId(index));
            }
        }

        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for neighbor in self.outgoing_neighbors(node) {
                indegree[neighbor.0] -= 1;
                if indegree[neighbor.0] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if order.len() != self.node_count() {
            return Err(MathError::InvalidGraph {
                message: "graph contains a cycle".to_string(),
            });
        }

        Ok(order)
    }

    pub fn dijkstra(&self, source: NodeId) -> MathResult<ShortestPaths> {
        self.ensure_node(source)?;
        let mut distances = vec![None; self.node_count()];
        let mut previous = vec![None; self.node_count()];
        let mut heap = BinaryHeap::new();

        distances[source.0] = Some(0.0);
        heap.push(QueueState {
            cost: 0.0,
            node: source,
        });

        while let Some(QueueState { cost, node }) = heap.pop() {
            if let Some(best) = distances[node.0] {
                if cost > best {
                    continue;
                }
            }

            for adjacency in &self.adjacency[node.0] {
                let edge = &self.edges[adjacency.edge_id.0];
                let next_cost = cost + edge.weight;
                let entry = &mut distances[adjacency.neighbor.0];

                if entry.is_none_or(|current| next_cost < current) {
                    *entry = Some(next_cost);
                    previous[adjacency.neighbor.0] = Some(node);
                    heap.push(QueueState {
                        cost: next_cost,
                        node: adjacency.neighbor,
                    });
                }
            }
        }

        Ok(ShortestPaths {
            source,
            distances,
            previous,
        })
    }

    pub fn minimum_spanning_tree(&self) -> MathResult<MinimumSpanningTree> {
        if self.directed {
            return Err(MathError::InvalidGraph {
                message: "minimum spanning tree requires an undirected graph".to_string(),
            });
        }

        let mut edge_ids: Vec<_> = self.edges.iter().map(|edge| edge.id).collect();
        edge_ids.sort_by(|lhs, rhs| {
            self.edges[lhs.0]
                .weight
                .partial_cmp(&self.edges[rhs.0].weight)
                .unwrap_or(Ordering::Equal)
        });

        let mut union_find = UnionFind::new(self.node_count());
        let mut selected = Vec::new();
        let mut total_weight = 0.0;

        for edge_id in edge_ids {
            let edge = &self.edges[edge_id.0];
            if union_find.union(edge.source.0, edge.target.0) {
                selected.push(edge_id);
                total_weight += edge.weight;
            }
        }

        Ok(MinimumSpanningTree {
            total_weight,
            edges: selected,
        })
    }

    fn outgoing_neighbors(&self, node: NodeId) -> Vec<NodeId> {
        self.adjacency[node.0]
            .iter()
            .map(|adjacency| adjacency.neighbor)
            .collect()
    }

    fn weak_neighbors(&self, node: NodeId) -> Vec<NodeId> {
        let mut neighbors = self.outgoing_neighbors(node);
        if self.directed {
            for edge in &self.edges {
                if edge.target == node {
                    neighbors.push(edge.source);
                }
            }
        }
        neighbors.sort();
        neighbors.dedup();
        neighbors
    }

    fn ensure_node(&self, node: NodeId) -> MathResult<()> {
        if node.0 >= self.node_count() {
            return Err(MathError::InvalidGraph {
                message: format!("node {:?} does not exist", node),
            });
        }

        Ok(())
    }
}

impl ShortestPaths {
    pub fn distance_to(&self, node: NodeId) -> Option<f64> {
        self.distances.get(node.0).and_then(|distance| *distance)
    }

    pub fn path_to(&self, node: NodeId) -> Option<Vec<NodeId>> {
        let _ = self.distances.get(node.0)?.as_ref()?;
        let mut path = Vec::new();
        let mut current = Some(node);

        while let Some(node_id) = current {
            path.push(node_id);
            if node_id == self.source {
                break;
            }
            current = self.previous[node_id.0];
        }

        path.reverse();
        Some(path)
    }
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            let parent = self.parent[value];
            self.parent[value] = self.find(parent);
        }
        self.parent[value]
    }

    fn union(&mut self, left: usize, right: usize) -> bool {
        let left_root = self.find(left);
        let right_root = self.find(right);
        if left_root == right_root {
            return false;
        }

        match self.rank[left_root].cmp(&self.rank[right_root]) {
            Ordering::Less => self.parent[left_root] = right_root,
            Ordering::Greater => self.parent[right_root] = left_root,
            Ordering::Equal => {
                self.parent[right_root] = left_root;
                self.rank[left_root] += 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, NodeId};

    #[test]
    fn graph_traversals_and_components_work() {
        let mut graph = Graph::<&str, ()>::new_undirected();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");

        graph.add_edge(a, b, 1.0, ()).unwrap();
        graph.add_edge(b, c, 2.0, ()).unwrap();

        assert_eq!(graph.bfs(a).unwrap(), vec![a, b, c]);
        assert_eq!(graph.dfs(a).unwrap(), vec![a, b, c]);

        let components = graph.connected_components();
        assert_eq!(components.len(), 2);
        assert!(
            components
                .iter()
                .any(|component| component == &vec![NodeId(3)])
        );

        graph.add_edge(c, d, 3.0, ()).unwrap();
        let mst = graph.minimum_spanning_tree().unwrap();
        assert_eq!(mst.edges.len(), 3);
        assert!((mst.total_weight - 6.0).abs() < 1e-10);
    }

    #[test]
    fn dijkstra_and_topological_sort_work() {
        let mut graph = Graph::<&str, ()>::new_directed();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");

        graph.add_edge(a, b, 1.0, ()).unwrap();
        graph.add_edge(a, c, 4.0, ()).unwrap();
        graph.add_edge(b, c, 2.0, ()).unwrap();
        graph.add_edge(b, d, 6.0, ()).unwrap();
        graph.add_edge(c, d, 1.0, ()).unwrap();

        let paths = graph.dijkstra(a).unwrap();
        assert!((paths.distance_to(d).unwrap() - 4.0).abs() < 1e-10);
        assert_eq!(paths.path_to(d).unwrap(), vec![a, b, c, d]);

        let order = graph.topological_sort().unwrap();
        assert_eq!(order.first().copied(), Some(a));
        assert_eq!(order.last().copied(), Some(d));
    }
}
