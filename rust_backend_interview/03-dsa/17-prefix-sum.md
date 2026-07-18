# Prefix Sum

## Interview Question

What is Prefix Sum and when would you use it in backend systems?

## Interview Answer

Prefix Sum is a preprocessing technique that computes cumulative sums of an array, enabling **O(1) range sum queries** after **O(n)** preprocessing. The prefix sum array `P` is defined as `P[i] = arr[0] + arr[1] + ... + arr[i]`. To query the sum of `arr[l..r]`, compute `P[r] - P[l-1]`. This transforms O(n) range sum queries into O(1). Prefix Sum extends to 2D (for matrix subregion sums), and combined with HashMap, solves "subarray sum equals k" problems. In backend systems, it's used for: cumulative metrics, dashboard aggregations, log analysis, and database range queries.

**Time Complexity**: O(n) preprocessing, O(1) per query
**Space Complexity**: O(n)

---

## Follow-up Questions & Answers

### Q1. How would you implement Prefix Sum in Rust?

**Interview Answer**

```rust
struct PrefixSum {
    prefix: Vec<i64>,
}

impl PrefixSum {
    fn new(arr: &[i64]) -> Self {
        let mut prefix = Vec::with_capacity(arr.len() + 1);
        prefix.push(0);
        for &val in arr {
            prefix.push(prefix.last().unwrap() + val);
        }
        PrefixSum { prefix }
    }

    fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix[r + 1] - self.prefix[l]
    }
}

// Usage:
let arr = [1, 2, 3, 4, 5];
let ps = PrefixSum::new(&arr);
assert_eq!(ps.range_sum(1, 3), 9); // arr[1] + arr[2] + arr[3]
```

Note the prefix array has length `n + 1` with `prefix[0] = 0` to handle the `l = 0` case cleanly.

---

### Q2. How is Prefix Sum used for "subarray sum equals k"?

**Interview Answer**

The classic approach uses Prefix Sum + HashMap. Iterate through the array, maintaining the running prefix sum. For each position `i`, check if `prefix_sum - k` exists in the HashMap — if so, there's a subarray ending at `i` with sum `k`. Store prefix sums and their frequencies in the HashMap. Time: O(n), Space: O(n). This handles both positive and negative numbers (unlike sliding window which requires non-negative). In Rust:

```rust
use std::collections::HashMap;

fn subarray_sum(arr: &[i32], k: i32) -> i32 {
    let mut count = 0;
    let mut prefix_sum = 0;
    let mut map = HashMap::new();
    map.insert(0, 1);

    for &val in arr {
        prefix_sum += val;
        if let Some(&c) = map.get(&(prefix_sum - k)) {
            count += c;
        }
        *map.entry(prefix_sum).or_insert(0) += 1;
    }
    count
}
```

---

### Q3. How does Prefix Sum extend to 2D matrices?

**Interview Answer**

For a 2D matrix, the prefix sum `P[i][j]` is the sum of all elements in the rectangle from (0,0) to (i-1, j-1). To compute the sum of a sub-rectangle (r1, c1) to (r2, c2), use the inclusion-exclusion principle: `P[r2+1][c2+1] - P[r1][c2+1] - P[r2+1][c1] + P[r1][c1]`. Preprocessing is O(n × m), each query is O(1). This is critical for image processing (computing sum of pixel regions), game development (area queries on 2D grids), and database analytics (aggregating over 2D data ranges).

```rust
fn build_2d_prefix(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let (n, m) = (matrix.len(), matrix[0].len());
    let mut prefix = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            prefix[i + 1][j + 1] = matrix[i][j]
                + prefix[i][j + 1]
                + prefix[i + 1][j]
                - prefix[i][j];
        }
    }
    prefix
}

fn range_sum_2d(prefix: &[Vec<i64>], r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
    prefix[r2 + 1][c2 + 1] - prefix[r1][c2 + 1] - prefix[r2 + 1][c1] + prefix[r1][c1]
}
```

---

### Q4. What is the difference between Prefix Sum and Segment Tree?

**Interview Answer**

