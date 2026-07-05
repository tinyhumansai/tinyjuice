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
) -> BTreeMap<V, BTreeMap<V, E>> { … 48 line(s) … ⟦tj:34932fe2c456b844392edaa7eecd2dbe⟧ }

#[cfg(test)]
mod tests {
    use super::{floyd_warshall, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
    }

    fn bi_add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) { … 4 line(s) … ⟦tj:628c3deb8939af59eedcbfef0a5284b5⟧ }

    #[test]
    fn single_vertex() { … 9 line(s) … ⟦tj:8a33699bc7e5f62866d92e62eb15cac1⟧ }

    #[test]
    fn single_edge() { … 21 line(s) … ⟦tj:ae02f70cb2d0137c119c9ee1a28913aa⟧ }

    #[test]
    fn graph_1() { … 74 line(s) … ⟦tj:3cc40fdee7886c6dab3ac06cf858fda2⟧ }
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (6322 bytes) is available by calling tinyjuice_retrieve with token "2a315547b7b6c55162e1fe836d7977c3" (marker ⟦tj:2a315547b7b6c55162e1fe836d7977c3⟧)]