# Segment Tree

## Interview Question

What is a Segment Tree and when would you use it in backend systems?

## Interview Answer

A Segment Tree is a binary tree data structure that supports efficient **range queries** and **point updates** on an array. Each node represents a segment (range) of the array, and stores the aggregate (sum, min, max, gcd) of that segment. It supports: **range query** in O(log n), **point update** in O(log n), and **range update** with lazy propagation in O(log n). It's used when you need to frequently query aggregates over subranges and update individual elements. In backend systems, Segment Trees power: time-series data aggregation, database range queries, real-time analytics dashboards, and competitive programming.

**Time Complexity**: O(log n) for query and update
**Space Complexity**: O(n) — 4n array representation

---

## Follow-up Questions & Answers

### Q1. How would you implement a Segment Tree in Rust?

**Interview Answer**

```rust
struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut st = SegmentTree { n, tree: vec![0; 4 * n] };
        st.build(arr, 1, 0, n - 1);
        st
    }

    fn build(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.build(arr, 2 * node, start, mid);
            self.build(arr, 2 * node + 1, mid + 1, end);
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    fn query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if r < start || end < l { return 0; }
        if l <= start && end <= r { return self.tree[node]; }
        let mid = (start + end) / 2;
        self.query(2 * node, start, mid, l, r)
            + self.query(2 * node + 1, mid + 1, end, l, r)
    }

    fn update(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64) {
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update(2 * node, start, mid, idx, val);
            } else {
                self.update(2 * node + 1, mid + 1, end, idx, val);
            }
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }
}
```

The tree is stored as an array where node `i` has children at `2i` and `2i + 1`.

---

### Q2. What is lazy propagation and when is it needed?

**Interview Answer**

Lazy propagation optimizes **range updates** (e.g., "add 5 to all elements in range [3, 7]"). Without lazy propagation, updating a range takes O(n) because you'd update each leaf individually. With lazy propagation, each node stores a "pending update" that hasn't been pushed to children. When querying, push the pending updates down only when needed. This keeps both range update and range query at O(log n). Each node stores both the aggregate value and a lazy tag. When a range update covers an entire node, store the tag and update the node's aggregate. When accessing children, propagate the tag first.

---

### Q3. How does Segment Tree compare to Fenwick Tree (Binary Indexed Tree)?

**Interview Answer**

**Segment Tree** is more versatile — supports any associative operation (sum, min, max, gcd, custom), range queries, range updates with lazy propagation, and can be extended to 2D. **Fenwick Tree** is simpler to implement, uses less memory (n + 1 array vs 4n), and has smaller constant factors. Fenwick supports point update + prefix sum in O(log n), and range update + range query with two Fenwick trees. **Choose Fenwick** for simple sum/range sum problems with point updates. **Choose Segment Tree** for min/max/gcd queries, range updates, or when you need maximum flexibility. In practice, Fenwick is preferred for its simplicity and cache performance.

---

### Q4. What are real-world applications of Segment Trees?

**Interview Answer**

**Time-series databases**: InfluxDB and TimescaleDB use Segment Tree-like structures (aggregation trees) for efficient time-range queries. When you query "total requests in the last hour," the database traverses the tree to aggregate precomputed chunks. **Database indexes**: Some OLAP databases use Segment Trees for range aggregation indexes. **Network monitoring**: Tracking minimum/maximum latency over sliding time windows. **Game development**: Collision detection using interval trees (a variant). **Compiler optimization**: Range-based data flow analysis. In Rust, implement a Segment Tree for real-time leaderboards — query the k-th player's score in O(log n) after each update.

---

### Q5. How would you implement a 2D Segment Tree?

**Interview Answer**

A 2D Segment Tree extends the concept to matrices. Each node in the outer tree contains a segment tree over columns. For an n×m matrix: build time O(n × m), range query O(log n × log m), point update O(log n × log m). Memory is O(n × m × 4 × 4) ≈ O(16nm). For many 2D problems, alternative approaches are more practical: **Offline processing** with 1D Segment Tree + sweep line, **Fenwick Tree 2D** for sum queries (simpler implementation), or **sqrt decomposition** for moderate constraints. In Rust, represent as `Vec<Vec<i64>>` with careful bounds checking. Use `vec![vec![0i64; 4*m]; 4*n]` for the tree storage.

---

### Q6. What is a persistent Segment Tree and when is it used?

**Interview Answer**

A persistent Segment Tree preserves all previous versions after each update. Each update creates O(log n) new nodes while sharing unchanged nodes with previous versions. This enables: **version queries** — "what was the sum at time t?", **range queries on historical data**, and **competitive programming** problems (k-th element in range). Space is O(n + q × log n) where q is the number of updates. Used in: version-controlled databases (querying historical states), temporal databases, and functional data structures. In Rust, use `Rc<Node>` or `Arc<Node>` for node sharing between versions, ensuring immutable shared structure.

---

### Q7. Can Segment Tree handle dynamic arrays (insertions/deletions)?

**Interview Answer**

A standard Segment Tree has fixed size. For dynamic arrays, alternatives exist: **Fenwick Tree with coordinate compression** for offline problems. **Balanced BST with augmented data** (order-statistics tree) for O(log n) insert/delete/rank. **Sqrt decomposition** — divide array into √n blocks, each a sorted Vec. Insert/delete in O(√n), query in O(√n). For most backend systems, the data size is known or bounded, so a static Segment Tree suffices. For truly dynamic data, combine a Segment Tree with a Fenwick Tree or use a B-Tree as the backing structure.

---

### Q8. How do you choose the right operation for a Segment Tree node?

**Interview Answer**

The operation must be **associative** — `(a op b) op c = a op (b op c)`. Common choices: **Sum** — range sum queries. **Min/Max** — range minimum/maximum queries. **GCD** — range GCD queries. **Bitwise AND/OR/XOR** — bitwise range queries. The operation determines the merge function at internal nodes. For sum: `tree[node] = tree[left] + tree[right]`. For min: `tree[node] = min(tree[left], tree[right])`. The merge function must also have an identity element (0 for sum, ∞ for min, -∞ for max). Custom operations are supported as long as they're associative.

---

### Q9. What is the relationship between Segment Tree and merge sort?

**Interview Answer**

A Segment Tree is structurally identical to a merge sort tree. The build process mirrors merge sort: divide the array in half, recursively build children, merge results at the parent. The key difference: merge sort builds the tree once and discards it after sorting. A Segment Tree builds once and supports O(log n) queries and updates afterward. A "merge sort tree" variant stores sorted subarrays at each node, enabling queries like "count elements ≤ x in range [l, r]" using binary search at each node — O(log² n) per query. This variant is useful for offline range queries in database systems.

---

### Q10. What are the space and cache performance considerations?

**Interview Answer**

A Segment Tree uses 4n space (worst case slightly less than 4n nodes). For n = 10^6, that's ~16MB with i64 values. **Cache performance** is a concern: tree traversal jumps between array positions, causing cache misses. **Improvement**: Use a bottom-up iterative implementation for better cache locality. **Alternative**: Fenwick Tree has better cache performance due to sequential memory access. For very large datasets, use **segment tree beats** (advanced variant) for more complex operations. In Rust, `Vec<i64>` provides contiguous memory. Consider padding to cache-line boundaries (64 bytes) for high-performance applications.
