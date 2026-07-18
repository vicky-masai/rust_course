# Merge K Sorted Lists

## Interview Question

Given an array of `k` sorted linked lists, merge them into a single sorted linked list.

## Interview Answer

The optimal approach uses a **Min Heap (Priority Queue)** that stores one node from each of the k lists. The heap always contains at most k elements, and we extract the minimum, append it to the result list, and push the next node from the same list into the heap. This processes elements one at a time in sorted order. The time complexity is **O(N log k)** where N is the total number of nodes across all lists, and the space complexity is **O(k)** for the heap.

---

## Follow-up Questions & Answers

### Q1. Why is the Min Heap approach optimal for this problem?

**Interview Answer**

The Min Heap approach is optimal because it minimizes comparisons. At each step, we only compare k elements (one from each list) rather than comparing all elements globally. The heap gives us the minimum in O(log k) time instead of O(k) for a linear scan. The total number of extractions and insertions is N (total nodes), each costing O(log k), giving **O(N log k)** total. Any comparison-based merge must do at least O(N log k) work, so this is asymptotically optimal. A brute-force approach (concatenate + sort) would be O(N log N), which is worse when k is small relative to N.

---

### Q2. How would you implement this in Rust?

**Interview Answer**

In Rust, since `std::collections::BinaryHeap` is a max heap, you wrap items in `Reverse` for min-heap behavior. Each heap entry stores `(value, list_index, node_index)` to track which list and position the node came from:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn merge_k_sorted(lists: Vec<Vec<i32>>) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    let mut indices: Vec<usize> = vec![0; lists.len()];

    for i in 0..lists.len() {
        if !lists[i].is_empty() {
            heap.push(Reverse((lists[i][0], i)));
            indices[i] = 1;
        }
    }

    let mut result = Vec::new();
    while let Some(Reverse((val, list_idx))) = heap.pop() {
        result.push(val);
        if indices[list_idx] < lists[list_idx].len() {
            let next = lists[list_idx][indices[list_idx]];
            indices[list_idx] += 1;
            heap.push(Reverse((next, list_idx)));
        }
    }
    result
}
```

---

### Q3. What are the alternative approaches and their trade-offs?

**Interview Answer**

Three main alternatives exist: **1) Min Heap** — O(N log k) time, O(k) space, best for k << N. **2) Divide and Conquer** — merge pairs of lists recursively (like merge sort), O(N log k) time but O(log k) recursion stack space; no heap allocation overhead. **3) Brute Force** — concatenate all lists and sort, O(N log N) time, O(N) space; simplest but slowest. The divide-and-conquer approach is often preferred in practice because it has better cache locality and avoids heap allocation. For very small k (e.g., k=2 or k=3), a simple sequential merge is fastest due to branch prediction and cache efficiency.

---

### Q4. How does the Divide and Conquer approach work?

**Interview Answer**

The Divide and Conquer approach recursively splits the k lists into halves, merges each half, then merges the two results — identical to merge sort's merge phase. Base case: merging two sorted lists takes O(n1 + n2) time using two pointers. The recursion depth is O(log k), and at each level, every node is processed exactly once, giving O(N log k) total. This avoids the heap overhead entirely. In Rust, you can implement this with a recursive function that takes a slice of lists and returns a merged `Vec<i32>`.

```rust
fn merge_two(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { result.push(a[i]); i += 1; }
        else { result.push(b[j]); j += 1; }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

fn merge_k_dc(lists: &[Vec<i32>]) -> Vec<i32> {
    if lists.len() == 1 { return lists[0].clone(); }
    if lists.len() == 2 { return merge_two(&lists[0], &lists[1]); }
    let mid = lists.len() / 2;
    let left = merge_k_dc(&lists[..mid]);
    let right = merge_k_dc(&lists[mid..]);
    merge_two(&left, &right)
}
```

---

### Q5. What edge cases should you handle?

**Interview Answer**

Key edge cases: **Empty list array** — return empty result. **Some or all lists are empty** — skip empty lists when populating the heap. **k = 1** — return the single list as-is. **Single-element lists** — each list has exactly one node. **All lists have identical elements** — the result should have all elements in order. **Lists of vastly different lengths** — one very long list, others very short. **Integer overflow** — not an issue in Rust with `i32`, but be aware of values at `i32::MIN`/`i32::MAX` boundaries. Always validate that the input is non-null and each list is already sorted.

---

### Q6. How does this apply to backend systems and databases?

**Interview Answer**

Merge K Sorted Lists is fundamental to **database query execution**. When a database performs a **merge join** or reads from multiple sorted index segments, it merges k sorted runs. **LSM-trees** (used in RocksDB, Cassandra, LevelDB) merge k sorted SSTables during compaction — this is exactly the merge-k-sorted-lists problem. **Log aggregation** systems merge sorted log streams from k servers. **Kafka consumers** merge messages from k partitions. In Rust backend services, this pattern appears when merging time-series data from multiple shards or combining results from parallel database queries.

---

### Q7. What is the time and space complexity comparison of all approaches?

**Interview Answer**

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Min Heap | O(N log k) | O(k) heap + O(N) output | Best when k << N |
| Divide & Conquer | O(N log k) | O(log k) stack + O(N) output | Best cache performance |
| Brute Force (sort) | O(N log N) | O(N) | Simplest code |
| Flatten + sort | O(N log N) | O(N) | Same as brute force |

The Min Heap and Divide & Conquer are asymptotically equivalent, but Divide & Conquer has better constants due to cache-friendly sequential access. The heap approach allocates/deallocates heap nodes repeatedly, which can cause cache misses.

---

### Q8. How would you handle this if the lists were stored on disk rather than in memory?

**Interview Answer**

When lists are on disk (e.g., SSTables in an LSM-tree), the merge becomes an **external merge sort**. You maintain k file handles, read one element (or block) from each into a memory buffer, use a min heap to find the minimum, write it to the output, and read the next element from that file. Modern databases optimize this by reading entire blocks (pages) at once to minimize I/O. The number of disk seeks is O(N) in the worst case, but block reads amortize this. **RocksDB** and **LevelDB** implement this exact pattern during SSTable compaction, using `Iterator` traits to abstract over sorted sequences.

---

### Q9. Can you use a heap from an external crate in Rust for this?

**Interview Answer**

Yes, several crates provide heap/priority queue implementations. The `heapless` crate provides stack-allocated priority queues (no heap allocation, useful for embedded). The `binary-heap-plus` crate adds comparison function support. For production, the standard library's `BinaryHeap` with `Reverse` is sufficient and well-optimized. If you need a custom comparator (e.g., merging by different fields), you can implement `Ord` on a wrapper struct or use `binary-heap-plus`. For very large k, consider a `BTreeMap`-based approach where keys are `(value, list_index)` — it provides O(log k) operations with ordered iteration.

---

### Q10. What if the k sorted lists are coming from a stream (unbounded)?

**Interview Answer**

If the lists are unbounded (streaming), you cannot load all elements into memory. The Min Heap approach still works — maintain k iterators (one per stream), each yielding the next element on demand. This requires lazy evaluation. In Rust, you can implement this using `Iterator` trait objects or async streams. Each stream produces elements one at a time, and the heap always has at most k entries. Memory usage is O(k) regardless of total elements processed. This is exactly how **Kafka consumers** merge messages from multiple topic partitions, and how **database cursors** merge results from parallel index scans without materializing the full result set.
