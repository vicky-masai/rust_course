# Time Complexity of HashMap

## Interview Question

What is the time complexity of HashMap operations and what causes worst-case performance?

## Interview Answer**

HashMap provides **O(1) average** time for `insert`, `get`, and `delete` operations. The **worst case is O(n)** — this occurs when all keys hash to the same bucket, creating a single linked list that must be traversed linearly. In practice, with a good hash function and load factor management, worst-case is extremely rare. Rust's `std::collections::HashMap` uses **SipHash** (DoS-resistant) and maintains a load factor below 0.875 by default, resizing when exceeded. The amortized cost of resizing is O(1) per operation.

---

## Follow-up Questions & Answers

### Q1. What causes O(n) worst-case in a HashMap?

**Interview Answer**

O(n) worst case occurs when all keys hash to the same bucket, degrading the HashMap to a linked list. This can happen with: **1) Poor hash function** — if the hash function maps many keys to the same value. **2) Adversarial input** — an attacker can craft keys that exploit predictable hash functions (HashDoS attack). **3) Pathological key patterns** — e.g., consecutive integers with a hash function that doesn't distribute well. Rust uses SipHash specifically because it's cryptographically strong and prevents HashDoS attacks. Java 8+ replaced linked lists with balanced trees in buckets (treeification) to cap worst-case at O(log n).

---

### Q2. How does Rust's HashMap handle hash collisions?

**Interview Answer**

Rust's `HashMap` uses **open addressing with robin-hood hashing** and **backward shift deletion**. Instead of chaining (linked lists at each bucket), it probes for the next empty slot when a collision occurs. Robin-hood hashing ensures that elements that are further from their ideal position get priority during probing, reducing variance. Backward shift deletion moves elements during removal to fill gaps, avoiding tombstones. The default hash function is SipHash-1-3, which provides excellent distribution and DoS resistance. The load factor threshold is 7/8 (0.875) — when exceeded, the table doubles in size and all elements are rehashed.

---

### Q3. What is the load factor and how does it affect performance?

**Interview Answer**

The **load factor** is `n/m` where n is the number of entries and m is the number of buckets. As the load factor increases: **1)** Collision probability increases, degrading O(1) toward O(n). **2)** Memory utilization improves (less wasted space). **3)** Resize frequency decreases. Rust's HashMap triggers resize at load factor > 0.875. A load factor of 0.5 gives excellent performance (~1 lookup per entry) but wastes 50% memory. A load factor of 0.9 wastes only 10% but has more collisions. The sweet spot for most applications is 0.5-0.75. For read-heavy workloads, a lower load factor (0.5) is better. For memory-constrained environments, a higher load factor (0.8-0.9) is acceptable.

---

### Q4. How does HashMap resizing work and what is the amortized cost?

**Interview Answer**

When the load factor exceeds the threshold (0.875 in Rust), the HashMap allocates a new array with double the capacity and rehashes all elements. The cost of a single resize is O(n) — every element must be rehashed and reinserted. However, this happens after n insertions, so the amortized cost per insertion is O(n)/n = O(1). In Rust, `HashMap::with_capacity()` pre-allocates to avoid resizing. The resize strategy is **geometric doubling** — capacity doubles each time, ensuring the total work across all resizes is O(n). After resize, the old array is dropped, which in Rust involves running destructors for all entries, potentially affecting performance.

---

### Q5. How does HashMap performance compare to BTreeMap in Rust?

**Interview Answer**

| Operation | HashMap | BTreeMap |
|-----------|---------|----------|
| Insert | O(1) avg, O(n) worst | O(log n) worst |
| Lookup | O(1) avg, O(n) worst | O(log n) worst |
| Range query | O(n log n) | O(log n + k) |
| Ordered iteration | O(n log n) sort needed | O(n) in order |
| Memory | Higher (load factor waste) | Lower (packed nodes) |
| Cache behavior | Random access | Better (sequential) |

