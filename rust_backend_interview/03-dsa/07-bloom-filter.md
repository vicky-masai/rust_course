# Bloom Filter

## Interview Question

What is a Bloom Filter and when would you use it in a backend system?

## Interview Answer

A Bloom Filter is a **probabilistic data structure** that tests whether an element is a member of a set. It uses a bit array of `m` bits and `k` independent hash functions. To add an element, hash it with each function and set the corresponding bits to 1. To query, check if all corresponding bits are set — if any is 0, the element is definitely **not** in the set; if all are 1, the element is **probably** in the set (with controllable false positive rate). It uses very little memory compared to exact structures and has **O(k)** insertion and lookup time with **no false negatives**.

---

## Follow-up Questions & Answers

### Q1. What are the false positive and false negative characteristics?

**Interview Answer**

A Bloom Filter has **zero false negatives** — if it says an element is not present, it is definitely not present. It has ** tunable false positives** — it might say an element is present when it isn't. The false positive rate is approximately `(1 - e^(-kn/m))^k` where m is bits, k is hash functions, and n is elements inserted. For example, with m = 10n bits and k = 7 hash functions, the false positive rate is about 1%. There is no way to remove elements from a standard Bloom Filter (that requires a Counting Bloom Filter). The false positive rate increases as more elements are added.

---

### Q2. How would you implement a Bloom Filter in Rust?

**Interview Answer**

```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

struct BloomFilter {
    bits: Vec<u64>,
    k: usize,
    m: usize,
}

impl BloomFilter {
    fn new(m: usize, k: usize) -> Self {
        BloomFilter {
            bits: vec![0; (m + 63) / 64],
            k,
            m,
        }
    }

    fn hash_positions(&self, item: &impl Hash) -> Vec<usize> {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let h1 = hasher.finish() as usize;

        let mut hasher2 = DefaultHasher::new();
        h1.hash(&mut hasher2);
        let h2 = hasher2.finish() as usize;

        (0..self.k).map(|i| (h1.wrapping_add(i.wrapping_mul(h2))) % self.m).collect()
    }

    fn insert(&mut self, item: &impl Hash) {
        for pos in self.hash_positions(item) {
            self.bits[pos / 64] |= 1u64 << (pos % 64);
        }
    }

    fn contains(&self, item: &impl Hash) -> bool {
        self.hash_positions(item).iter().all(|&pos| {
            self.bits[pos / 64] & (1u64 << (pos % 64)) != 0
        })
    }
}
```

The double-hashing technique (`h1 + i*h2`) simulates k independent hash functions with only two hash computations.

---

### Q3. How do you choose the optimal m (bits) and k (hash functions)?

**Interview Answer**

For a desired false positive rate `p` and `n` expected elements: **m = -(n × ln(p)) / (ln(2))²** bits, and **k = (m/n) × ln(2)** hash functions. For example, with n = 1 million and p = 0.1% (0.001): m ≈ 14.4 million bits (≈1.74 MB) and k ≈ 10 hash functions. The formula ensures the filter is optimally sized for the target false positive rate. Using more bits reduces false positives but increases memory; using more hash functions improves accuracy until the filter becomes too full, then it hurts. The `bloomfilter` crate in Rust handles this calculation automatically.

---

### Q4. What are the real-world use cases in backend systems?

**Interview Answer**

Bloom Filters are used extensively: **Redis** supports Bloom Filters via the `RedisBloom` module for membership testing. **Apache Cassandra** and **HBase** use Bloom Filters to avoid unnecessary disk reads — before reading an SSTable, check the Bloom Filter. **Chrome** used Bloom Filters to check URLs against a malicious URL database (6 bytes per URL). **Medium** uses Bloom Filters to recommend articles you haven't seen. **Bitcoin** uses Bloom Filters for lightweight wallet node transaction filtering. **CDNs** use them to avoid caching one-hit-wonders. In Rust backend services, Bloom Filters are ideal for checking API key existence without storing all keys in memory.

---

### Q5. What is the space advantage over other data structures?

