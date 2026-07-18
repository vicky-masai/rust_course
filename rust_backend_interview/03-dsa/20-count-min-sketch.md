# Count-Min Sketch

## Interview Question

What is a Count-Min Sketch and when would you use it in backend systems?

## Interview Answer**

A Count-Min Sketch is a **probabilistic data structure** that estimates the frequency of items in a data stream. It uses a 2D array of `d` rows and `w` columns, with `d` independent hash functions. To record an item, hash it with each function and increment the corresponding counters. To query a frequency, take the minimum of all hashed counter values. It guarantees **no underestimation** but may **overestimate** due to hash collisions. The error is bounded: with w = e/ε and d = ln(1/δ), the estimate is within `ε × n` of the true frequency with probability `1 - δ`.

**Time Complexity**: O(d) for insert and query (d is constant, typically 4-7)
**Space Complexity**: O(w × d) — typically a few KB

---

## Follow-up Questions & Answers

### Q1. How would you implement a Count-Min Sketch in Rust?

**Interview Answer**

```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

struct CountMinSketch {
    depth: usize,
    width: usize,
    table: Vec<Vec<u64>>,
    seeds: Vec<u64>,
}

impl CountMinSketch {
    fn new(depth: usize, width: usize) -> Self {
        let seeds: Vec<u64> = (0..depth).map(|i| i as u64 * 0x5bd1e995).collect();
        CountMinSketch {
            depth,
            width,
            table: vec![vec![0; width]; depth],
            seeds,
        }
    }

    fn hash(&self, item: &impl Hash, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        item.hash(&mut hasher);
        hasher.finish() as usize % self.width
    }

    fn insert(&mut self, item: &impl Hash) {
        for i in 0..self.depth {
            let pos = self.hash(item, self.seeds[i]);
            self.table[i][pos] += 1;
        }
    }

    fn query(&self, item: &impl Hash) -> u64 {
        (0..self.depth)
            .map(|i| self.table[i][self.hash(item, self.seeds[i])])
            .min()
            .unwrap()
    }
}
```

---

### Q2. How is Count-Min Sketch used in network traffic monitoring?

**Interview Answer**

Routers and network switches use Count-Min Sketch for **heavy hitter detection** — identifying flows (source IP pairs) that consume disproportionate bandwidth. As packets arrive, their flow ID is inserted into the sketch. Periodically, query top flows by estimated frequency. This uses O(KB) of memory vs O(n) for exact counting (where n is the number of distinct flows — millions). **Denial-of-service detection**: sudden spikes in flow frequencies indicate attacks. **CDN analytics**: tracking popular content without storing every request. In Rust, network monitoring tools like `ntopng` use Count-Min Sketch for real-time traffic analysis.

---

### Q3. What is the difference between Count-Min Sketch and Bloom Filter?

**Interview Answer**

**Bloom Filter**: Tests **membership** (is this element in the set?) — returns yes/no with false positives. **Count-Min Sketch**: Estimates **frequency** (how many times has this element appeared?) — returns a count that may overestimate. Both are space-efficient probabilistic structures. Bloom Filter uses a bit array; Count-Min Sketch uses counter arrays. Both use multiple hash functions. The Bloom Filter has no false negatives; Count-Min Sketch has no underestimation. Use Bloom Filter for "have I seen this before?" Use Count-Min Sketch for "how many times have I seen this?" For frequency estimation with guaranteed bounds, Count-Min Sketch is the standard choice.

---

### Q4. What is the Space-Time tradeoff in Count-Min Sketch?

**Interview Answer**

**Width (w)**: Controls the overestimation error. w = e/ε ensures error < εn. Larger w → less error but more memory. **Depth (d)**: Controls the probability of the error bound holding. d = ln(1/δ) ensures the bound holds with probability 1-δ. Larger d → higher confidence but more hash computations. For 1% error (ε=0.01) with 99% confidence (δ=0.01): w ≈ 272, d = 5, total size ≈ 1,360 counters. At 8 bytes each, that's ~11 KB. This can track millions of items with 1% accuracy. For backend systems, this tradeoff is excellent — 11 KB of memory replaces potentially gigabytes of exact frequency maps.

