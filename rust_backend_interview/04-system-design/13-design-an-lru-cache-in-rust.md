# Design an LRU Cache in Rust

## Interview Question

How would you design and implement an LRU (Least Recently Used) cache in Rust?

## Interview Answer

I would implement an LRU cache using a `HashMap<Key, usize>` mapping keys to indices in a `VecDeque` (or a doubly-linked list) that tracks access order. When a key is accessed or inserted, it's moved to the front of the deque. When the cache is full and a new item is inserted, the item at the back of the deque (least recently used) is evicted. For thread safety, I wrap the cache in `Arc<RwLock<LruCache>>` for multi-threaded use cases, or use `tokio::sync::RwLock` for async contexts. The `moka` crate provides a production-ready concurrent LRU cache with TTL support, which is preferable for production systems over a custom implementation. The custom implementation is valuable for understanding ownership patterns and the `Rc<RefCell<T>>` idiom for intrusive doubly-linked lists in Rust.

---

## Follow-up Questions & Answers

### Q1. How do you implement the doubly-linked list in Rust without a garbage collector?

**Interview Answer**

Rust's ownership model makes traditional doubly-linked lists challenging because each node would have two owners (prev and next pointers). I use `Rc<RefCell<Node>>` for single-threaded implementations, where `Rc` provides reference-counted shared ownership and `RefCell` enables interior mutability. Each node holds `Rc<RefCell<Option<Node>>>` for prev and next pointers. For thread-safe versions, I replace `Rc` with `Arc` and `RefCell` with `Mutex`, though this adds locking overhead. An alternative approach that avoids `Rc` entirely is using an arena allocator with indices instead of pointers — the `VecDeque` approach where nodes are stored in a `Vec` and linked by index, which is more idiomatic Rust and avoids the borrow checker issues.

---

### Q2. What is the time complexity of LRU cache operations?

**Interview Answer**

All three core operations — `get`, `put`, and `remove` — are O(1) amortized. `Get` performs a HashMap lookup (O(1) average) and moves the accessed node to the front of the linked list (O(1) since we have a direct pointer/reference). `Put` performs a HashMap insert (O(1)) and appends to the list front (O(1)), with eviction from the list back (O(1)). The amortized O(1) is because `HashMap` resizing is O(n) but happens infently (geometric growth), so per-operation cost is O(1) amortized. In the VecDeque implementation, `push_front` and `pop_back` are O(1) amortized. The `moka` crate achieves O(1) using a sharded lock-free approach with `crossbeam` channel-based eviction.

---

### Q3. How do you handle the ownership challenges of the linked list?

**Interview Answer**

The fundamental challenge is that a doubly-linked list node needs to be referenced from three places: the HashMap value, the previous node's `next` pointer, and the next node's `prev` pointer. In Rust, this violates the single-owner rule. Solutions include: (1) `Rc<RefCell<Node>>` — shared ownership with runtime borrow checking, which panics on concurrent borrows; (2) Arena allocation — store nodes in a `Vec<Node>` and use indices as references, avoiding pointer invalidation entirely; (3) Use `moka` or `lru` crates that handle this internally. For interviews, I prefer the arena approach because it's safe, performant, and idiomatic Rust — no `Rc`/`RefCell` overhead, and the `Vec` provides cache-friendly memory layout.

---

### Q4. How do you make the LRU cache thread-safe?

**Interview Answer**

For a synchronous multi-threaded context, I wrap the cache in `Arc<RwLock<LruCache<K, V>>>`, where `RwLock` allows concurrent reads but exclusive writes. For async contexts (like Axum handlers), I use `tokio::sync::RwLock` which doesn't block the Tokio runtime during waits. However, both approaches have contention under high write loads. A better approach for high-concurrency scenarios is to use `moka::sync::Cache`, which uses lock-free data structures internally (sharded maps with epoch-based reclamation) and achieves near-linear scalability. In an Axum service, I store the cache as `Extension<Arc<moka::sync::Cache<String, Value>>>` and access it in handlers without any explicit locking.

---

### Q5. How do you implement TTL (time-to-live) for cache entries?

**Interview Answer**

