using DataStructures.Graph;

namespace Algorithms.Graph;

/// <summary>
///     Topological Sort is a linear ordering of vertices in a Directed Acyclic Graph (DAG)
///     such that for every directed edge (u, v), vertex u comes before vertex v in the ordering.
///
///     KEY CONCEPTS:
///     1. Only applicable to Directed Acyclic Graphs (DAGs) - graphs with no cycles.
///     2. A DAG can have multiple valid topological orderings.
///     3. Used in dependency resolution, task scheduling, build systems, and course prerequisites.
///
///     ALGORITHM APPROACHES:
///     1. DFS-based (Depth-First Search): Uses post-order traversal and reverses the result.
///     2. Kahn's Algorithm: Uses in-degree counting and processes vertices with zero in-degree.
///
///     TIME COMPLEXITY: O(V + E) where V is vertices and E is edges.
///     SPACE COMPLEXITY: O(V) for the visited set and result stack.
///
///     Reference: "Introduction to Algorithms" (CLRS) by Cormen, Leiserson, Rivest, and Stein.
///     Also covered in "Algorithm Design Manual" by Steven Skiena.
/// </summary>
/// <typeparam name="T">Vertex data type.</typeparam>
public class TopologicalSort<T> where T : IComparable<T>
{
    { … 337 line(s) … ⟦tj:c25d9fa7c8a6c92870e3030581976deb⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (14419 bytes) is available by calling tinyjuice_retrieve with token "5dc7491667a2d80f9805a7947293b5f7" (marker ⟦tj:5dc7491667a2d80f9805a7947293b5f7⟧)]