# Skip List

## Interview Question

What is a Skip List and why is it used in certain database systems?

## Interview Answer

A Skip List is a **probabilistic data structure** that provides O(log n) average-case search, insert, and delete operations. It consists of multiple layers of sorted linked lists — the bottom layer contains all elements, and each higher layer acts as an "express lane" for the layer below. Elements are promoted to higher levels with a probability (typically 1/2). Search starts from the top layer, traversing forward until the next element would be too large, then dropping down a level. It achieves **O(log n)** expected time without the complexity of balanced trees like AVL or Red-Black trees.

---

## Follow-up Questions & Answers

### Q1. How does a Skip List achieve O(log n) search time?

**Interview Answer**

A Skip List with n elements has approximately `log₂(n)` levels on average (since each level has half the elements of the level below). Search begins at the topmost level and moves right until it would overshoot the target, then drops down one level. At each level, we skip over many elements, reducing the search space by approximately half. The expected number of steps at each level is O(1), and there are O(log n) levels, giving O(log n) total. This is analogous to binary search on a sorted array, but implemented with pointers instead of array indices. The probabilistic balancing ensures no degenerate cases.

---

### Q2. How would you implement a Skip List in Rust?

**Interview Answer**

```rust
use rand::Rng;

struct SkipNode {
    key: i32,
    value: String,
    forward: Vec<Option<usize>>,
}

struct SkipList {
    nodes: Vec<SkipNode>,
    head: usize,
    max_level: usize,
    probability: f64,
}

impl SkipList {
    fn new(max_level: usize, probability: f64) -> Self {
        let head = SkipNode {
            key: i32::MIN,
            value: String::new(),
            forward: vec![None; max_level],
        };
        SkipList {
            nodes: vec![head],
            head: 0,
            max_level,
            probability,
        }
    }

    fn random_level(&self) -> usize {
        let mut level = 1;
        let mut rng = rand::thread_rng();
        while rng.gen::<f64>() < self.probability && level < self.max_level {
            level += 1;
        }
        level
    }
}
```

The `forward` vector stores pointers to the next node at each level. The `random_level` function determines how many levels a new node occupies.

---

### Q3. What is the time and space complexity of Skip List operations?

**Interview Answer**

**Search**: O(log n) average, O(n) worst case (extremely unlikely). **Insert**: O(log n) average — find insertion point, then update forward pointers for each level the new node occupies. **Delete**: O(log n) average — find the node, then update forward pointers to bypass it. **Space**: O(n log n) average — each element appears in approximately 2 levels on average (with p=0.5). The worst case is O(n²) if all elements are promoted to all levels, but the probability is astronomically low. The probabilistic balancing means there are no rotations or restructuring operations, making Skip Lists simpler to implement than balanced trees.

---

### Q4. Why does Redis use Skip Lists for Sorted Sets (ZSET)?

**Interview Answer**

Redis uses Skip Lists for `ZSET` (sorted set) because they provide **O(log n)** insertion, deletion, and range queries with a simple implementation. Unlike Red-Black trees, Skip Lists don't require complex rotation logic, making the code easier to maintain and debug. Redis's Skip List implementation (in `t_zset.c`) stores elements sorted by score, enabling efficient range queries (`ZRANGEBYSCORE`), rank queries (`ZRANK`), and iteration. The probabilistic balancing is sufficient for Redis's workload — worst-case performance is extremely unlikely. Redis also combines the Skip List with a HashMap for O(1) score lookups, providing both ordered and random access.

---

### Q5. How does a Skip List compare to a Balanced BST (AVL/Red-Black)?

**Interview Answer**

| Feature | Skip List | AVL/Red-Black |
|---------|-----------|---------------|
| Search | O(log n) avg | O(log n) worst |
| Insert | O(log n) avg | O(log n) worst |
| Delete | O(log n) avg | O(log n) worst |
| Range queries | O(log n + k) | O(log n + k) |
| Implementation | Simple | Complex |
| Cache locality | Poor (pointer chasing) | Better (tree traversal) |
| Concurrent access | Lock-free possible | Requires rebalancing locks |
| Memory overhead | Higher (multiple pointers per node) | Lower (2-3 pointers per node) |

