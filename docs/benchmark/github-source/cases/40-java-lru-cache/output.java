package com.thealgorithms.datastructures.caches;

import java.util.HashMap;
import java.util.Map;

/**
 * A Least Recently Used (LRU) Cache implementation.
 *
 * <p>An LRU cache is a fixed-size cache that maintains items in order of use. When the cache reaches
 * its capacity and a new item needs to be added, it removes the least recently used item first.
 * This implementation provides O(1) time complexity for both get and put operations.</p>
 *
 * <p>Features:</p>
 * <ul>
 *   <li>Fixed-size cache with configurable capacity</li>
 *   <li>Constant time O(1) operations for get and put</li>
 *   <li>Thread-unsafe - should be externally synchronized if used in concurrent environments</li>
 *   <li>Supports null values but not null keys</li>
 * </ul>
 *
 * <p>Implementation Details:</p>
 * <ul>
 *   <li>Uses a HashMap for O(1) key-value lookups</li>
 *   <li>Maintains a doubly-linked list for tracking access order</li>
 *   <li>The head of the list contains the least recently used item</li>
 *   <li>The tail of the list contains the most recently used item</li>
 * </ul>
 *
 * <p>Example usage:</p>
 * <pre>
 * LRUCache<String, Integer> cache = new LRUCache<>(3); // Create cache with capacity 3
 * cache.put("A", 1); // Cache: A=1
 * cache.put("B", 2); // Cache: A=1, B=2
 * cache.put("C", 3); // Cache: A=1, B=2, C=3
 * cache.get("A");    // Cache: B=2, C=3, A=1 (A moved to end)
 * cache.put("D", 4); // Cache: C=3, A=1, D=4 (B evicted)
 * </pre>
 *
 * @param <K> the type of keys maintained by this cache
 * @param <V> the type of mapped values
 */
public class LRUCache<K, V> {
    { … 193 line(s) … ⟦tj:b022773d05aaae685264226c28d2e363⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (6665 bytes) is available by calling tinyjuice_retrieve with token "297a09e22043249ecebe054941693b12" (marker ⟦tj:297a09e22043249ecebe054941693b12⟧)]