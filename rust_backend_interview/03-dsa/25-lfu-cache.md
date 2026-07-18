# LFU Cache

## Interview Question

Design and implement an LFU (Least Frequently Used) Cache that supports `get` and `put` operations.

## Interview Answer

An LFU Cache evicts the least frequently accessed item when at capacity. Unlike LRU (which evicts based on recency), LFU tracks access frequency. The implementation uses: a **HashMap of key → node** for O(1) lookup, a **HashMap of frequency → doubly linked list** of nodes with that frequency, and a **min_frequency** tracker to quickly find the least frequent bucket. On `get` or `put`, the node's frequency increases by 1 — it moves from its current frequency list to the next one. When evicting, remove the LRU item from the min_frequency list. This gives O(1) for both `get` and `put`.

**Time Complexity**: O(1) for get and put
**Space Complexity**: O(capacity)

---

## Follow-up Questions & Answers

### Q1. What is the difference between LFU and LRU eviction?

**Interview Answer**

**LRU** (Least Recently Used) evicts the item that was accessed longest ago. It favors recency — an item accessed once recently is preferred over an item accessed 100 times but not recently. **LFU** (Least Frequently Used) evicts the item with the lowest access count. It favors frequency — an item accessed 100 times is kept even if not accessed recently. LRU is simpler and works well for most caching workloads. LFU is better for workloads with clear hot/cold data separation (e.g., popular products vs long-tail items). LFU suffers from "frequency pollution" — an item accessed many times in the past but no longer relevant retains high frequency. Redis supports both via `allkeys-lru`/`allkeys-lfu`.

---

### Q2. How would you implement LFU Cache in Rust?

**Interview Answer**

```rust
use std::collections::{HashMap, VecDeque};

struct LFUCache {
    capacity: usize,
    min_freq: usize,
    key_to_val: HashMap<i32, i32>,
    key_to_freq: HashMap<i32, usize>,
    freq_to_keys: HashMap<usize, VecDeque<i32>>,
}

impl LFUCache {
    fn new(capacity: usize) -> Self {
        LFUCache {
            capacity,
            min_freq: 0,
            key_to_val: HashMap::new(),
            key_to_freq: HashMap::new(),
            freq_to_keys: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.key_to_val.contains_key(&key) {
            return -1;
        }
        let freq = *self.key_to_freq.get(&key).unwrap();
        self.update_freq(key, freq);
        *self.key_to_val.get(&key).unwrap()
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 { return; }

        if self.key_to_val.contains_key(&key) {
            let freq = *self.key_to_freq.get(&key).unwrap();
            self.key_to_val.insert(key, value);
            self.update_freq(key, freq);
            return;
        }

        if self.key_to_val.len() >= self.capacity {
            // evict LRU from min_freq bucket
            let evict_key = self.freq_to_keys.get_mut(&self.min_freq)
                .unwrap().pop_front().unwrap();
            self.key_to_val.remove(&evict_key);
            self.key_to_freq.remove(&evict_key);
        }

        self.key_to_val.insert(key, value);
        self.key_to_freq.insert(key, 1);
        self.freq_to_keys.entry(1).or_default().push_back(key);
        self.min_freq = 1;
    }

    fn update_freq(&mut self, key: i32, old_freq: usize) {
        let keys = self.freq_to_keys.get_mut(&old_freq).unwrap();
        keys.retain(|&k| k != key);
        if keys.is_empty() {
            self.freq_to_keys.remove(&old_freq);
            if self.min_freq == old_freq {
                self.min_freq = old_freq + 1;
            }
        }
        let new_freq = old_freq + 1;
        self.key_to_freq.insert(key, new_freq);
        self.freq_to_keys.entry(new_freq).or_default().push_back(key);
    }
}
```

---

### Q3. How does LFU handle ties (items with the same frequency)?

**Interview Answer**

When multiple items share the same minimum frequency, LFU evicts the **least recently used** among them — combining frequency and recency. This is implemented by using a doubly linked list (or VecDeque) for each frequency bucket, maintaining insertion/access order. The LRU item is at the front of the list; the MRU item is at the back. When evicting, remove from the front. This tie-breaking rule is critical: without it, items with equal frequency would be evicted arbitrarily, leading to poor cache performance. In Redis's LFU implementation, the tie is broken by the idle time — the item with the longest idle time is evicted first.

---

### Q4. What are the real-world use cases of LFU Cache?

**Interview Answer**

**CDN caching**: Popular content (viral videos, trending articles) is accessed frequently and should be cached. LFU naturally keeps popular content while evicting rarely accessed files. **Database buffer pools**: Frequently queried pages should stay in memory. PostgreSQL's `pg_stat_user_tables` tracks access frequency for cache management. **DNS caching**: Popular domains (google.com, facebook.com) are queried millions of times; LFU keeps them cached. **Web browser caching**: Frequently visited pages benefit from LFU eviction. **Redis**: Supports LFU via `maxmemory-policy allkeys-lfu` (introduced in Redis 4.0). In Rust backend services, LFU is ideal for API response caching where some endpoints are hit far more than others.