I augment each cache entry with a timestamp indicating when it expires. In the custom implementation, I store `(Value, Instant)` tuples in the HashMap. On every `get` call, I check if `Instant::now() > expiry` and treat expired entries as misses. A background task (using `tokio::time::interval`) scans for and removes expired entries periodically to reclaim memory. For `moka`, TTL is built in: `Cache::builder().time_to_live(Duration::from_secs(300)).build()`. The trade-off is that periodic scanning adds overhead — I use a probabilistic scan (checking 10% of entries per interval) to balance cleanup frequency with CPU usage. For exact expiration, I use a `BTreeMap<Instant, Key>` to efficiently find the next expiring entry.

---

### Q6. How do you implement LRU cache with capacity eviction?

**Interview Answer**

When the cache reaches its capacity, inserting a new entry evicts the least recently used item. In the VecDeque implementation, the LRU item is always at the back of the deque. I use `deque.pop_back()` to evict it in O(1) and `HashMap::remove()` to clean up the key mapping. The `moka` cache handles this internally using a TinyLFU admission policy, which is more sophisticated than pure LRU — it admits new entries only if their access frequency exceeds the evicted entry's frequency, achieving higher hit rates. For a custom implementation, I configure capacity in the constructor: `LruCache::new(10_000)` and check `if map.len() >= capacity { evict_lru(); }` before each insert.

---

### Q7. How do you benchmark and optimize the LRU cache?

**Interview Answer**

I benchmark using `criterion` crate with three metrics: throughput (ops/second for get/put), latency (p50/p99 per operation), and memory usage (bytes per cached item). The VecDeque approach benefits from cache-friendly memory layout — nodes are stored contiguously in memory, reducing CPU cache misses. I optimize by using short keys (integer IDs instead of strings) and small values (pointers to data stored elsewhere). For the `moka` cache, I tune `initial_capacity` to avoid rehashing, and configure `num_shards` based on CPU core count. I also profile with `cargo flamegraph` to identify hot paths — typically the HashMap lookup dominates. Using `ahash` instead of the default hasher improves throughput by 20-30% for integer keys.

---

### Q8. How do you handle concurrent access patterns in Axum?

**Interview Answer**

In Axum, the cache is shared across request handlers via `Extension`. I use `Arc<moka::sync::Cache<K, V>>` as the extension type, which is `Clone` and cheap to pass to each handler. Handlers access the cache via `cache.get(&key)` and `cache.insert(key, value)`, both of which are lock-free internally. For read-heavy workloads (90% reads, 10% writes), `moka` achieves near-linear scalability across Tokio worker threads. I also implement a request coalescing pattern using `DashMap<String, JoinHandle<V>>` — when multiple concurrent requests access the same cache key, only one actually queries the database, and others wait on the same `JoinHandle`. This prevents cache stampede on cold keys.

---

### Q9. What are the trade-offs between LRU and other eviction policies?

**Interview Answer**

LRU evicts the least recently accessed item, which works well for recency-biased workloads but performs poorly when there's scan resistance (a one-time full scan pollutes the cache with items accessed once). LFU (Least Frequently Used) addresses this by tracking access frequency, but requires more memory for counters and is slower to update. ARC (Adaptive Replacement Cache) dynamically balances between LRU and LFU, achieving the best hit rates but with more implementation complexity. FIFO is simplest but has the worst hit rate. Clock (second-chance) is a practical approximation of LRU with less overhead. I choose LRU for most applications because it's simple, well-understood, and performs well for typical web workloads with temporal locality.

---

### Q10. How do you implement a distributed LRU cache across multiple nodes?

**Interview Answer**

A true LRU cache across nodes requires global coordination, which is impractical at scale. Instead, I implement a local LRU cache per node (using `moka`) with a distributed cache layer (Redis) underneath. The lookup order is: local LRU → Redis → database. Each Axum instance maintains its own LRU with a capacity of 10,000 entries. Redis provides the shared cache across instances, but Redis itself doesn't implement LRU — it uses allkeys-lru eviction policy at the Redis configuration level. The combination gives us local fast-path performance (sub-microsecond) with shared consistency (Redis sub-millisecond). The cache hit rate improves dramatically: local LRU catches 30-40% of requests, Redis catches another 40-50%, and only 10-20% reach the database.