---

### Q5. How does Count-Min Sketch handle decrements and deletions?

**Interview Answer**

Standard Count-Min Sketch only supports increments (insertions). For decrements, use **Count-Min Sketch with conservative update** or **Count Sketch** (which uses signed counters). Conservative update: only increment a counter if it's the minimum among all hashed positions — this reduces overestimation. Count Sketch: use +1/-1 increments at alternating hash positions, giving unbiased estimates with higher variance. For streaming with deletions (e.g., tracking active connections), Count Sketch is preferred. In Rust, use `Vec<i64>` instead of `Vec<u64>` for Count Sketch to support negative counters.

---

### Q6. How is Count-Min Sketch used in database query optimization?

**Interview Answer**

Database query optimizers use Count-Min Sketch to estimate **cardinality** (number of distinct values) and **frequency** of column values. PostgreSQL maintains frequency sketches in `pg_stat_statements` for query performance analysis. When the optimizer sees `WHERE status = 'active'`, it consults the sketch to estimate how many rows match, choosing between index scan and sequential scan. **Join estimation**: sketches on join columns help estimate result sizes. **Distinct count estimation**: combined with HyperLogLog for cardinality. The sketch is built during table scans and updated incrementally. This avoids full table scans just for statistics.

---

### Q7. What is a Conservative Count-Min Sketch?

**Interview Answer**

Conservative update modifies the standard Count-Min Sketch to reduce overestimation. When inserting an item: find the minimum value `min_val` among all hashed positions. Only increment positions where the current value equals `min_val`. This prevents positions that were inflated by other items from being incremented further. The result: estimates are much closer to true frequencies, especially for items with low counts. The space complexity is the same, but the practical accuracy improves significantly. The downside: insertions are slightly slower (need to read all positions before writing). For most backend applications, the conservative variant is preferred.

---

### Q8. Can Count-Min Sketch be used for top-K element detection?

**Interview Answer**

Yes, combined with a min-heap. Maintain a Count-Min Sketch for frequency estimation and a min-heap of size K tracking the current top-K candidates. For each new item: query its estimated frequency. If it's in the heap, update its count. If it's not in the heap and its count exceeds the heap minimum, replace the minimum. Periodically clean up the heap by re-querying frequencies (since estimates may have changed). This gives approximate top-K in O(n × d × log K) time with O(w × d + K) space. Used in: network traffic analysis (top talkers), database query analysis (top queries), and recommendation systems (top trending items).

---

### Q9. How does Count-Min Sketch handle hash collisions in practice?

**Interview Answer**

Hash collisions cause overestimation — when two items hash to the same counter, querying either returns the sum of both frequencies. The multi-hash design (d hash functions) mitigates this: taking the **minimum** across d counters reduces the impact because it's unlikely all d hash functions collide for the same pair. With w = e/ε and d = ln(1/δ), the probability that any estimate exceeds the true frequency by more than εn is less than δ. In practice, for well-distributed hash functions, the overestimation is much smaller than the theoretical bound. The `fnv` and `murmur3` hash families are commonly used for their good distribution properties.

---

### Q10. What is the relationship between Count-Min Sketch and frequency moments?

**Interview Answer**

The k-th frequency moment is `F_k = Σ f_i^k` where `f_i` is the frequency of the i-th distinct item. **F_0** = number of distinct items (cardinality — use HyperLogLog). **F_1** = total number of items (trivial counter). **F_2** = sum of squared frequencies (use Count Sketch). Count-Min Sketch estimates individual frequencies, from which frequency moments can be computed. **AMS (Alon-Matias-Szegedy) algorithm** uses Count Sketch variants to estimate F_2 in O(1) space. For backend systems, F_2 measures "burstiness" — high F_2 means a few items dominate (like a DDoS attack). Count-Min Sketch is the practical foundation for all frequency-based streaming analytics.
