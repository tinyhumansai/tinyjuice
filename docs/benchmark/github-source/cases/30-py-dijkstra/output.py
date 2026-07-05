# Title: Dijkstra's Algorithm for finding single source shortest path from scratch
# Author: Shubham Malik
# References: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

import math
import sys

# For storing the vertex set to retrieve node with the lowest distance


class PriorityQueue:
    # Based on Min Heap
    def __init__(self):
        """
        Priority queue class constructor method.

        Examples:
        >>> priority_queue_test = PriorityQueue()
        >>> priority_queue_test.cur_size
        0
        >>> priority_queue_test.array
        []
        >>> priority_queue_test.pos
        {}
        """
        self.cur_size = 0
        self.array = []
        self.pos = {}  # To store the pos of node in array

    def is_empty(self):
        """
        Conditional boolean method to determine if the priority queue is empty or not.
...  # 9 line(s) collapsed ⟦tj:e9a3c55f96e272abfbd740b3ae869ff9⟧
        return self.cur_size == 0

    def min_heapify(self, idx):
        """
        Sorts the queue array so that the minimum element is root.
...  # 38 line(s) collapsed ⟦tj:e68ba07f1cf37bf4fc4fb12980512969⟧
            self.min_heapify(smallest)

    def insert(self, tup):
        """
        Inserts a node into the Priority Queue.
...  # 16 line(s) collapsed ⟦tj:aa9f5de3e7060348d33e72a5f676422d⟧
        self.decrease_key((sys.maxsize, tup[1]), tup[0])

    def extract_min(self):
        """
        Removes and returns the min element at top of priority queue.
...  # 17 line(s) collapsed ⟦tj:ed24fe442d98453d401abe5dbf9ca749⟧
        return min_node

    def left(self, i):
        ...  # 11 line(s) collapsed ⟦tj:2bf034471ee2687f6b6e2707afa7548a⟧

    def right(self, i):
        ...  # 11 line(s) collapsed ⟦tj:fa338c4bf843f141050caae2164ba094⟧

    def par(self, i):
        """
        Returns the index of parent
...  # 10 line(s) collapsed ⟦tj:94ca3c6fba4d67ce39435985eeeb378f⟧
        return math.floor(i / 2)

    def swap(self, i, j):
        """
        Swaps array elements at indices i and j, update the pos{}
...  # 16 line(s) collapsed ⟦tj:765ebf08d947f25d711be0c64d62c5a7⟧
        self.array[j] = temp

    def decrease_key(self, tup, new_d):
        """
        Decrease the key value for a given tuple, assuming the new_d is at most old_d.
...  # 15 line(s) collapsed ⟦tj:8730a79118e1c3f399c9ccaac3270c55⟧
            idx = self.par(idx)


class Graph:
    def __init__(self, num):
        """
        Graph class constructor

        Examples:
        >>> graph_test = Graph(1)
        >>> graph_test.num_nodes
        1
        >>> graph_test.dist
        [0]
        >>> graph_test.par
        [-1]
        >>> graph_test.adjList
        {}
        """
        self.adjList = {}  # To store graph: u -> (v,w)
        self.num_nodes = num  # Number of nodes in graph
        # To store the distance from source vertex
        self.dist = [0] * self.num_nodes
        self.par = [-1] * self.num_nodes  # To store the path

    def add_edge(self, u, v, w):
        """
        Add edge going from node u to v and v to u with weight w: u (w)-> v, v (w) -> u
...  # 18 line(s) collapsed ⟦tj:8eca862055cf8bdc980d37815306ea47⟧
            self.adjList[v] = [(u, w)]

    def show_graph(self):
        """
        Show the graph: u -> v(w)
...  # 14 line(s) collapsed ⟦tj:5d165ae537bcf53fbe6e6e9d55008d61⟧
            print(u, "->", " -> ".join(str(f"{v}({w})") for v, w in self.adjList[u]))

    def dijkstra(self, src):
        """
        Dijkstra algorithm
...  # 98 line(s) collapsed ⟦tj:548dc4e1d8688d2341645e1dcdc2a9f7⟧
        self.show_distances(src)

    def show_distances(self, src):
        """
        Show the distances from src to all other nodes in a graph
...  # 9 line(s) collapsed ⟦tj:f15126b8518182330dc1862cfffb2582⟧
            print(f"Node {u} has distance: {self.dist[u]}")

    def show_path(self, src, dest):
        """
        Shows the shortest path from src to dest.
...  # 39 line(s) collapsed ⟦tj:3101fb4a6bf0fb7464d8028248d8a35e⟧
        print("\nTotal cost of path: ", cost)


if __name__ == "__main__":
    from doctest import testmod

    testmod()
    graph = Graph(9)
    graph.add_edge(0, 1, 4)
    graph.add_edge(0, 7, 8)
    graph.add_edge(1, 2, 8)
    graph.add_edge(1, 7, 11)
    graph.add_edge(2, 3, 7)
    graph.add_edge(2, 8, 2)
    graph.add_edge(2, 5, 4)
    graph.add_edge(3, 4, 9)
    graph.add_edge(3, 5, 14)
    graph.add_edge(4, 5, 10)
    graph.add_edge(5, 6, 2)
    graph.add_edge(6, 7, 1)
    graph.add_edge(6, 8, 6)
    graph.add_edge(7, 8, 7)
    graph.show_graph()
    graph.dijkstra(0)
    graph.show_path(0, 4)

# OUTPUT
# 0 -> 1(4) -> 7(8)
# 1 -> 0(4) -> 2(8) -> 7(11)
# 7 -> 0(8) -> 1(11) -> 6(1) -> 8(7)
# 2 -> 1(8) -> 3(7) -> 8(2) -> 5(4)
# 3 -> 2(7) -> 4(9) -> 5(14)
# 8 -> 2(2) -> 6(6) -> 7(7)
# 5 -> 2(4) -> 3(14) -> 4(10) -> 6(2)
# 4 -> 3(9) -> 5(10)
# 6 -> 5(2) -> 7(1) -> 8(6)
# Distance from node: 0
# Node 0 has distance: 0
# Node 1 has distance: 4
# Node 2 has distance: 12
# Node 3 has distance: 19
# Node 4 has distance: 21
# Node 5 has distance: 11
# Node 6 has distance: 9
# Node 7 has distance: 8
# Node 8 has distance: 14
# ----Path to reach 4 from 0----
# 0 -> 7 -> 6 -> 5 -> 4
# Total cost of path:  21
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (14647 bytes): call tinyjuice_retrieve with token "96e231f5562b5d93c73ae777b452257e"]