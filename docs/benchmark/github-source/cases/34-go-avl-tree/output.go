// AVL tree is a self-balancing binary search tree.
//
// For more details check out those link below here:
// Wikipedia article: https://en.wikipedia.org/wiki/AVL_tree
// see avl.go

package tree

import (
	"github.com/TheAlgorithms/Go/constraints"
	"github.com/TheAlgorithms/Go/math/max"
)

// Verify Interface Compliance
var _ Node[int] = &AVLNode[int]{}

// AVLNode represents a single node in the AVL.
type AVLNode[T constraints.Ordered] struct {
    { … 6 line(s) … ⟦tj:46db24fe48aaa7d04561f3862fba07e3⟧ }

func (n *AVLNode[T]) Key() T {
	return n.key
}

func (n *AVLNode[T]) Parent() Node[T] {
	return n.parent
}

func (n *AVLNode[T]) Left() Node[T] {
	return n.left
}

func (n *AVLNode[T]) Right() Node[T] {
	return n.right
}

func (n *AVLNode[T]) Height() int {
	return n.height
}

// AVL represents a AVL tree.
// By default, _NIL = nil.
type AVL[T constraints.Ordered] struct {
	Root *AVLNode[T]
	_NIL *AVLNode[T] // a sentinel value for nil
}

// NewAVL creates a novel AVL tree
func NewAVL[T constraints.Ordered]() *AVL[T] {
    { … 5 line(s) … ⟦tj:b076771d6cad8fb67078f377271d08ce⟧ }

// Empty determines the AVL tree is empty
func (avl *AVL[T]) Empty() bool {
	return avl.Root == avl._NIL
}

// Push a chain of Node's into the AVL Tree
func (avl *AVL[T]) Push(keys ...T) {
    { … 4 line(s) … ⟦tj:a2ef1b85bd4aad26202fc1058ffb8ef0⟧ }

// Delete a Node from the AVL Tree
func (avl *AVL[T]) Delete(key T) bool {
    { … 7 line(s) … ⟦tj:0e4f6bde6f2e850a43efea147195b386⟧ }

// Get a Node from the AVL Tree
func (avl *AVL[T]) Get(key T) (Node[T], bool) {
	return searchTreeHelper[T](avl.Root, avl._NIL, key)
}

// Has Determines the tree has the node of Key
func (avl *AVL[T]) Has(key T) bool {
	_, ok := searchTreeHelper[T](avl.Root, avl._NIL, key)
	return ok
}

// PreOrder Traverses the tree in the following order Root --> Left --> Right
func (avl *AVL[T]) PreOrder() []T {
    { … 4 line(s) … ⟦tj:31cfa541c2cc29de48ede458e7088dc9⟧ }

// InOrder Traverses the tree in the following order Left --> Root --> Right
func (avl *AVL[T]) InOrder() []T {
	return inOrderHelper[T](avl.Root, avl._NIL)
}

// PostOrder traverses the tree in the following order Left --> Right --> Root
func (avl *AVL[T]) PostOrder() []T {
    { … 4 line(s) … ⟦tj:69e2758871ec8e028117f7ab9f7a0dc4⟧ }

// LevelOrder returns the level order traversal of the tree
func (avl *AVL[T]) LevelOrder() []T {
    { … 4 line(s) … ⟦tj:1634d3a053a7fd8ccd64a0c3be30e49b⟧ }

// AccessNodesByLayer accesses nodes layer by layer (2-D array),  instead of printing the results as 1-D array.
func (avl *AVL[T]) AccessNodesByLayer() [][]T {
	return accessNodeByLayerHelper[T](avl.Root, avl._NIL)
}

// Depth returns the calculated depth of the AVL tree
func (avl *AVL[T]) Depth() int {
	return calculateDepth[T](avl.Root, avl._NIL, 0)
}

// Max returns the Max value of the tree
func (avl *AVL[T]) Max() (T, bool) {
    { … 7 line(s) … ⟦tj:03fb58d2e178073a3089be801385dab0⟧ }

// Min returns the Min value of the tree
func (avl *AVL[T]) Min() (T, bool) {
    { … 7 line(s) … ⟦tj:84d30b329501a0aba4b79de2b3cc7ec8⟧ }

// Predecessor returns the Predecessor of the node of Key
// if there is no predecessor, return default value of type T and false
// otherwise return the Key of predecessor and true
func (avl *AVL[T]) Predecessor(key T) (T, bool) {
    { … 7 line(s) … ⟦tj:1aa336a3189149a40771df568a349ee8⟧ }

// Successor returns the Successor of the node of Key
// if there is no successor, return default value of type T and false
// otherwise return the Key of successor and true
func (avl *AVL[T]) Successor(key T) (T, bool) {
    { … 7 line(s) … ⟦tj:788dbc238e6a569cc253123f7c730d7c⟧ }

func (avl *AVL[T]) pushHelper(root *AVLNode[T], key T) *AVLNode[T] {
    { … 48 line(s) … ⟦tj:980887955ae29a60ec188453f315bb2b⟧ }

func (avl *AVL[T]) deleteHelper(root *AVLNode[T], key T) *AVLNode[T] {
    { … 69 line(s) … ⟦tj:5968df42eb2e9bf0e038350f6a17601a⟧ }

func (avl *AVL[T]) height(root *AVLNode[T]) int {
    { … 13 line(s) … ⟦tj:ac4c4a02d31189805dbfdda04678ad4e⟧ }

// balanceFactor : negative balance factor means subtree Root is heavy toward Left
// and positive balance factor means subtree Root is heavy toward Right side
func (avl *AVL[T]) balanceFactor(root *AVLNode[T]) int {
    { … 9 line(s) … ⟦tj:6833e3b19b64eaeaa420e27831f254b4⟧ }

func (avl *AVL[T]) leftRotate(x *AVLNode[T]) *AVLNode[T] {
    { … 16 line(s) … ⟦tj:805efcbfabd2ec7742d2c78dcb28d09e⟧ }

func (avl *AVL[T]) rightRotate(x *AVLNode[T]) *AVLNode[T] {
    { … 16 line(s) … ⟦tj:bcae9ddabd0666126496fc679b7b1258⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (7718 bytes) is available by calling tinyjuice_retrieve with token "b8ff72f42c7f56dcce67519bb8cc458f" (marker ⟦tj:b8ff72f42c7f56dcce67519bb8cc458f⟧)]