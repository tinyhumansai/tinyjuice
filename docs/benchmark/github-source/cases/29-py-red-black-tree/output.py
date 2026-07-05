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
        ...  # 12 line(s) collapsed ⟦tj:9450e24d6e7d58b04ce863cb03ae6cf6⟧

    # Here are functions which are specific to red-black trees

    def rotate_left(self) -> RedBlackTree:
        ...  # 20 line(s) collapsed ⟦tj:2d75ce0d2b19ff14671b16d3e6a80c71⟧

    def rotate_right(self) -> RedBlackTree:
        ...  # 20 line(s) collapsed ⟦tj:57b1ba48ed62415ddd706f934f108f5c⟧

    def insert(self, label: int) -> RedBlackTree:
        ...  # 23 line(s) collapsed ⟦tj:befc51b2ab78f3e7262e26b070551572⟧

    def _insert_repair(self) -> None:
        ...  # 36 line(s) collapsed ⟦tj:eb8901b7247bd361503e31762ef5946d⟧

    def remove(self, label: int) -> RedBlackTree:
        ...  # 53 line(s) collapsed ⟦tj:ca0594ef386d04b2c3cd01c176d85a0d⟧

    def _remove_repair(self) -> None:
        ...  # 71 line(s) collapsed ⟦tj:8b779663a824d8051f2d6ceb1cafdae2⟧

    def check_color_properties(self) -> bool:
        ...  # 31 line(s) collapsed ⟦tj:6ae263c8b4785c8ff038ca7fe5fc2532⟧

    def check_coloring(self) -> bool:
        ...  # 8 line(s) collapsed ⟦tj:6f222bb54b2d9cd83443aeb145866922⟧

    def black_height(self) -> int | None:
        ...  # 18 line(s) collapsed ⟦tj:a94beb9afa94e459836439dca902e2a7⟧

    # Here are functions which are general to all binary search trees

    def __contains__(self, label: int) -> bool:
        ...  # 5 line(s) collapsed ⟦tj:2bd317852088786340149b3bee42ad04⟧

    def search(self, label: int) -> RedBlackTree | None:
        ...  # 15 line(s) collapsed ⟦tj:80cb2d2ac7ee2d0164a8757f921d7f5d⟧

    def floor(self, label: int) -> int | None:
        ...  # 15 line(s) collapsed ⟦tj:6dec1252e934e8b07966f01932645985⟧

    def ceil(self, label: int) -> int | None:
        ...  # 16 line(s) collapsed ⟦tj:8de4eb1f4a101aeeb1194d74e8e752f6⟧

    def get_max(self) -> int | None:
        ...  # 8 line(s) collapsed ⟦tj:8bdfb21b11fe5bbefc6b8febba4f8615⟧

    def get_min(self) -> int | None:
        ...  # 8 line(s) collapsed ⟦tj:adbc3c85206b25c59ea3627f2120a5cc⟧

    @property
    def grandparent(self) -> RedBlackTree | None:
        ...  # 5 line(s) collapsed ⟦tj:1bdd45dad3c7164b34cc35abe5540066⟧

    @property
    def sibling(self) -> RedBlackTree | None:
        ...  # 7 line(s) collapsed ⟦tj:69f0048fd96c3b0b13f80daad909df35⟧

    def is_left(self) -> bool:
        ...  # 4 line(s) collapsed ⟦tj:65f460f2b20e18818908ff123a0c4295⟧

    def is_right(self) -> bool:
        ...  # 4 line(s) collapsed ⟦tj:432019dab95fd673368c135f0fdcd51f⟧

    def __bool__(self) -> bool:
        return True

    def __len__(self) -> int:
        ...  # 9 line(s) collapsed ⟦tj:ebd8f15925c8f11b71d6cd845452af0b⟧

    def preorder_traverse(self) -> Iterator[int | None]:
        ...  # 5 line(s) collapsed ⟦tj:f7ba3ed7559e5dd76568c005ea2b940f⟧

    def inorder_traverse(self) -> Iterator[int | None]:
        ...  # 5 line(s) collapsed ⟦tj:8c6ccb1ab82bcfeb7f454c5b5672840f⟧

    def postorder_traverse(self) -> Iterator[int | None]:
        ...  # 5 line(s) collapsed ⟦tj:a3c8694fa7e2b41e4cf1e69036f22ae9⟧

    def __repr__(self) -> str:
        ...  # 13 line(s) collapsed ⟦tj:d44c4d9bcc3f958d1c37644a7d3588f1⟧

    def __eq__(self, other: object) -> bool:
        ...  # 7 line(s) collapsed ⟦tj:e5f9be90e699771941770d20052596ae⟧


def color(node: RedBlackTree | None) -> int:
    ...  # 5 line(s) collapsed ⟦tj:9e6fd66fef4430e51f4395a500f91df3⟧


"""
Code for testing the various
functions of the red-black tree.
"""


def test_rotations() -> bool:
    ...  # 31 line(s) collapsed ⟦tj:7d1ff0bd5bbe190bad63ee4d195099ff⟧


def test_insertion_speed() -> bool:
    ...  # 7 line(s) collapsed ⟦tj:63915eb27cc5776d9b126e39c0e8412e⟧


def test_insert() -> bool:
    ...  # 18 line(s) collapsed ⟦tj:f4efcfb66393e30deadc97a9f0818ecd⟧


def test_insert_and_search() -> bool:
    ...  # 13 line(s) collapsed ⟦tj:ff4ad8b630ad119a070eb5bce9ca26e7⟧


def test_insert_delete() -> bool:
    ...  # 19 line(s) collapsed ⟦tj:b6ce061d7d7792a5dab40659ac1bc13c⟧


def test_floor_ceil() -> bool:
    ...  # 13 line(s) collapsed ⟦tj:2ca8cb739269cbe6d9d866cc990c6fca⟧


def test_min_max() -> bool:
    ...  # 9 line(s) collapsed ⟦tj:e8e7a6939a49692824f75633d610bdcf⟧


def test_tree_traversal() -> bool:
    ...  # 13 line(s) collapsed ⟦tj:a55d5a0ea8e2fe9a8518b8c6a7489dcb⟧


def test_tree_chaining() -> bool:
    ...  # 8 line(s) collapsed ⟦tj:42389fa4fea178af6462cede193209a5⟧


def print_results(msg: str, passes: bool) -> None:
    print(str(msg), "works!" if passes else "doesn't work :(")


def pytests() -> None:
    ...  # 7 line(s) collapsed ⟦tj:b20322c50ae95f45f10a5156c136af19⟧


def main() -> None:
    ...  # 14 line(s) collapsed ⟦tj:45f7bf67cfa71dcd4c8bd2bf3f59d644⟧


if __name__ == "__main__":
    main()
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (25467 bytes) is available by calling tinyjuice_retrieve with token "bd143cc1e37121407c74ca03595ccd59" (marker ⟦tj:bd143cc1e37121407c74ca03595ccd59⟧)]