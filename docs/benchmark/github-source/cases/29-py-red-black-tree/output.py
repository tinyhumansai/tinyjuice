from __future__ import annotations

from collections.abc import Iterator


class RedBlackTree:
    """
    A Red-Black tree, which is a self-balancing BST (binary search
    tree).
    This tree has similar performance to AVL trees, but the balancing is
    less strict, so it will perform faster for writing/deleting nodes
    and slower for reading in the average case, though, because they're
    both balanced binary search trees, both will get the same asymptotic
    performance.
    To read more about them, https://en.wikipedia.org/wiki/Red-black_tree
    Unless otherwise specified, all asymptotic runtimes are specified in
    terms of the size of the tree.
    """

    def __init__(
        self,
        label: int | None = None,
        color: int = 0,
        parent: RedBlackTree | None = None,
        left: RedBlackTree | None = None,
        right: RedBlackTree | None = None,
    ) -> None:
        """Initialize a new Red-Black Tree node with the given values:
        label: The value associated with this node
        color: 0 if black, 1 if red
        parent: The parent to this node
        left: This node's left child
        right: This node's right child
        """
        self.label = label
        self.parent = parent
        self.left = left
        self.right = right
        self.color = color

    # Here are functions which are specific to red-black trees

    def rotate_left(self) -> RedBlackTree:
        """Rotate the subtree rooted at this node to the left and
        returns the new root to this subtree.
...  # 17 line(s) collapsed ⟦tj:2d75ce0d2b19ff14671b16d3e6a80c71⟧
        return right

    def rotate_right(self) -> RedBlackTree:
        """Rotate the subtree rooted at this node to the right and
        returns the new root to this subtree.
...  # 17 line(s) collapsed ⟦tj:57b1ba48ed62415ddd706f934f108f5c⟧
        return left

    def insert(self, label: int) -> RedBlackTree:
        """Inserts label into the subtree rooted at self, performs any
        rotations necessary to maintain balance, and then returns the
...  # 20 line(s) collapsed ⟦tj:befc51b2ab78f3e7262e26b070551572⟧
        return self.parent or self

    def _insert_repair(self) -> None:
        """Repair the coloring from inserting into a tree."""
        if self.parent is None:
...  # 33 line(s) collapsed ⟦tj:eb8901b7247bd361503e31762ef5946d⟧
                    self.grandparent._insert_repair()

    def remove(self, label: int) -> RedBlackTree:
        """Remove label from this tree."""
        if self.label == label:
...  # 50 line(s) collapsed ⟦tj:ca0594ef386d04b2c3cd01c176d85a0d⟧
        return self.parent or self

    def _remove_repair(self) -> None:
        """Repair the coloring of the tree that may have been messed up."""
        if (
...  # 68 line(s) collapsed ⟦tj:8b779663a824d8051f2d6ceb1cafdae2⟧
            self.parent.sibling.color = 0

    def check_color_properties(self) -> bool:
        """Check the coloring of the tree, and return True iff the tree
        is colored in a way which matches these five properties:
...  # 28 line(s) collapsed ⟦tj:6ae263c8b4785c8ff038ca7fe5fc2532⟧
        return True

    def check_coloring(self) -> bool:
        ...  # 8 line(s) collapsed ⟦tj:6f222bb54b2d9cd83443aeb145866922⟧

    def black_height(self) -> int | None:
        """Returns the number of black nodes from this node to the
        leaves of the tree, or None if there isn't one such value (the
...  # 15 line(s) collapsed ⟦tj:a94beb9afa94e459836439dca902e2a7⟧
        return left + (1 - self.color)

    # Here are functions which are general to all binary search trees

    def __contains__(self, label: int) -> bool:
        """Search through the tree for label, returning True iff it is
        found somewhere in the tree.
        Guaranteed to run in O(log(n)) time.
        """
        return self.search(label) is not None

    def search(self, label: int) -> RedBlackTree | None:
        """Search through the tree for label, returning its node if
        it's found, and None otherwise.
...  # 12 line(s) collapsed ⟦tj:80cb2d2ac7ee2d0164a8757f921d7f5d⟧
            return self.left.search(label)

    def floor(self, label: int) -> int | None:
        """Returns the largest element in this tree which is at most label.
        This method is guaranteed to run in O(log(n)) time."""