Use **HashMap** for fast point lookups with no ordering requirements. Use **BTreeMap** when you need sorted keys, range queries, or ordered iteration. For small collections (< 20 elements), `BTreeMap` can be faster due to cache effects and lower overhead.

---

### Q6. What are real-world implications of HashMap performance in backend systems?

**Interview Answer**

HashMap performance is critical in: **Redis** — uses HashMap for key storage, O(1) lookup is fundamental to Redis's speed. **HTTP headers** — parsed into HashMaps for O(1) header access. **Session management** — session tokens map to user data via HashMap. **Database indexing** — hash indexes provide O(1) point lookups. **DNS caching** — domain names map to IP addresses via HashMap. In Rust backend services, HashMap is the go-to data structure for caching, configuration storage, and any key-value mapping. Understanding load factor and resizing helps optimize memory usage in high-throughput systems.

---

### Q7. How do you choose a good hash function for a HashMap?

**Interview Answer**

A good hash function should: **1) Distribute keys uniformly** across buckets to minimize collisions. **2) Be deterministic** — same key always produces the same hash. **3) Be fast** — O(1) computation. **4) Be avalanche-resistant** — small key changes produce very different hashes. Rust's default **SipHash** satisfies all of these and is also cryptographic (prevents HashDoS). For custom hash maps, alternatives include **FxHash** (faster, non-cryptographic, used in Rust compiler), **AHash** (fast, SIMD-accelerated), and **XxHash** (extremely fast, non-cryptographic). The `ahash` crate is recommended for performance-critical Rust applications where DoS resistance isn't needed.

---

### Q8. How does the `Entry` API in Rust's HashMap work and why is it useful?

**Interview Answer**

The `Entry` API provides an efficient way to handle the "get or insert" pattern without double lookup:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
let count = map.entry("key").or_insert(0);
*count += 1;
```

`entry()` returns an `Entry` enum: `Vacant(entry)` if the key doesn't exist, or `Occupied(entry)` if it does. Methods: `or_insert(default)` inserts default if vacant and returns a mutable reference. `or_insert_with(fn)` lazily computes the default. `or_default()` uses `Default::default()`. This is more efficient than `contains_key()` followed by `insert()` because it only hashes the key once. The `Entry` API is especially useful for counting patterns (word frequency, log aggregation) and is O(1) amortized.

---

### Q9. What is the difference between HashMap and HashSet in terms of complexity?

**Interview Answer**

`HashSet<T>` is implemented as a `HashMap<T, ()>` — it uses the same underlying hash table but stores unit values. The time complexities are identical: **O(1) average** for `insert`, `contains`, and `remove`. A HashSet is more memory-efficient than a HashMap with dummy values because it doesn't store the value (just the key). Use HashSet when you need fast membership testing without associated data (e.g., tracking seen IPs, deduplication). In Rust, `HashSet` is in `std::collections` and wraps `HashMap` internally. For Bloom Filter-like behavior (probabilistic membership with less memory), use a dedicated Bloom Filter crate instead.

---

### Q10. How does concurrent access affect HashMap performance?

**Interview Answer**

Rust's `std::collections::HashMap` is **not thread-safe** — concurrent reads and writes cause undefined behavior. Options for concurrent access: **1) `Mutex<HashMap>`** — simple but serializes all operations. **2) `RwLock<HashMap>`** — allows concurrent reads, exclusive writes. **3) `DashMap`** — sharded concurrent HashMap with per-bucket locking, providing near-linear scaling. **4) ` flurry`** — port of Java's ConcurrentHashMap with lock-free reads. For read-heavy workloads, `RwLock` is sufficient. For write-heavy or mixed workloads, `DashMap` or `flurry` are better. The `concurrent-hashmap` crate provides a thread-safe HashMap with lock-free reads. Performance degrades under contention — choosing the right concurrency strategy depends on the read/write ratio.