**Prefix Sum**: O(n) preprocessing, O(1) range query, but O(n) point update (must recompute all subsequent prefixes). Best for **static data** with many read-only queries. **Segment Tree**: O(n) build, O(log n) query, O(log n) point update. Best for **dynamic data** with mixed reads and writes. Use Prefix Sum when data doesn't change after preprocessing. Use Segment Tree when data is updated frequently. For range minimum/maximum queries, Prefix Sum doesn't apply — use Segment Tree. Prefix Sum is also used as a building block within other algorithms (e.g., combined with HashMap for subarray sum problems).

---

### Q5. How is Prefix Sum used in difference arrays?

**Interview Answer**

A difference array is the inverse of prefix sum. Given a difference array `D` where `D[i] = arr[i] - arr[i-1]`, the original array is the prefix sum of D. The key application: perform multiple range updates efficiently. To add `val` to all elements in range `[l, r]`, set `D[l] += val` and `D[r+1] -= val`. After all updates, compute the prefix sum of D to get the final array. This reduces O(n × q) (q range updates, each O(n)) to O(n + q). Used in: database bulk updates, time-series data adjustments, and batch processing. In Rust, `Vec<i64>` with push operations handles this naturally.

---

### Q6. Can Prefix Sum be used for non-numeric data?

**Interview Answer**

Yes, with bitwise operations. **XOR Prefix Sum**: `P[i] = arr[0] ⊕ arr[1] ⊕ ... ⊕ arr[i]`. Range XOR query: `P[r] ⊕ P[l-1]`. Used for: finding the XOR of a range, finding the element that appears an odd number of times. **Bitwise OR/AND Prefix**: Similar patterns. **Categorical data**: Convert categories to integers and use prefix counts. For example, "count of 'error' log entries in time range [t1, t2]" uses a prefix count array where each position counts errors up to that point. In Rust, `BitXor` trait enables XOR prefix sums with generic types.

---

### Q7. How does Prefix Sum relate to integral images in computer vision?

**Interview Answer**

An **integral image** (also called summed-area table) is exactly a 2D prefix sum. For an image where each pixel has a value, the integral image at (x, y) stores the sum of all pixels from (0,0) to (x,y). This enables computing the sum of any rectangular region in O(1) time, which is critical for: **Haar cascade classifiers** (face detection — computing feature sums over rectangular regions), **image processing** (box blur, adaptive thresholding), and **texture analysis**. Viola-Jones face detection uses integral images for real-time face detection. The technique was first described in 1984 for graphics and independently rediscovered for computer vision.

---

### Q8. What is the difference between Prefix Sum and running sum?

**Interview Answer**

They're the same concept with different names. **Running sum** typically refers to the cumulative sum computed incrementally: `running_sum += arr[i]`. **Prefix sum** refers to the array of cumulative sums: `P[i] = P[i-1] + arr[i]`. The prefix sum array stores all running sums at each position, enabling O(1) range queries. Running sum is a process; prefix sum is a data structure. In backend systems, running sums are used for real-time counters (total requests served), while prefix sum arrays are used for batch analytics (total requests in time window). Both are O(n) time and O(1) space (running sum) or O(n) space (prefix sum array).

---

### Q9. How would you handle prefix sum with updates in a streaming context?

**Interview Answer**

For streaming data where elements arrive one at a time and you need range sum queries: use a **Fenwick Tree** (Binary Indexed Tree) which supports O(log n) updates and O(log n) prefix sum queries. Alternatively, for approximate answers, use a **Count-Min Sketch** or **exponential histogram** for sliding window sums. For exact answers with bounded time ranges, use a circular buffer of prefix sums. In Rust, `VecDeque<f64>` as a sliding window with running sum handles the streaming case. For production systems, Prometheus uses similar techniques for range queries on time-series data.

---

### Q10. What are the common pitfalls when implementing Prefix Sum?

**Interview Answer**

**Off-by-one errors**: The most common bug. Using `P[r] - P[l]` instead of `P[r+1] - P[l]` or `P[r] - P[l-1]` (depending on indexing). The safest approach: use 1-indexed prefix array with `P[0] = 0`. **Integer overflow**: Cumulative sums can exceed the original data range. Use `i64` or `u64` even if the input is `i32`. **Empty ranges**: Handle `l > r` gracefully (return 0). **Floating-point precision**: For floating-point data, prefix sums accumulate rounding errors. For high precision, use Kahan summation. **Single-element updates**: Recomputing the entire prefix array after one change is O(n) — use Fenwick Tree for dynamic data. In Rust, `checked_add` and `overflowing_mul` help catch overflow bugs.
