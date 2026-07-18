# Caching Strategy

## Interview Question

Caching Strategy.

## Interview Answer

"I cache frequently read data in Redis and invalidate the cache after updates."

---

## Follow-up Questions & Answers

### Q1. What are the common caching strategies and when do you use each?

**Interview Answer**

Cache-aside loads data into the cache on first access and invalidates on writes. Write-through writes to cache and database simultaneously for consistency. Write-behind writes to cache first and asynchronously persists to the database for speed. I primarily use cache-aside for read-heavy APIs because it's simple, handles misses gracefully, and gives me full control over invalidation.

---

### Q2. How do you handle cache invalidation in a backend system?

**Interview Answer**

I invalidate cache entries when the underlying data changes, using a write-through or event-driven approach. For simple cases, I delete the cache key after a successful database write. For distributed systems, I publish invalidation events through a message broker so all instances clear their local caches. The hardest part of caching is invalidation, so I keep the logic centralized in the service layer.

---

### Q3. What is a cache stampede and how do you prevent it?

**Interview Answer**

A cache stampede happens when a popular cache key expires and many requests simultaneously hit the database to rebuild it. I prevent this using distributed locks with Redis `SETNX` so only one request rebuilds the cache while others wait or return stale data. I also use background refresh with Tokio tasks to proactively refresh keys before they expire.

---

### Q4. How do you implement Redis caching with an Axum backend?

**Interview Answer**

I use the `redis` crate with `RedisPool` stored in Axum's `State`. Before querying the database, I check Redis with `GET`. If the key exists, I deserialize and return it. On a cache miss, I query PostgreSQL, serialize the result, and store it in Redis with an expiration using `SETEX`. I wrap this in a generic helper function to avoid repeating the pattern across handlers.

---

### Q5. What cache eviction policies do you know and which do you prefer?

**Interview Answer**

LRU evicts the least recently used keys, LFU evicts the least frequently used, and TTL-based expiry removes keys after a fixed duration. Redis supports all three through its maxmemory policies and per-key TTLs. I use TTL-based expiry as the primary strategy with LRU as a fallback for memory pressure, since it's predictable and easy to reason about in API caching.

---

### Q6. When would you NOT use caching?

**Interview Answer**

I skip caching for data that changes frequently because the invalidation overhead outweighs the read benefit. I also avoid caching for strong consistency requirements like financial balances where stale data is unacceptable. For write-heavy workloads, caching adds complexity without proportional gains, so I optimize the database layer with proper indexing and connection pooling instead.

---

### Q7. How do you handle cache warming after a deployment?

**Interview Answer**

After a deployment, all in-memory caches are empty and Redis keys may have expired, causing a cold start performance hit. I run a background cache warming process on startup that pre-loads the most frequently accessed data into Redis. For critical endpoints, I implement a health check that verifies cache readiness before the instance starts accepting traffic.

---

### Q8. What is the difference between local in-memory caching and Redis caching?

**Interview Answer**

In-memory caching with something like `moka` is extremely fast since there's no network overhead, but it's per-instance so different servers may have different cached values. Redis is a shared cache accessible by all instances, providing consistency across the cluster. I use local caching for hot data that rarely changes and Redis for shared state that needs to be consistent across all backend instances.

---
