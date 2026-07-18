# Fenwick Tree (Binary Indexed Tree)

## Interview Question

What is a Fenwick Tree and when would you use it over a Segment Tree?

## Interview Answer

A Fenwick Tree (Binary Indexed Tree, BIT) is a data structure that supports **point updates** and **prefix sum queries** in O(log n) time. It's simpler and more space-efficient than a Segment Tree — uses a single array of size n+1 vs 4n. The key insight: each position `i` in the BIT stores the sum of a specific range of the original array, determined by the **lowest set bit** of `i`. BIT supports: point update (+val at index i), prefix sum query (sum of arr[0..i]), and range sum query (using two prefix sums). It's ideal for **frequency counting**, **inversion counting**, and simple range sum problems.

**Time Complexity**: O(log n) for update and query
**Space Complexity**: O(n)

---

## Follow-up Questions & Answers

### Q1. How would you implement a Fenwick Tree in Rust?

**Interview Answer**

```rust
struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree { tree: vec![0; n + 1] }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        idx += 1; // 1-indexed
        while idx < self.tree.len() {
            self.tree[idx] += delta;
            idx += idx & (!idx + 1); // lowest set bit
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        idx += 1; // 1-indexed
        let mut sum = 0;
        while idx > 0 {
            sum += self.tree[idx];
            idx &= idx - 1; // remove lowest set bit
        }
        sum
    }

    fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.query(r) - self.query(l - 1)
    }
}
```

The two key operations: `idx & (!idx + 1)` extracts the lowest set bit, and `idx & (idx - 1)` clears it.

---

### Q2. What is the difference between Fenwick Tree and Segment Tree?

**Interview Answer**

**Fenwick Tree**: Simpler to implement (~20 lines), uses O(n) space, constant factor is smaller. Supports point update + prefix sum in O(log n). Can be extended to range update + range query using two BITs. **Segment Tree**: More versatile, supports range update with lazy propagation, min/max/gcd queries, and custom operations. Uses O(4n) space. **Choose Fenwick** when: you only need sum queries with point updates, space matters, or you want simpler code. **Choose Segment Tree** when: you need min/max/gcd queries, range updates, or more complex operations. For competitive programming, Fenwick is preferred for its simplicity. For production systems needing flexibility, Segment Tree is often better.

---

### Q3. How is Fenwick Tree used for frequency counting?

**Interview Answer**

Fenwick Tree is perfect for maintaining frequency counts with range queries. Initialize with all zeros. For each element `x` in the stream, call `update(x, 1)`. To query "how many elements are ≤ x", call `query(x)`. To query "how many elements are in [l, r]", call `range_sum(l, r)`. Both operations are O(log n). This is used for: **Online rank queries** — "what rank is element x among all seen elements?" **Range frequency queries** — "how many times does value v appear in range [l, r]?" **Order statistics** — combined with binary search to find the k-th smallest element in O(log² n).

---

### Q4. How is Fenwick Tree used to count inversions?

**Interview Answer**

An inversion is a pair (i, j) where i < j but arr[i] > arr[j]. Counting inversions: iterate right-to-left through the array. For each element, query the BIT for "how many elements smaller than arr[i] have been seen?" — this is the number of inversions involving arr[i]. Then update the BIT with arr[i]. Time: O(n log n) vs O(n²) brute force. The values must be coordinate-compressed to [1, n] for the BIT indices. Used in: measuring array "sortedness," ranking similarity (Kendall tau distance), and database query optimization (estimating join selectivity).

---

### Q5. How do you support range updates with Fenwick Tree?

**Interview Answer**

Use **two BITs** (B1 and B2) to support range update + range query in O(log n). To add `val` to range [l, r]: `B1.update(l, val)`, `B1.update(r+1, -val)`, `B2.update(l, val*(l-1))`, `B2.update(r+1, -val*r)`. Prefix sum at index i: `B1.query(i)*i - B2.query(i)`. Range sum [l, r]: `prefix(r) - prefix(l-1)`. This is a mathematical trick based on difference arrays. The implementation is more complex than a simple BIT but maintains the O(n) space advantage over Segment Trees with lazy propagation.

---

### Q6. What is coordinate compression and why is it needed with Fenwick Tree?

**Interview Answer**

Fenwick Tree requires indices in [1, n]. When values are large (e.g., coordinates up to 10^9), coordinate compression maps them to [1, n] while preserving relative order. Steps: (1) collect all values, (2) sort and deduplicate, (3) replace each value with its rank. For example, [100, 300, 200, 100] becomes [1, 3, 2, 1]. This preserves all comparisons needed for counting inversions, frequency queries, etc. In Rust: `let mut sorted: Vec<i64> = arr.to_vec(); sorted.sort(); sorted.dedup();` then use binary search to find ranks. Time: O(n log n) for compression + O(n log n) for BIT operations.

---

### Q7. Can Fenwick Tree support 2D range queries?

**Interview Answer**

Yes, a **2D Fenwick Tree** supports point updates and range sum queries on a 2D grid. Each update/query touches O(log n × log m) cells. Space: O(n × m). Implementation: nested loops for update and query instead of single loops. Used for: 2D prefix sums with updates (e.g., updating a cell in a matrix and querying rectangle sums), image processing (sum of pixel values in a region), and grid-based analytics. The 2D BIT is simpler than a 2D Segment Tree and has better constant factors. In Rust, use `Vec<Vec<i64>>` with careful bounds checking.

---

### Q8. How is Fenwick Tree used for order statistics (k-th element)?

**Interview Answer**

To find the k-th smallest element in a dynamic set, use the BIT's binary search property. Start at position 0 and use the BIT's tree structure to navigate: check the left child's count; if it's ≥ k, go left; otherwise, go right and subtract. This finds the k-th element in O(log n) without explicitly binary searching. Implementation: start with `idx = 0`, and for each power-of-two step from high to low, check if `tree[idx + step] < k` — if so, add step to idx and subtract tree[idx] from k. Used in: real-time analytics (median calculation), database queries (TOP-K), and streaming statistics.

---

### Q9. What is a Fenwick Tree with point update and range query vs range update and point query?

**Interview Answer**

**Point update + Range query** (standard BIT): Update a single element, query prefix/range sums. Use case: frequency counting, streaming sums. **Range update + Point query** (difference array BIT): Add a value to a range, query a single element's total. Use case: interval scheduling (how many intervals cover point x?), cumulative effects (apply bonus to range of employees, query individual's total). **Range update + Range query**: Use two BITs (described in Q5). The difference is which operation is O(1) naturally and which requires the BIT structure. Choose based on which operation you do more frequently.

---

### Q10. What are the practical performance characteristics of Fenwick Tree vs alternatives?

**Interview Answer**

**Fenwick Tree**: Best constant factor due to simple bit operations and cache-friendly sequential access. ~20 lines of code. O(n) space. **Segment Tree**: 4× more memory, more cache misses due to tree traversal, but supports more operations. **Prefix Sum Array**: O(1) query but O(n) update — best for static data. **Skip List**: O(log n) for both but higher constant factor and O(n) space. In practice, for sum queries with point updates, Fenwick Tree is 2-3× faster than Segment Tree due to cache performance. The `fenwick` crate in Rust provides a well-optimized implementation. For very large datasets (10^7+ elements), Fenwick Tree's compact representation fits in L2 cache better than Segment Tree.
