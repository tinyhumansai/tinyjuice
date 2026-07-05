/**
 * A heap is a complete binary tree
 * In a complete binary tree each level is filled before lower levels are added
 * Each level is filled from left to right
 *
 * In a (min|max) heap the value of every node is (less|greater) than that of its children
 *
 * The heap is often implemented using an array structure.
 * In the array implementation, the relationship between a parent index and its two children
 * are ((parentindex * 2) + 1) and ((parentindex * 2) + 2)
 */
export abstract class Heap<T> {
  protected heap: T[]
  // A comparison function. Returns true if a should be the parent of b.
  protected compare: (a: T, b: T) => boolean

  constructor(compare: (a: T, b: T) => boolean) { … 4 line(s) … ⟦tj:be0b603ce3085f47bdd24ab8c53aa4ea⟧ }

  /**
   * Compares the value at parentIndex with the value at childIndex
   * In a maxHeap, the value at parentIndex should be larger than the value at childIndex
   * In a minHeap, the value at parentIndex should be smaller than the value at childIndex
   */
  private isRightlyPlaced(childIndex: number, parentIndex: number): boolean {
    return this.compare(this.heap[parentIndex], this.heap[childIndex])
  }

  /**
   * In a maxHeap, the index with the larger value is returned
   * In a minHeap, the index with the smaller value is returned
   */
  private getChildIndexToSwap(
    leftChildIndex: number,
    rightChildIndex: number
  ): number { … 8 line(s) … ⟦tj:dcb84a17745802fe17ac95514de5922c⟧ }

  public insert(value: T): void { … 4 line(s) … ⟦tj:5f9f1aef4bd454c0bd86cee082060388⟧ }

  public extract(): T { … 7 line(s) … ⟦tj:e81e0416fbd3cf46c72eefea114e2b0a⟧ }

  public size(): number {
    return this.heap.length
  }

  public isEmpty(): boolean {
    return this.size() === 0
  }

  protected swap(a: number, b: number): void {
    ;[this.heap[a], this.heap[b]] = [this.heap[b], this.heap[a]]
  }

  protected bubbleUp(index: number = this.size() - 1): void { … 10 line(s) … ⟦tj:1a1fd5cd306243355600dbcfa172c818⟧ }

  private sinkDown(): void { … 18 line(s) … ⟦tj:2963b0929fd7273aa9ec7bddb00deac2⟧ }

  private getLeftChildIndex(index: number): number {
    return index * 2 + 1
  }

  private getRightChildIndex(index: number): number {
    return index * 2 + 2
  }

  public check(): void {
    this._check()
  }

  private _check(index: number = 0): void { … 22 line(s) … ⟦tj:38cd81861143707f6fcd53b70fa0d6d4⟧ }
}

export class MinHeap<T> extends Heap<T> {
  constructor(compare: (a: T, b: T) => boolean = (a: T, b: T) => a < b) {
    super(compare)
  }
}

export class MaxHeap<T> extends Heap<T> {
  constructor(compare: (a: T, b: T) => boolean = (a: T, b: T) => a > b) {
    super(compare)
  }
}

export class PriorityQueue<T> extends MinHeap<T> {
  // Maps from the n'th node to its index within the heap.
  private keys: number[]
  // Maps from element to its index with keys.
  private keys_index: (a: T) => number

  constructor(
    keys_index: (a: T) => number,
    num_keys: number,
    compare: (a: T, b: T) => boolean = (a: T, b: T) => a < b
  ) { … 5 line(s) … ⟦tj:2ecc07abd8381948e4f52fd60c4057ab⟧ }

  protected swap(a: number, b: number): void { … 6 line(s) … ⟦tj:36490f4c0eaf01a5a3c6de5a64fe640a⟧ }

  public insert(value: T): void { … 4 line(s) … ⟦tj:fd1e6fafd8c08aa773bb23651c089a72⟧ }

  public extract(): T { … 8 line(s) … ⟦tj:49acd1b8c345648491329d47efd89062⟧ }

  public increasePriority(idx: number, value: T): void { … 15 line(s) … ⟦tj:a5a81ac499398714426c7fdf598f10f3⟧ }
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (5726 bytes) is available by calling tinyjuice_retrieve with token "00c5ab612cd0c92619c52f54d185463b" (marker ⟦tj:00c5ab612cd0c92619c52f54d185463b⟧)]