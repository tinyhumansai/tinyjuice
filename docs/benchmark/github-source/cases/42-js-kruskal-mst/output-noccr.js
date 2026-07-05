class DisjointSetTreeNode {
  // Disjoint Set Node to store the parent and rank
  constructor(key) {
    this.key = key
    this.parent = this
    this.rank = 0
  }
}

class DisjointSetTree {
  // Disjoint Set DataStructure
  constructor() {
    // map to from node name to the node object
    this.map = {}
  }

  makeSet(x) {
    // Function to create a new set with x as its member
    this.map[x] = new DisjointSetTreeNode(x)
  }

  findSet(x) {
    // Function to find the set x belongs to (with path-compression)
    if (this.map[x] !== this.map[x].parent) {
      this.map[x].parent = this.findSet(this.map[x].parent.key)
    }
    return this.map[x].parent
  }

  union(x, y) {
    // Function to merge 2 disjoint sets
    this.link(this.findSet(x), this.findSet(y))
  }

  link(x, y) { … 11 line(s) … }
}

class GraphWeightedUndirectedAdjacencyList {
  // Weighted Undirected Graph class
  constructor() {
    this.connections = {}
    this.nodes = 0
  }

  addNode(node) {
    // Function to add a node to the graph (connection represented by set)
    this.connections[node] = {}
    this.nodes += 1
  }

  addEdge(node1, node2, weight) { … 11 line(s) … }

  KruskalMST() {
    // Kruskal's Algorithm to generate a Minimum Spanning Tree (MST) of a graph
    // Details: https://en.wikipedia.org/wiki/Kruskal%27s_algorithm
    { … 28 line(s) … }
    return graph
}
}

export { GraphWeightedUndirectedAdjacencyList }

// const graph = new GraphWeightedUndirectedAdjacencyList()
// graph.addEdge(1, 2, 1)
// graph.addEdge(2, 3, 2)
// graph.addEdge(3, 4, 1)
// graph.addEdge(3, 5, 100) // Removed in MST
// graph.addEdge(4, 5, 5)
// graph.KruskalMST()