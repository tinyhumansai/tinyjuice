class DisjointSetTreeNode {
  // Disjoint Set Node to store the parent and rank
  constructor(key) { … 5 line(s) … ⟦tj:cc203209ef83cf88d95f82bbc1ff2e63⟧ }
}

class DisjointSetTree {
  // Disjoint Set DataStructure
  constructor() { … 4 line(s) … ⟦tj:423f66fac68fd7c61caff56fa67e3e01⟧ }

  makeSet(x) { … 4 line(s) … ⟦tj:3be76121c4e24a58afbeca0ec04cd9c0⟧ }

  findSet(x) { … 7 line(s) … ⟦tj:100847979048df823ae6a5423d44d937⟧ }

  union(x, y) { … 4 line(s) … ⟦tj:d3f1c74c4d1b4efa72fe8d37129a1d34⟧ }

  link(x, y) { … 11 line(s) … ⟦tj:9f99af8cff2f922138fd7f3a02ff748b⟧ }
}

class GraphWeightedUndirectedAdjacencyList {
  // Weighted Undirected Graph class
  constructor() { … 4 line(s) … ⟦tj:e126c8ad49d316abc603fbd0df82d5ff⟧ }

  addNode(node) { … 5 line(s) … ⟦tj:a4388b9b38823db199f56247533ec4bd⟧ }

  addEdge(node1, node2, weight) { … 11 line(s) … ⟦tj:8cf29be138c5b1ef6e39d1dd4e890e1c⟧ }

  KruskalMST() { … 33 line(s) … ⟦tj:073a50b8b3172c5f099f335e9b01768a⟧ }
}

export { GraphWeightedUndirectedAdjacencyList }

// const graph = new GraphWeightedUndirectedAdjacencyList()
// graph.addEdge(1, 2, 1)
// graph.addEdge(2, 3, 2)
// graph.addEdge(3, 4, 1)
// graph.addEdge(3, 5, 100) // Removed in MST
// graph.addEdge(4, 5, 5)
// graph.KruskalMST()
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (3086 bytes) is available by calling tinyjuice_retrieve with token "f9a94aceb7b774ae7e215ec33370d62c" (marker ⟦tj:f9a94aceb7b774ae7e215ec33370d62c⟧)]