...  # 12 line(s) collapsed ⟦tj:6dec1252e934e8b07966f01932645985⟧
            return self.label

    def ceil(self, label: int) -> int | None:
        """Returns the smallest element in this tree which is at least label.
        This method is guaranteed to run in O(log(n)) time.
...  # 13 line(s) collapsed ⟦tj:8de4eb1f4a101aeeb1194d74e8e752f6⟧
            return self.label

    def get_max(self) -> int | None:
        ...  # 8 line(s) collapsed ⟦tj:8bdfb21b11fe5bbefc6b8febba4f8615⟧

    def get_min(self) -> int | None:
        ...  # 8 line(s) collapsed ⟦tj:adbc3c85206b25c59ea3627f2120a5cc⟧

    @property
    def grandparent(self) -> RedBlackTree | None:
        """Get the current node's grandparent, or None if it doesn't exist."""
        if self.parent is None:
            return None
        else:
            return self.parent.parent

    @property
    def sibling(self) -> RedBlackTree | None:
        """Get the current node's sibling, or None if it doesn't exist."""
        if self.parent is None:
            return None
        elif self.parent.left is self:
            return self.parent.right
        else:
            return self.parent.left

    def is_left(self) -> bool:
        """Returns true iff this node is the left child of its parent."""
        if self.parent is None:
            return False
        return self.parent.left is self

    def is_right(self) -> bool:
        """Returns true iff this node is the right child of its parent."""
        if self.parent is None:
            return False
        return self.parent.right is self

    def __bool__(self) -> bool:
        return True

    def __len__(self) -> int:
        ...  # 9 line(s) collapsed ⟦tj:ebd8f15925c8f11b71d6cd845452af0b⟧

    def preorder_traverse(self) -> Iterator[int | None]:
        yield self.label
        if self.left:
            yield from self.left.preorder_traverse()
        if self.right:
            yield from self.right.preorder_traverse()

    def inorder_traverse(self) -> Iterator[int | None]:
        if self.left:
            yield from self.left.inorder_traverse()
        yield self.label
        if self.right:
            yield from self.right.inorder_traverse()

    def postorder_traverse(self) -> Iterator[int | None]:
        if self.left:
            yield from self.left.postorder_traverse()
        if self.right:
            yield from self.right.postorder_traverse()
        yield self.label

    def __repr__(self) -> str:
        from pprint import pformat

...  # 10 line(s) collapsed ⟦tj:d44c4d9bcc3f958d1c37644a7d3588f1⟧
        )

    def __eq__(self, other: object) -> bool:
        """Test if two trees are equal."""
        if not isinstance(other, RedBlackTree):
            return NotImplemented
        if self.label == other.label:
            return self.left == other.left and self.right == other.right
        else:
            return False


def color(node: RedBlackTree | None) -> int:
    """Returns the color of a node, allowing for None leaves."""
    if node is None:
        return 0
    else:
        return node.color


"""
Code for testing the various
functions of the red-black tree.
"""


def test_rotations() -> bool:
    """Test that the rotate_left and rotate_right functions work."""
    # Make a tree to test on
...  # 28 line(s) collapsed ⟦tj:7d1ff0bd5bbe190bad63ee4d195099ff⟧
    return tree == right_rot


def test_insertion_speed() -> bool:
    """Test that the tree balances inserts to O(log(n)) by doing a lot
    of them.
    """
    tree = RedBlackTree(-1)
    for i in range(300000):
        tree = tree.insert(i)
    return True


def test_insert() -> bool:
    """Test the insert() method of the tree correctly balances, colors,
    and inserts.
...  # 15 line(s) collapsed ⟦tj:f4efcfb66393e30deadc97a9f0818ecd⟧
    return tree == ans


def test_insert_and_search() -> bool:
    """Tests searching through the tree for values."""
    tree = RedBlackTree(0)
...  # 10 line(s) collapsed ⟦tj:ff4ad8b630ad119a070eb5bce9ca26e7⟧
    return all(i in tree for i in (11, 12, -8, 0))


def test_insert_delete() -> bool:
    """Test the insert() and delete() method of the tree, verifying the
    insertion and removal of elements, and the balancing of the tree.
...  # 16 line(s) collapsed ⟦tj:b6ce061d7d7792a5dab40659ac1bc13c⟧
    return list(tree.inorder_traverse()) == [-8, 0, 4, 8, 10, 11, 12]


def test_floor_ceil() -> bool:
    """Tests the floor and ceiling functions in the tree."""
    tree = RedBlackTree(0)
...  # 10 line(s) collapsed ⟦tj:2ca8cb739269cbe6d9d866cc990c6fca⟧
    return True


def test_min_max() -> bool:
    ...  # 9 line(s) collapsed ⟦tj:e8e7a6939a49692824f75633d610bdcf⟧


def test_tree_traversal() -> bool:
    """Tests the three different tree traversal functions."""
    tree = RedBlackTree(0)
...  # 10 line(s) collapsed ⟦tj:a55d5a0ea8e2fe9a8518b8c6a7489dcb⟧
    return list(tree.postorder_traverse()) == [-16, 8, 20, 24, 22, 16, 0]


def test_tree_chaining() -> bool:
    ...  # 8 line(s) collapsed ⟦tj:42389fa4fea178af6462cede193209a5⟧


def print_results(msg: str, passes: bool) -> None:
    print(str(msg), "works!" if passes else "doesn't work :(")


def pytests() -> None:
    assert test_rotations()
    assert test_insert()
    assert test_insert_and_search()
    assert test_insert_delete()
    assert test_floor_ceil()
    assert test_tree_traversal()
    assert test_tree_chaining()


def main() -> None:
    """
    >>> pytests()
    """
    print_results("Rotating right and left", test_rotations())
    print_results("Inserting", test_insert())
    print_results("Searching", test_insert_and_search())
    print_results("Deleting", test_insert_delete())
    print_results("Floor and ceil", test_floor_ceil())
    print_results("Tree traversal", test_tree_traversal())
    print_results("Tree traversal", test_tree_chaining())
    print("Testing tree balancing...")
    print("This should only be a few seconds.")
    test_insertion_speed()
    print("Done!")


if __name__ == "__main__":
    main()
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (25467 bytes): call tinyjuice_retrieve with token "bd143cc1e37121407c74ca03595ccd59"]