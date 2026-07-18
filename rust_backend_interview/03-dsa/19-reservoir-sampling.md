# Reservoir Sampling

## Interview Question

What is Reservoir Sampling and when would you use it in backend systems?

## Interview Answer

Reservoir Sampling is an algorithm for randomly selecting `k` items from a data stream of unknown or very large size `n`, with equal probability (1/n for each item). It works by maintaining a "reservoir" of size `k` and replacing items with decreasing probability as the stream progresses. For the i-th item (starting from index k), replace a random item in the reservoir with probability `k/i`. This ensures each item has an equal `k/n` chance of being selected. It's optimal for: streaming data, database sampling, A/B test user selection, and log analysis when you can't store all data.

**Time Complexity**: O(n) — single pass through the data
**Space Complexity**: O(k) — only store the reservoir

---

## Follow-up Questions & Answers

### Q1. How would you implement Reservoir Sampling in Rust?

**Interview Answer**

```rust
use rand::Rng;

fn reservoir_sampling(stream: &[i32], k: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut reservoir: Vec<i32> = stream[..k].to_vec();

    for i in k..stream.len() {
        let j = rng.gen_range(0..=i);
        if j < k {
            reservoir[j] = stream[i];
        }
    }
    reservoir
}

// Streaming variant
struct ReservoirSampler {
    reservoir: Vec<i32>,
    k: usize,
    count: usize,
}

impl ReservoirSampler {
    fn new(k: usize) -> Self {
        ReservoirSampler { reservoir: Vec::with_capacity(k), k, count: 0 }
    }

    fn add(&mut self, item: i32) {
        self.count += 1;
        if self.reservoir.len() < self.k {
            self.reservoir.push(item);
        } else {
            let j = rand::thread_rng().gen_range(0..self.count);
            if j < self.k {
                self.reservoir[j] = item;
            }
        }
    }
}
```

---

### Q2. Why is Reservoir Sampling needed instead of just taking the first k items?

**Interview Answer**

Taking the first `k` items gives a **biased** sample — items early in the stream are overrepresented, and items arriving after position `k` are never selected. Reservoir Sampling gives each item an **equal probability** of `k/n` regardless of its position. This is critical for: **A/B testing** — users must be selected uniformly, not just the first ones. **Database sampling** — rows are not in insertion order. **Stream processing** — data patterns change over time (non-stationary). Without Reservoir Sampling, statistical analyses would be biased toward early data points.

---

### Q3. How is Reservoir Sampling used in database query sampling?

**Interview Answer**

SQL's `TABLESAMPLE` clause uses Reservoir Sampling (or variants) to return a random subset of rows. PostgreSQL supports `TABLESAMPLE BERNOULLI(10)` (10% sample) and `TABLESAMPLE SYSTEM(10)` (block-level sampling). The optimizer uses Reservoir Sampling for: estimating query results before full execution, cardinality estimation, and cost-based optimization. When you run `SELECT * FROM users ORDER BY RANDOM() LIMIT 100`, a naive approach loads all rows. Reservoir Sampling can produce the same result in a single pass with O(100) memory. In distributed databases, each node runs local Reservoir Sampling and results are merged.

---

### Q4. What is weighted Reservoir Sampling and when is it needed?

**Interview Answer**

Weighted Reservoir Sampling (Algorithm A-Res) assigns different selection probabilities to items based on weights. Each item's key is `u^(1/w)` where `u` is random [0,1] and `w` is the weight. The k items with the highest keys form the reservoir. Used when: items have different importance (e.g., sampling popular products for recommendation), stratified sampling (oversampling minority groups), and weighted random selection in load balancing. In Rust, compute `rand_f64.powf(1.0 / weight)` for each item and maintain a min-heap of size k. The `rand` crate provides uniform random numbers needed for this computation.

---

### Q5. How does Reservoir Sampling handle weighted distributions in distributed systems?

**Interview Answer**

In distributed systems, each node runs local Reservoir Sampling on its partition. To merge: collect all local reservoirs and run Reservoir Sampling on the combined set, adjusting probabilities for the merge step. For weighted sampling: each node computes local weighted samples, then a coordinator merges them using the weights. This is used in: **Spark's `sample()`** operation, **distributed A/B testing** (ensuring uniform user selection across shards), and **log aggregation** (sampling from multiple log streams). The merge step requires knowing the total count `n` across all partitions, which can be computed via a MapReduce count step.

---

### Q6. What is the difference between Reservoir Sampling and蓄水池抽样?

**Interview Answer**

They're the same algorithm — "蓄水池抽样" is the Chinese name for Reservoir Sampling, literally translating to "reservoir sampling." The algorithm was popularized by Chinese computer science literature. The core idea is identical: maintain a reservoir of size k, replace items with probability k/i. The algorithm is attributed to Alan Waterman (1963) and independently described by Vitter (1985) with an optimized version (Algorithm R). Understanding both names is useful when reading Chinese technical literature or interviewing with Chinese tech companies.

---

### Q7. Can Reservoir Sampling be used for sampling from a database without loading all rows?

**Interview Answer**

Yes, that's its primary advantage. Execute a query that streams rows one at a time (using a cursor in PostgreSQL, or streaming results in MySQL). For each row, apply the Reservoir Sampling algorithm. After all rows are processed, the reservoir contains a uniform random sample. This uses O(k) memory regardless of table size — crucial for tables with billions of rows. PostgreSQL's `TABLESAMPLE SYSTEM` uses a block-level approximation (faster but less uniform). For exact uniform sampling, use `TABLESAMPLE BERNOULLI` or implement Reservoir Sampling at the application level with cursor-based iteration.

---

### Q8. What is Algorithm Z (Vitter's Algorithm) and how does it improve Reservoir Sampling?

**Interview Answer**

Algorithm Z by Vitter (1985) improves upon basic Reservoir Sampling (Algorithm R) by **skipping** items that don't need processing. Instead of processing every item with probability k/i, it uses a random number to determine how many items to skip. This reduces the number of random numbers generated and comparisons made, improving performance for very large streams. Time complexity remains O(n) but with smaller constant factors. For most backend systems, basic Reservoir Sampling is sufficient. Algorithm Z is important for systems processing millions of items per second where the constant factor matters.

---

### Q9. How do you verify that Reservoir Sampling produces uniform samples?

**Interview Answer**

**Mathematical proof**: After processing n items, the probability that any specific item is in the reservoir is k/n. By induction: for item i > k, P(in reservoir) = P(replace) × P(not replaced later) = (k/i) × Π(j=i+1 to n, 1 - 1/j × k/i...) = k/n. **Empirical verification**: Run the algorithm many times (e.g., 100,000 iterations) with n=1000, k=10. Count how often each item appears. The frequency should be approximately k/n = 1% with uniform distribution. In Rust, use statistical tests: chi-squared test to verify uniformity, or simply check that the variance of selection counts is within expected bounds.

---

### Q10. What are the alternatives to Reservoir Sampling for streaming data?

**Interview Answer**

**蓄水池抽样 (Reservoir Sampling)**: Best for uniform random sampling. **Exponential decay sampling**: Weight recent items more heavily (useful for time-series). **Deterministic sampling**: Hash-based (e.g., `hash(item_id) % 100 < 5` for 5% sample) — deterministic but biased by hash distribution. **Block sampling**: Sample every k-th item — simple but biased by periodic patterns. **Count-Min Sketch**: For frequency estimation, not sampling. **HyperLogLog**: For cardinality estimation. **Bloom Filter**: For membership testing. Choose Reservoir Sampling when you need uniform random samples from an unknown-sized stream. Choose hash-based sampling when determinism and reproducibility matter.
