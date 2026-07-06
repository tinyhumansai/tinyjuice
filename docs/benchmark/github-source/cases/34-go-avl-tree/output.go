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
	key    T
	parent *AVLNode[T]
	left   *AVLNode[T]
	right  *AVLNode[T]
	height int
}

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
	return &AVL[T]{
		Root: nil,
		_NIL: nil,
	}
}

// Empty determines the AVL tree is empty
func (avl *AVL[T]) Empty() bool {
	return avl.Root == avl._NIL
}

// Push a chain of Node's into the AVL Tree
func (avl *AVL[T]) Push(keys ...T) {
	for _, k := range keys {
		avl.Root = avl.pushHelper(avl.Root, k)
	}
}

// Delete a Node from the AVL Tree
func (avl *AVL[T]) Delete(key T) bool {
	if !avl.Has(key) {
		return false
	}

	avl.Root = avl.deleteHelper(avl.Root, key)
	return true
}

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
	traversal := make([]T, 0)
	preOrderRecursive[T](avl.Root, avl._NIL, &traversal)
	return traversal
}

// InOrder Traverses the tree in the following order Left --> Root --> Right
func (avl *AVL[T]) InOrder() []T {
	return inOrderHelper[T](avl.Root, avl._NIL)
}

// PostOrder traverses the tree in the following order Left --> Right --> Root
func (avl *AVL[T]) PostOrder() []T {
	traversal := make([]T, 0)
	postOrderRecursive[T](avl.Root, avl._NIL, &traversal)
	return traversal
}

// LevelOrder returns the level order traversal of the tree
func (avl *AVL[T]) LevelOrder() []T {
	traversal := make([]T, 0)
	levelOrderHelper[T](avl.Root, avl._NIL, &traversal)
	return traversal
}

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
	ret := maximum[T](avl.Root, avl._NIL)
	if ret == avl._NIL {
		var dft T
		return dft, false
	}
	return ret.Key(), true
}

// Min returns the Min value of the tree
func (avl *AVL[T]) Min() (T, bool) {
	ret := minimum[T](avl.Root, avl._NIL)
	if ret == avl._NIL {
		var dft T
		return dft, false
	}
	return ret.Key(), true
}

// Predecessor returns the Predecessor of the node of Key
// if there is no predecessor, return default value of type T and false
// otherwise return the Key of predecessor and true
func (avl *AVL[T]) Predecessor(key T) (T, bool) {
	node, ok := searchTreeHelper[T](avl.Root, avl._NIL, key)
	if !ok {
		var dft T
		return dft, ok
	}
	return predecessorHelper[T](node, avl._NIL)
}

// Successor returns the Successor of the node of Key
// if there is no successor, return default value of type T and false
// otherwise return the Key of successor and true
func (avl *AVL[T]) Successor(key T) (T, bool) {
	node, ok := searchTreeHelper[T](avl.Root, avl._NIL, key)
	if !ok {
		var dft T
		return dft, ok
	}
	return successorHelper[T](node, avl._NIL)
}

func (avl *AVL[T]) pushHelper(root *AVLNode[T], key T) *AVLNode[T] {
	if root == avl._NIL {
		return &AVLNode[T]{
	{ … 44 line(s) … ⟦tj:980887955ae29a60ec188453f315bb2b⟧ }
	return root

func (avl *AVL[T]) deleteHelper(root *AVLNode[T], key T) *AVLNode[T] {
	if root == avl._NIL {
		return root
	{ … 65 line(s) … ⟦tj:5968df42eb2e9bf0e038350f6a17601a⟧ }
	return root

func (avl *AVL[T]) height(root *AVLNode[T]) int {
	if root == avl._NIL {
		return 1
	{ … 9 line(s) … ⟦tj:ac4c4a02d31189805dbfdda04678ad4e⟧ }
	return 1 + max.Int(leftHeight, rightHeight)

// balanceFactor : negative balance factor means subtree Root is heavy toward Left
// and positive balance factor means subtree Root is heavy toward Right side
func (avl *AVL[T]) balanceFactor(root *AVLNode[T]) int {
	{ … 9 line(s) … ⟦tj:6833e3b19b64eaeaa420e27831f254b4⟧ }

func (avl *AVL[T]) leftRotate(x *AVLNode[T]) *AVLNode[T] {
	y := x.right
	yl := y.left
	{ … 12 line(s) … ⟦tj:805efcbfabd2ec7742d2c78dcb28d09e⟧ }
	return y

func (avl *AVL[T]) rightRotate(x *AVLNode[T]) *AVLNode[T] {
	y := x.left
	yr := y.right
	{ … 12 line(s) … ⟦tj:bcae9ddabd0666126496fc679b7b1258⟧ }
	return y
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (7718 bytes): call tinyjuice_retrieve with token "b8ff72f42c7f56dcce67519bb8cc458f"]