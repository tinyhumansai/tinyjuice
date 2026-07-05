package com.thealgorithms.datastructures.graphs;

import java.util.Scanner;

class BellmanFord /*
                   * Implementation of Bellman ford to detect negative cycles. Graph accepts
                   * inputs
                   * in form of edges which have start vertex, end vertex and weights. Vertices
                   * should be labelled with a
                   * number between 0 and total number of vertices-1,both inclusive
                   */
{

    int vertex;
    int edge;
    private Edge[] edges;
    private int index = 0;

    BellmanFord(int v, int e) {
        vertex = v;
        edge = e;
        edges = new Edge[e];
    }

    class Edge {

        int u;
        int v;
        int w;

        /**
         * @param u Source Vertex
         * @param v End vertex
         * @param c Weight
         */
        Edge(int a, int b, int c) {
            u = a;
            v = b;
            w = c;
        }
    }

    /**
     * @param p[] Parent array which shows updates in edges
     * @param i   Current vertex under consideration
     */
    void printPath(int[] p, int i) {
        if (p[i] == -1) { // Found the path back to parent
            return;
        }
        printPath(p, p[i]);
        System.out.print(i + " ");
    }

    public static void main(String[] args) {
        BellmanFord obj = new BellmanFord(0, 0); // Dummy object to call nonstatic variables
        obj.go();
    }

    public void go() {
        // shows distance to all vertices
        // Interactive run for understanding the
        { … 58 line(s) … }
        }

    /**
     * @param source Starting vertex
     * @param end    Ending vertex
     * @param Edge   Array of edges
     */
    public void show(int source, int end,
        Edge[] arr) { // be created by using addEdge() method and passed by calling getEdgeArray()
                      // method // Just shows results of computation, if graph is passed to it. The
                      // graph should
                      { … 36 line(s) … }
        }

    /**
     * @param x Source Vertex
     * @param y End vertex
     * @param z Weight
     */
    public void addEdge(int x, int y, int z) { // Adds unidirectional edge
        edges[index++] = new Edge(x, y, z);
    }

    public Edge[] getEdgeArray() {
        return edges;
    }
}