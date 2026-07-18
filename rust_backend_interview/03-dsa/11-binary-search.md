# Binary Search

## Interview Question

Explain binary search and its variants. When would you use each variant in a backend system?

## Interview Answer

Binary search is an efficient algorithm for finding an element in a **sorted** data structure by repeatedly dividing the search interval in half. It compares the target value to the middle element and eliminates half the remaining elements each step. The classic variant finds an exact match in **O(log n)** time. Key variants include: **lower_bound** (first position where element >= target), **upper_bound** (first position where element > target), **binary search on answer** (searching over a range of possible solutions), and **rotated array search** (searching in a sorted but rotated array). Binary search is foundational for database index lookups, log-structured merge trees, and range queries.

**Time Complexity**: O(log n)
**Space Complexity**: O(1) iterative, O(log n) recursive

---

## Follow-up Questions & Answers

### Q1. What are the common binary search variants and when to use each?

**Interview Answer**

There are four primary variants: **Standard Binary Search** — finds exact match in sorted array, used for index lookups. **Lower Bound** — finds first position where element >= target, used for range queries and insert positions. **Upper Bound** — finds first position where element > target, used for counting elements in range [low, high). **Binary Search on Answer** — when the answer space is monotonic (e.g., minimum capacity to ship packages in D days), binary search over possible answers. In Rust, `slice.binary_search()` returns `Result<usize, usize>` where the `Err` variant gives the insertion point, which is equivalent to lower_bound.

---

### Q2. How would you implement binary search in Rust?

**Interview Answer**

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    None
}

fn lower_bound(arr: &[i32], target: i32) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}
```

Note `low + (high - low) / 2` instead of `(low + high) / 2` to prevent integer overflow — a critical detail in production code.

---

### Q3. How is binary search used in database index lookups?

**Interview Answer**

B-Tree and B+ Tree indexes — the backbone of most relational databases — use binary search within each node to find the correct child pointer or key. PostgreSQL's `btree` index performs binary search across sorted keys in O(log n) levels. LSM Trees (used in RocksDB, Cassandra) use binary search within SSTable indexes to locate the correct data block. When you execute `SELECT * FROM users WHERE id = 42`, PostgreSQL traverses the B+ Tree using binary search at each level, achieving O(log n) lookup. The actual disk I/O is O(log n) as well, since each level may require a separate page read.

---

### Q4. What is binary search on answer and how does it work?

**Interview Answer**

Binary search on answer is used when the problem asks for the minimum/maximum value that satisfies a condition, and the condition is monotonic — if value `x` works, all values > x also work (for minimization) or all values < x also work (for maximization). The approach: define a search range `[lo, hi]`, compute `mid`, check if `mid` satisfies the condition using a helper function, and adjust the range accordingly. Classic examples: "find minimum capacity to ship packages in D days" (LeetCode 1011), "split array largest sum" (LeetCode 410), "Koko eating bananas" (LeetCode 875). The time complexity is O(n * log(range)) where n is input size and range is the answer space.

---

### Q5. How do you handle rotated sorted arrays?

**Interview Answer**

A rotated sorted array is a sorted array that has been rotated at some pivot (e.g., `[4,5,6,7,0,1,2]`). To search: at each step, determine which half is sorted by comparing `arr[mid]` with `arr[low]`. If the left half is sorted (`arr[low] <= arr[mid]`), check if the target lies in that half. If the right half is sorted (`arr[mid] <= arr[high]`), check if the target lies there. This maintains O(log n) time. This is useful in backend systems where sorted data structures are periodically rotated due to append operations or ring buffer implementations.

```rust
fn search_rotated(arr: &[i32], target: i32) -> Option<usize> {
    let (mut low, mut high) = (0, arr.len() - 1);
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target { return Some(mid); }
        if arr[low] <= arr[mid] {
            if arr[low] <= target && target < arr[mid] { high = mid - 1; }
            else { low = mid + 1; }
        } else {
            if arr[mid] < target && target <= arr[high] { low = mid + 1; }
            else { high = mid - 1; }
        }
    }
    None
}
```

---

### Q6. What is the difference between `binary_search`, `partition_point`, and manual binary search in Rust?

**Interview Answer**

`slice.binary_search(&target)` returns `Result<usize, usize>` — the index on success, or the insertion point on failure. `slice.partition_point(|&x| x < target)` returns the index of the first element that does NOT satisfy the predicate (equivalent to lower_bound). Manual binary search gives full control over the comparison logic and termination condition. Use `binary_search` for exact lookups, `partition_point` for range queries, and manual implementation for custom logic (e.g., searching on a transformed value, or searching in a rotated array). `partition_point` is the cleanest for lower_bound semantics.

---

### Q7. How does binary search relate to logarithmic indexing in distributed systems?

**Interview Answer**

Binary search is the core of **log-structured** data management. In distributed databases like Cassandra and ScyllaDB, SSTables are sorted files. Binary search within the SSTable's index block locates the data block containing a key. The **Log-Structured Merge Tree (LSM Tree)** used in LevelDB, RocksDB, and Cassandra's storage engine relies on binary search within each level's sorted runs. In distributed systems, consistent hashing uses binary search on the hash ring to find the responsible node. Timestamp-based partitioning (e.g., time-series databases) uses binary search to identify which partition contains a given time range.

---

### Q8. What are the pitfalls of binary search implementation?

**Interview Answer**

The most common pitfalls are: **Integer overflow** in `mid = (low + high) / 2` — use `low + (high - low) / 2` or `low + ((high - low) >> 1)`. **Off-by-one errors** — choosing `<` vs `<=` and `mid + 1` vs `mid` incorrectly. **Infinite loops** when `low` and `high` don't converge — ensure both branches make progress. **Overflow in range calculation** for binary search on answer when the answer space is large (use `u64` or check bounds). **Non-monotonic data** — binary search requires sorted/monotonic data. In Rust, the compiler won't catch these logic errors, so unit tests with edge cases (empty slice, single element, all same elements, target at boundaries) are essential.

---

### Q9. Can binary search be applied to floating-point numbers?

**Interview Answer**

Yes, but with care. For floating-point binary search, instead of checking exact equality, check if the range is small enough: `while high - low > epsilon`. The epsilon should be appropriate for the problem — typically `1e-9` for competitive programming, but in production systems, the epsilon should be relative to the magnitude of values. Binary search on floating-point is used for: computing square roots, finding optimal floating-point parameters in optimization problems, and geometric computations. Be aware of floating-point precision issues — `f64` has ~15 decimal digits of precision, and subtraction of close values can lose significant digits.

---

### Q10. What is exponential search and how does it relate to binary search?

**Interview Answer**

Exponential search finds the range where an element might exist in an unbounded or very large sorted array, then performs binary search within that range. It starts with index 1 and doubles the index until it overshoots the target, then binary searches between `index/2` and `index`. Time complexity is O(log n) where n is the position of the element. It's useful when the array size is unknown or very large (e.g., searching in a file, a stream, or a dynamically growing sorted structure). In backend systems, exponential search is used for searching in unbounded logs or time-series data where the total size isn't known upfront.

```rust
fn exponential_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() { return None; }
    if arr[0] == target { return Some(0); }
    let mut bound = 1;
    while bound < arr.len() && arr[bound] <= target {
        bound *= 2;
    }
    let low = bound / 2;
    let high = bound.min(arr.len());
    arr[low..high].binary_search(&target).ok().map(|i| i + low)
}
```
