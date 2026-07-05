<?php

/*
 * Created by: Ramy-Badr-Ahmed (https://github.com/Ramy-Badr-Ahmed)
 * in Pull Request #163: https://github.com/TheAlgorithms/PHP/pull/163
 * and #173: https://github.com/TheAlgorithms/PHP/pull/173
 *
 * Please mention me (@Ramy-Badr-Ahmed) in any issue or pull request addressing bugs/corrections to this file.
 * Thank you!
 */

namespace DataStructures\AVLTree;

/**
 * Class AVLTree
 * Implements an AVL Tree data structure with self-balancing capability.
 */
class AVLTree
{
    private ?AVLTreeNode $root;
    private int $counter;

    public function __construct()
    {
        $this->root = null;
        $this->counter = 0;
    }

    /**
     * Get the root node of the AVL Tree.
     */
    public function getRoot(): ?AVLTreeNode
    {
        return $this->root;
    }

    /**
     * Retrieve a node by its key.
     *
     * @param mixed $key The key of the node to retrieve.
     * @return ?AVLTreeNode The node with the specified key, or null if not found.
     */
    public function getNode($key): ?AVLTreeNode
    {
        return $this->searchNode($this->root, $key);
    }

    /**
     * Get the number of nodes in the AVL Tree.
     */
    public function size(): int
    {
        return $this->counter;
    }

    /**
     * Insert a key-value pair into the AVL Tree.
     *
     * @param mixed $key The key to insert.
     * @param mixed $value The value associated with the key.
     */
    public function insert($key, $value): void
    {
        $this->root = $this->insertNode($this->root, $key, $value);
        $this->counter++;
    }

    /**
     * Delete a node by its key from the AVL Tree.
     *
     * @param mixed $key The key of the node to delete.
     */
    public function delete($key): void
    {
        $this->root = $this->deleteNode($this->root, $key);
        $this->counter--;
    }

    /**
     * Search for a value by its key.
     *
     * @param mixed $key The key to search for.
     * @return mixed The value associated with the key, or null if not found.
     */
    public function search($key)
    {
        $node = $this->searchNode($this->root, $key);
        return $node ? $node->value : null;
    }

    /**
     * Perform an in-order traversal of the AVL Tree.
     * Initiates the traversal on the root node directly and returns the array of key-value pairs.
     */
    public function inOrderTraversal(): array
    {
        return TreeTraversal::inOrder($this->root);
    }

    /**
     * Perform a pre-order traversal of the AVL Tree.
     * Initiates the traversal on the root node directly and returns the array of key-value pairs.
     */
    public function preOrderTraversal(): array
    {
        return TreeTraversal::preOrder($this->root);
    }

    /**
     * Perform a post-order traversal of the AVL Tree.
     * Initiates the traversal on the root node directly and returns the array of key-value pairs.
     */
    public function postOrderTraversal(): array
    {
        return TreeTraversal::postOrder($this->root);
    }

    /**
     * Perform a breadth-first traversal of the AVL Tree.
     */
    public function breadthFirstTraversal(): array
    {
        return TreeTraversal::breadthFirst($this->root);
    }

    /**
     * Check if the AVL Tree is balanced.
     * This method check balance starting from the root node directly
     */
    public function isBalanced(): bool
    {
        return $this->isBalancedHelper($this->root);
    }