Skip Lists are preferred when simplicity, concurrent access, and range queries are priorities. Balanced BSTs are preferred when worst-case guarantees and cache efficiency matter more.

---

### Q6. What are the real-world applications of Skip Lists beyond Redis?

**Interview Answer**

Skip Lists are used in: **LevelDB/RocksDB** — the `memtable` (in-memory sorted buffer) is implemented as a Skip List, enabling concurrent writes and efficient range scans. **Apache HBase** — uses Skip Lists internally for sorted data. **Concurrent priority queues** — Skip Lists enable lock-free concurrent access. **IP routing tables** — longest prefix matching with Skip Lists. **Text editors** — maintaining line indices for efficient line-number lookups. **Memory allocators** — tracking free memory blocks. In Rust, the `skiplist` crate provides a doubly-linked Skip List implementation, and `crossbeam-skiplist` provides a concurrent variant for multi-threaded applications.

---

### Q7. How do you handle concurrent access to a Skip List?

**Interview Answer**

Skip Lists are naturally suited for concurrent access because insertions/deletions at each level are localized. **Lock-free Skip Lists** use compare-and-swap (CAS) operations to update pointers atomically — the `crossbeam-skiplist` crate in Rust provides this. The key insight is that during insertion, we only need to atomically update the `forward` pointers of the predecessor node at each level. **Lock-based** approaches use fine-grained locking (lock only the affected nodes, not the entire list). Redis uses a global lock for its Skip List, but alternatives like the `concurrent-skiplist` crate enable true lock-free operations. The `crossbeam` ecosystem in Rust provides excellent support for lock-free data structures.

---

### Q8. What is the memory layout impact on performance?

**Interview Answer**

Skip Lists have poor cache locality because nodes are allocated individually on the heap, leading to pointer chasing across non-contiguous memory. Each node stores multiple forward pointers (one per level), increasing memory overhead. Mitigation strategies: **1)** Use arena allocation (`bumpalo` crate in Rust) to allocate nodes contiguously. **2)** Reduce the number of levels (use p=0.25 instead of p=0.5). **3)** Use a `Vec<SkipNode>` with indices instead of raw pointers for better locality. Balanced BSTs like Red-Black trees generally have better cache performance because tree traversal follows a more predictable pattern. For performance-critical applications, consider B-Trees instead, which maximize cache line utilization.

---

### Q9. Can you implement a Skip List with Rust's ownership model safely?

**Interview Answer**

Yes, using indices instead of raw pointers. Store nodes in a `Vec<SkipNode>` where each node's `forward` vector contains `Option<usize>` indices. This avoids `unsafe` code entirely since Rust's `Vec` handles memory management. The head node is always at index 0. Insertions may require `push` to the vec and index updates. Deletions can use a free-list for index reuse (mark nodes as deleted without removing from the vec). The `skiplist` crate uses `unsafe` for doubly-linked Skip Lists with raw pointers, but the index-based approach is safer and nearly as fast. For production use, the `crossbeam-skiplist` crate provides a well-tested concurrent Skip List.

---

### Q10. What are the limitations of Skip Lists?

**Interview Answer**

**1) Probabilistic guarantees** — O(log n) is expected, not guaranteed (worst case O(n)). **2) Poor cache locality** — pointer chasing causes cache misses. **3) Higher memory overhead** — each node stores multiple forward pointers. **4) No constant-time operations** — unlike hash tables, Skip Lists always involve traversal. **5) Not disk-friendly** — B-Trees are preferred for disk-based storage due to fewer disk seeks. **6) Difficult to persist** — forward pointers become invalid when serialized. For backend systems, Skip Lists excel as in-memory sorted structures (Redis ZSET, memtables), while B-Trees are preferred for persistent storage (database indices).
