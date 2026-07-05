// Segment Tree Data Structure for efficient range queries on an array of integers.
// It can query the sum and update the elements to a new value of any range of the array.
// Build: O(n*log(n))
// Query: O(log(n))
// Update: O(log(n))
// reference: https://cp-algorithms.com/data_structures/segment_tree.html
package segmenttree

import (
	"github.com/TheAlgorithms/Go/math/max"
	"github.com/TheAlgorithms/Go/math/min"
)

const emptyLazyNode = 0

// SegmentTree represents the data structure of a segment tree with lazy propagation
type SegmentTree struct {
    { … 4 line(s) … ⟦tj:aac4f0274a1949e2bfb8e08913f8f06d⟧ }

// Propagate propagates the lazy updates to the child nodes
func (s *SegmentTree) Propagate(node int, leftNode int, rightNode int) {
    { … 19 line(s) … ⟦tj:275c6a6ade52bfab833e0cef116197e2⟧ }

// Query returns the sum of elements of the array in the interval [firstIndex, leftIndex].
// node, leftNode and rightNode should always start with 1, 0 and len(Array)-1, respectively.
func (s *SegmentTree) Query(node int, leftNode int, rightNode int, firstIndex int, lastIndex int) int {
    { … 21 line(s) … ⟦tj:c4903fc762fa139e9c248aa15c751f78⟧ }

// Update updates the elements of the array in the range [firstIndex, lastIndex]
// with the new value provided and recomputes the sum of different ranges.
// node, leftNode and rightNode should always start with 1, 0 and len(Array)-1, respectively.
func (s *SegmentTree) Update(node int, leftNode int, rightNode int, firstIndex int, lastIndex int, value int) {
    { … 23 line(s) … ⟦tj:6ee939f1c046fd3425d7d5626ca90d0c⟧ }

// Build builds the SegmentTree by computing the sum of different ranges.
// node, leftNode and rightNode should always start with 1, 0 and len(Array)-1, respectively.
func (s *SegmentTree) Build(node int, left int, right int) {
    { … 13 line(s) … ⟦tj:f74716ead13f10781d7285f33c1f4964⟧ }

// NewSegmentTree returns a new instance of a SegmentTree. It takes an input
// array of integers representing Array, initializes and builds the SegmentTree.
func NewSegmentTree(Array []int) *SegmentTree {
    { … 20 line(s) … ⟦tj:bc0a487fe05ff9be2e0639b82c25be15⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (4496 bytes) is available by calling tinyjuice_retrieve with token "2b0546900ac8781d41cad05f73b52964" (marker ⟦tj:2b0546900ac8781d41cad05f73b52964⟧)]