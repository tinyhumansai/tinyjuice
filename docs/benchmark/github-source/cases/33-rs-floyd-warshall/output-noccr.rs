use num_traits::Zero;
use std::collections::BTreeMap;
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

/// Performs the Floyd-Warshall algorithm on the input graph.\
/// The graph is a weighted, directed graph with no negative cycles.
///
/// Returns a map storing the distance from each node to all the others.\
/// i.e. For each vertex `u`, `map[u][v] == Some(distance)` means
/// distance is the sum of the weights of the edges on the shortest path
/// from `u` to `v`.
///
/// For a key `v`, if `map[v].len() == 0`, then `v` cannot reach any other vertex, but is in the graph
/// (island node, or sink in the case of a directed graph)
pub fn floyd_warshall<V: Ord + Copy, E: Ord + Copy + Add<Output = E> + num_traits::Zero>(
    graph: &Graph<V, E>,
) -> BTreeMap<V, BTreeMap<V, E>> {
    let mut map: BTreeMap<V, BTreeMap<V, E>> = BTreeMap::new();
    for (u, edges) in graph.iter() {
    { … 43 line(s) … }
    map
}

#[cfg(test)]
mod tests {
    use super::{floyd_warshall, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
    }

    fn bi_add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        add_edge(graph, v1, v2, c);
        add_edge(graph, v2, v1, c);
    }

    #[test]
    fn single_vertex() {
        let mut graph: Graph<usize, usize> = BTreeMap::new();
        graph.insert(0, BTreeMap::new());

        let mut dists = BTreeMap::new();
        dists.insert(0, BTreeMap::new());
        dists.get_mut(&0).unwrap().insert(0, 0);
        assert_eq!(floyd_warshall(&graph), dists);
    }

    #[test]
    fn single_edge() {
        let mut graph = BTreeMap::new();
        bi_add_edge(&mut graph, 0, 1, 2);
        { … 16 line(s) … }
        assert_eq!(floyd_warshall(&graph), dists_0);
}

    #[test]
    fn graph_1() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 'a', 'c', 12);
        { … 69 line(s) … }
        assert_eq!(floyd_warshall(&graph), dists_a);
}
}