---

### Q5. What is the "frequency stale" problem in LFU?

**Interview Answer**

LFU has a staleness problem: an item accessed 1000 times yesterday but never again today still has frequency 1000, while a new item accessed 5 times today has frequency 5. The stale item won't be evicted because its frequency is higher. Solutions: **Time-decay LFU**: multiply all frequencies by a decay factor periodically (e.g., halve every hour). **LFU with aging**: cap maximum frequency or use logarithmic frequency. **Redis's LFU implementation**: uses a probabilistic counter that increments with probability `1/counter_value`, effectively creating logarithmic frequency and natural decay. In Rust, implement time-decay by storing timestamps and computing effective frequency as `count × decay_factor^(now - last_access)`.

---

### Q6. How would you implement time-decay LFU in Rust?

**Interview Answer**

```rust
use std::collections::{HashMap, BTreeMap};
use std::time::{Instant, Duration};

struct TimeDecayLFU {
    capacity: usize,
    entries: HashMap<i32, (i32, Instant)>,   // key -> (value, last_access)
    frequencies: HashMap<i32, f64>,           // key -> effective frequency
    decay_rate: f64,                           // per second
}

impl TimeDecayLFU {
    fn effective_freq(&self, key: &i32) -> f64 {
        let count = self.frequencies.get(key).unwrap_or(&0.0);
        let last = self.entries.get(key).map(|(_, t)| *t).unwrap_or(Instant::now());
        let elapsed = last.elapsed().as_secs_f64();
        count * (-self.decay_rate * elapsed).exp()
    }

    fn evict(&mut self) {
        if let Some((evict_key, _)) = self.entries.keys()
            .min_by(|a, b| {
                self.effective_freq(a).partial_cmp(&self.effective_freq(b))
                    .unwrap()
            }) {
            let evict_key = *evict_key;
            self.entries.remove(&evict_key);
            self.frequencies.remove(&evict_key);
        }
    }
}
```

The exponential decay naturally reduces old frequencies, allowing new hot items to replace stale ones.

---

### Q7. What is the space overhead of LFU vs LRU?

**Interview Answer**

**LRU**: HashMap (key → node) + doubly linked list. Space: O(n) with moderate constant factor (each node stores key, value, prev, next pointers). **LFU**: HashMap (key → node) + HashMap (key → freq) + HashMap (freq → list). Space: O(n) with higher constant factor — LFU needs at least 3 HashMaps vs LRU's 2. In Rust, each HashMap has ~56 bytes overhead plus heap allocation. For a cache with capacity 1M entries: LRU ≈ 64MB, LFU ≈ 96MB. The extra space buys better eviction decisions for workloads with clear frequency patterns. For memory-constrained systems, consider LFU-LRU hybrid (evict by frequency, break ties by recency).

---

### Q8. How is LFU implemented in Redis?

**Interview Answer**

Redis's LFU (introduced in Redis 4.0) uses a **Morris counter** — a probabilistic counter that increments with decreasing probability. The counter uses 8 bits (0-255). The increment probability is `1/counter_value × LFU_log_factor`. This creates logarithmic frequency: doubling the counter value roughly doubles the true access count. The counter decays over time using `lfu-decay-time` (default 1 minute) — the counter value decreases by 1 for each `lfu-decay-time` period of inactivity. For eviction, Redis selects the key with the lowest LFU counter, breaking ties by idle time (`idle` parameter in `OBJECT IDLETIME`). This implementation uses only 8 bits per key for frequency, making it very memory-efficient.

---

### Q9. Can LFU be combined with other eviction policies?

**Interview Answer**

Yes, hybrid policies combine LFU with other strategies: **LFU-LRU**: Use frequency as primary, recency as tiebreaker (Redis's approach). **ARC** (Adaptive Replacement Cache): Dynamically balances between LRU-like and LFU-like behavior based on workload. **LIRS** (Low Inter-reference Recency Set): Uses both recency and frequency to identify cache-resistant entries. **W-TinyLFU** (used in Caffeine, Java): Combines a small LRU window with a large LFU probation/protection structure. The Window TinyLFU achieves near-optimal hit rates by using a Count-Min Sketch to estimate frequency cheaply. In Rust, the `moka` crate implements a W-TinyLFU-inspired concurrent cache.

---

### Q10. What is the time complexity breakdown for each LFU operation?

**Interview Answer**

**get(key)**: O(1) amortized — HashMap lookup O(1), move node between frequency lists O(1) (remove from old list, append to new list). **put(key, value)**: O(1) amortized — same as get for existing keys. For new keys: O(1) HashMap insert, O(1) list append. Eviction: O(1) list pop + O(1) HashMap remove. **update_freq**: O(1) amortized — remove from VecDeque (O(n) worst case with `retain`, O(1) with doubly linked list), append to new list O(1). **Finding min_freq**: O(1) — track it during updates. The VecDeque `retain` in my Rust implementation is O(n) — a proper doubly linked list would make it O(1). For production, use an intrusive doubly linked list or the `lru`/`moka` crates which provide O(1) LFU implementations.
