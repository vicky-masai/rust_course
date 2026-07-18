# Redis Memory Management

## Interview Question

How does Redis manage memory, what eviction policies are available, and how do you optimize memory usage in production?

## Interview Answer

Redis stores all data in RAM and uses several internal optimizations: compact encodings (ziplist/listpack for small collections), memory allocators (jemalloc by default), and key-level TTL expiration. When `maxmemory` is reached, Redis evicts keys based on the configured eviction policy — `noeviction` (return errors on writes), `allkeys-lru` (evict least recently used keys globally), `volatile-lru` (evict LRU keys with TTL only), `allkeys-lfu` (evict least frequently used), `volatile-ttl` (evict keys with shortest TTL), and `random`. Monitor memory with `INFO memory` and use `MEMORY USAGE key` to audit individual keys. In Rust, set `maxmemory` via config and handle `OOM` errors gracefully.

---

## Follow-up Questions & Answers

### Q1. What is the difference between LRU and LFU eviction?

**Interview Answer**

LRU (Least Recently Used) evicts the key that was accessed longest ago, based on approximate LRU tracking via a sampling algorithm (checks random keys and evicts the oldest). LFU (Least Frequently Used) evicts the key with the lowest access frequency, using a probabilistic counter that decays over time. LFU is better for workloads with clear hot/cold data patterns — it keeps frequently accessed keys even if they were last accessed slightly earlier than rarely-used keys. LRU is simpler and works well for general-purpose caching. Redis 4.0+ supports both via `maxmemory-policy`.

---

### Q2. How do you set and monitor `maxmemory`?

**Interview Answer**

Set `maxmemory` in `redis.conf` or at runtime:

```
CONFIG SET maxmemory 4gb
```

When the limit is reached, Redis applies the eviction policy. Monitor with:

```
INFO memory
```

Key fields: `used_memory` (total allocated), `used_memory_rss` (OS-level RSS), `mem_fragmentation_ratio` (RSS/used_memory — values >1.5 indicate fragmentation), `maxmemory` (configured limit), `maxmemory_policy` (active eviction). In Rust, periodically run `redis::cmd("INFO").arg("memory")` and parse the response to track usage.

---

### Q3. What is memory fragmentation in Redis and how do you handle it?

**Interview Answer**

Fragmentation occurs when Redis's memory allocator (jemalloc) leaves gaps between allocated blocks due to varying object sizes. A `mem_fragmentation_ratio` >1.5 indicates significant fragmentation. Solutions: (1) Restart Redis (resets allocator). (2) Enable `activedefrag yes` (Redis 4.0+) for online defragmentation. (3) Use `MEMORY PURGE` to force jemalloc to release unused pages. (4) Keep key sizes uniform to reduce fragmentation. Monitor `used_memory` vs `used_memory_rss` — the difference is fragmentation overhead.

---

### Q4. How do you optimize Redis memory usage in production?

**Interview Answer**

Strategies: (1) Use hash encoding for small objects instead of multiple string keys (`HSET user:123 field value` instead of `SET user:123:field value`). (2) Set TTLs on all keys to prevent unbounded growth. (3) Use `redis-rs` to compress large values before storing (e.g., `bincode` + `zstd`). (4) Monitor `MEMORY USAGE key` for individual keys and set `maxmemory` conservatively. (5) Use `SCAN` instead of `KEYS` to avoid memory spikes. (6) Avoid storing large strings (>100KB) — split into hashes or use external storage. (7) Use `OBJECT ENCODING key` to verify compact encodings are active.

---

### Q5. What is `noeviction` and when should you use it?

**Interview Answer**

`noeviction` means Redis returns `OOM` errors for write commands when `maxmemory` is reached, while reads continue normally. Use it when data correctness is critical and you cannot afford to lose any cached data — the application must handle `OOM` errors by either rejecting the write or pushing to a fallback queue. It is the default policy. For caching use cases where occasional eviction is acceptable, use `allkeys-lru` instead. In Rust, handle `OOM` errors by catching `RedisError` with kind `OutOfMemory`.

---

### Q6. How does Redis track key access for LRU/LFU?

**Interview Answer**

Redis does not track every access (too expensive). For LRU, it uses an approximate algorithm: when evaluating keys for eviction, it samples a fixed number of keys (default 5, configurable via `maxmemory-samples`) and evicts the oldest among the sample. For LFU, it uses a Morris counter (probabilistic frequency counter) that increments on access and decays over time (configurable via `lfu-decay-time`). This approximation is efficient and accurate enough for eviction decisions. The tradeoff is that eviction is not globally optimal but is statistically effective.

---

### Q7. How do you choose the right eviction policy?

**Interview Answer**

`allkeys-lru`: general-purpose caching where all keys are equally important. `volatile-lru`: caching with TTLs where you want to keep non-expiring keys (like config). `allkeys-lfu`: caching with clear hot/cold patterns (e.g., popular products vs. rarely viewed). `volatile-ttl`: prefer evicking keys closest to expiration. `allkeys-random`: no access pattern, simple eviction. `noeviction`: data must not be evicted (queue, lock storage). Default: `noeviction`. Most caching use cases use `allkeys-lru` or `allkeys-lfu`.

---

### Q8. What happens when Redis exceeds `maxmemory` with `noeviction`?

**Interview Answer**

Write commands (`SET`, `HSET`, `LPUSH`, `INCR`, etc.) return `-OOM command not allowed` errors. Read commands (`GET`, `HGET`, etc.) continue working normally. `DEL` and `EXPIRE` also continue working (you can still free memory). This protects Redis from crashing due to out-of-memory, but the application must handle OOM errors — in Rust, catch the error and either reject the request or fall back to a different storage. Monitor `maxmemory` usage to trigger alerts before hitting the limit.

---

### Q9. How do you profile Redis memory usage per key?

**Interview Answer**

Use `MEMORY USAGE key` to get the memory footprint of a specific key in bytes, including overhead for the key metadata and encoding. Use `SCAN` to iterate all keys and check each:

```rust
let mut cursor = 0;
loop {
    let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
        .arg(cursor).arg("COUNT").arg(100)
        .query_async(&mut con).await?;
    for key in &keys {
        let size: i64 = redis::cmd("MEMORY").arg("USAGE").arg(key)
            .query_async(&mut con).await.unwrap_or(0);
        println!("{}: {} bytes", key, size);
    }
    cursor = new_cursor;
    if cursor == 0 { break; }
}
```

Use `MEMORY DOCTOR` for automated analysis of memory issues.

---

### Q10. How do you handle Redis memory in a Kubernetes environment?

**Interview Answer**

Set `maxmemory` to ~70-80% of the container's memory limit to leave headroom for fork overhead and fragmentation. Use `container_memory_bytes` metric from cAdvisor/Kubelet to monitor actual usage. Configure Pod resource limits and requests based on Redis memory needs. Use `redis-exporter` for Prometheus metrics and alert on `redis_memory_used_bytes` approaching `maxmemory`. For persistence, ensure the PersistentVolume has enough space for RDB/AOF files. Consider `memory-huge-pages no` to avoid huge page defragmentation issues in containers.