    /**
     * Insert a node into the AVL Tree and balance the tree.
     *
     * @param ?AVLTreeNode $node The current node.
     * @param mixed $key The key to insert.
     * @param mixed $value The value to insert.
     * @return AVLTreeNode The new root of the subtree.
     */
    private function insertNode(?AVLTreeNode $node, $key, $value): AVLTreeNode
        if ($node === null) {
            return new AVLTreeNode($key, $value);
        { … 11 line(s) … }
        return $this->balance($node);

    /**
     * Delete a node by its key and balance the tree.
     *
     * @param ?AVLTreeNode $node The current node.
     * @param mixed $key The key of the node to delete.
     * @return ?AVLTreeNode The new root of the subtree.
     */
    private function deleteNode(?AVLTreeNode $node, $key): ?AVLTreeNode
        if ($node === null) {
            return null;
        { … 21 line(s) … }
        return $this->balance($node);

    /**
     * Search for a node by its key.
     *
     * @param ?AVLTreeNode $node The current node.
     * @param mixed $key The key to search for.
     * @return ?AVLTreeNode The node with the specified key, or null if not found.
     */
    private function searchNode(?AVLTreeNode $node, $key): ?AVLTreeNode
        { … 13 line(s) … }

    /**
     * Helper method to check if a subtree is balanced.
     *
     * @param ?AVLTreeNode $node The current node.
     * @return bool True if the subtree is balanced, false otherwise.
     */
    private function isBalancedHelper(?AVLTreeNode $node): bool
        if ($node === null) {
            return true;
        { … 10 line(s) … }
        return $this->isBalancedHelper($node->left) && $this->isBalancedHelper($node->right);

    /**
     * Balance the subtree rooted at the given node.
     *
     * @param ?AVLTreeNode $node The current node.
     * @return ?AVLTreeNode The new root of the subtree.
     */
    private function balance(?AVLTreeNode $node): ?AVLTreeNode
        if ($node->balanceFactor() > 1) {
            if ($node->left && $node->left->balanceFactor() < 0) {
        { … 12 line(s) … }
        return $node;

    /**
     * Perform a left rotation on the given node.
     *
     * @param AVLTreeNode $node The node to rotate.
     * @return AVLTreeNode The new root of the rotated subtree.
     */
    private function rotateLeft(AVLTreeNode $node): AVLTreeNode
        { … 10 line(s) … }

    /**
     * Perform a right rotation on the given node.
     *
     * @param AVLTreeNode $node The node to rotate.
     * @return AVLTreeNode The new root of the rotated subtree.
     */
    private function rotateRight(AVLTreeNode $node): AVLTreeNode
        { … 10 line(s) … }

    /**
     * Get the node with the minimum key in the given subtree.
     *
     * @param AVLTreeNode $node The root of the subtree.
     * @return AVLTreeNode The node with the minimum key.
     */
    private function getMinNode(AVLTreeNode $node): AVLTreeNode
    {
        while ($node->left) {
            $node = $node->left;
        }
        return $node;
    }

    /**
     * Serializes the segment tree into a JSON string.
     *
     * @return string The serialized AVL Tree as a JSON string.
     */
    public function serialize(): string
    {
        return json_encode($this->serializeTree($this->root));
    }

    /**
     * Recursively serializes the AVL Tree.
     *
     * @param AVLTreeNode|null $node
     * @return array
     */
    private function serializeTree(?AVLTreeNode $node): array
        { … 12 line(s) … }

    /**
     * Deserializes a JSON string into an AVL Tree object
     *
     * @param string $data The JSON representation of an AVL Tree to deserialize.
     */
    public function deserialize(string $data): void
    {
        $this->root = $this->deserializeTree(json_decode($data, true));
        $this->counter = 0;
        $this->updateNodeCount($this->root);
    }

    /**
     * Recursively deserializes an AVL Tree from an array representation.
     *
     * @param array $data The serialized data for the node.
     * @return AVLTreeNode|null The root node of the deserialized tree.
     */
    private function deserializeTree(array $data): ?AVLTreeNode
        { … 13 line(s) … }

    /**
     * Updates the deserialized tree size.
     *
     * @param AVLTreeNode|null $node The root node of the deserialized tree.
     */
    private function updateNodeCount(?AVLTreeNode $node): void
    {
        if ($node !== null) {
            $this->counter++;
            $this->updateNodeCount($node->left);
            $this->updateNodeCount($node->right);
        }
    }
}