**Interview Answer**

A HashSet storing 1 million 64-bit integers uses ~8 MB plus overhead. A Bloom Filter with the same false positive rate (1%) uses ~1.2 MB — **6-7x less memory**. With a 0.1% false positive rate, it uses ~1.74 MB. For membership testing where false positives are acceptable, Bloom Filters provide enormous space savings. The trade-off is that you cannot enumerate elements, cannot remove them (standard Bloom Filter), and must accept a fixed false positive rate. For exact membership testing with memory constraints, alternatives include Cuckoo Filters (support deletion, better cache performance) or Count-Min Sketch (for frequency estimation).

---

### Q6. What is a Counting Bloom Filter and when would you use it?

**Interview Answer**

A Counting Bloom Filter replaces each bit with a counter (typically 4 bits). To add an element, increment counters instead of setting bits. To remove, decrement counters. This supports **deletion** — the standard Bloom Filter does not. However, counters can overflow (if an element is added multiple times), and the memory usage increases 4x. Use a Counting Bloom Filter when you need add/remove operations with acceptable false positives, such as maintaining a dynamic set of recently seen items. In Rust, use `Vec<u8>` for 8-bit counters or `Vec<u32>` for full counters. The `bloomfilter` crate provides both variants.

---

### Q7. How does a Bloom Filter compare to a Cuckoo Filter?

**Interview Answer**

**Cuckoo Filters** support deletion, have better cache performance (fewer hash lookups), and achieve lower false positive rates for the same space. **Bloom Filters** are simpler to implement and analyze. Cuckoo Filters use cuckoo hashing with fingerprint storage — each element stores a small fingerprint, and displacement during insertion moves existing fingerprints. For the same false positive rate, Cuckoo Filters use ~1.05x the space of Bloom Filters but support deletion. In backend systems, Cuckoo Filters are preferred for dynamic sets (items added and removed), while Bloom Filters are preferred for static sets (write-once, read-many). The `cuckoofilter` crate in Rust provides a production-ready implementation.

---

### Q8. Can Bloom Filters be used in distributed systems?

**Interview Answer**

Yes. **Distributed Bloom Filters** are used across multiple nodes for coordinated membership testing. However, synchronizing a shared Bloom Filter across nodes is expensive. Instead, each node maintains a local Bloom Filter, and periodic synchronization merges them. **Cassandra** distributes Bloom Filters per SSTable across nodes. **Redis Bloom** provides a distributed Bloom Filter server. In microservice architectures, a centralized Bloom Filter service (backed by Redis) can be queried by all services. For Rust distributed systems, you can shard Bloom Filters by key — each shard handles a partition of the keyspace, enabling parallel queries.

---

### Q9. What happens as the Bloom Filter approaches capacity?

**Interview Answer**

As more elements are added, more bits are set to 1, and the false positive rate increases. Once the filter reaches about 80% capacity, the false positive rate rises sharply. At 100% capacity (all bits set), every query returns "probably present." The optimal approach is to monitor the fill ratio and create a new Bloom Filter when the false positive rate exceeds the target. Some implementations use **layered Bloom Filters** — when one filter fills, start a new one and query all layers. In Cassandra, Bloom Filters are created per SSTable and rebuilt when the table is rewritten during compaction.

---

### Q10. How do Bloom Filters interact with Rust's type system and performance?

**Interview Answer**

Rust's zero-cost abstractions make Bloom Filters very efficient — the hash computation and bit manipulation compile to tight machine code with no runtime overhead. Using `Hash` trait generics, Bloom Filters work with any hashable type. SIMD instructions can parallelize hash computation across multiple elements. The `bitvec` crate provides optimized bit array operations. For high-performance Bloom Filters in Rust, consider the `bloomfilter` crate which uses SipHash and supports SIMD acceleration. Memory layout matters — using `Vec<u64>` instead of `Vec<bool>` ensures cache-friendly access. In benchmarking, Rust Bloom Filters achieve 50-100 million queries per second on modern hardware.
