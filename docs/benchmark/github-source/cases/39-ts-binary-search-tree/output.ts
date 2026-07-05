/**
 * Represents a node of a binary search tree.
 *
 * @template T The type of the value stored in the node.
 */
class TreeNode<T> {
  constructor(
    public data: T,
    public leftChild?: TreeNode<T>,
    public rightChild?: TreeNode<T>
  ) {}
}

/**
 * An implementation of a binary search tree.
 *
 * A binary tree is a tree with only two children per node. A binary search tree on top sorts the children according
 * to following rules:
 * - left child < parent node
 * - right child > parent node
 * - all children on the left side < root node
 * - all children on the right side > root node
 *
 * For profound information about trees
 * @see https://www.geeksforgeeks.org/introduction-to-tree-data-structure-and-algorithm-tutorials/
 *
 * @template T The data type of the values in the binary tree.
 */
export class BinarySearchTree<T> {
  rootNode?: TreeNode<T>

  /**
   * Instantiates the binary search tree.
   *
   * @param rootNode The root node.
   */
  constructor() {
    this.rootNode = undefined
  }

  /**
   * Checks, if the binary search tree is empty, i. e. has no root node.
   *
   * @returns Whether the binary search tree is empty.
   */
  isEmpty(): boolean {
    return this.rootNode === undefined
  }

  /**
   * Checks whether the tree has the given data or not.
   *
   * @param data The data to check for.
   */
  has(data: T): boolean {
    if (!this.rootNode) {
      return false
    { … 19 line(s) … ⟦tj:c314b509a6e54416c9cb7e9eb985ca9e⟧ }
    return true
}

  /**
   * Inserts the given data into the binary search tree.
   *
   * @param data The data to be stored in the binary search tree.
   * @returns
   */
  insert(data: T): void {
    if (!this.rootNode) {
      this.rootNode = new TreeNode<T>(data)
    { … 20 line(s) … ⟦tj:d0937293e564ddc2c7605b529bb69e1d⟧ }
    }
}

  /**
   * Finds the minimum value of the binary search tree.
   *
   * @returns The minimum value of the binary search tree
   */
  findMin(): T { … 11 line(s) … ⟦tj:e14172a42b670bc2cd911e1e5648775c⟧ }

  /**
   * Finds the maximum value of the binary search tree.
   *
   * @returns The maximum value of the binary search tree
   */
  findMax(): T { … 11 line(s) … ⟦tj:3d7b9b43fd855181f4ccc90c6e752ec5⟧ }

  /**
   * Traverses to the binary search tree in in-order, i. e. it follow the schema of:
   * Left Node -> Root Node -> Right Node
   *
   * @param array The already found node data for recursive access.
   * @returns
   */
  inOrderTraversal(array: T[] = []): T[] {
    if (!this.rootNode) {
      return array
    { … 13 line(s) … ⟦tj:56a0cfe04911f60efc6baa21b22ee892⟧ }
    return traverse(this.rootNode)
}

  /**
   * Traverses to the binary search tree in pre-order, i. e. it follow the schema of:
   * Root Node -> Left Node -> Right Node
   *
   * @param array The already found node data for recursive access.
   * @returns
   */
  preOrderTraversal(array: T[] = []): T[] {
    if (!this.rootNode) {
      return array
    { … 14 line(s) … ⟦tj:3f4174a7b354ddfe8236979e1bb0e325⟧ }
    return traverse(this.rootNode)
}

  /**
   * Traverses to the binary search tree in post-order, i. e. it follow the schema of:
   * Left Node -> Right Node -> Root Node
   *
   * @param array The already found node data for recursive access.
   * @returns
   */
  postOrderTraversal(array: T[] = []): T[] {
    if (!this.rootNode) {
      return array
    { … 14 line(s) … ⟦tj:d2a4b84ecd9bb10491e2783baa27e559⟧ }
    return traverse(this.rootNode)
}
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (5205 bytes) is available by calling tinyjuice_retrieve with token "c015bc624a6090ff6b3173a0417647b5" (marker ⟦tj:c015bc624a6090ff6b3173a0417647b5⟧)]