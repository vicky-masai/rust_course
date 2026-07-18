# Redis Caching Strategies

## Interview Question

Explain the common caching strategies with Redis, when to use each, and how to implement them in a Rust backend.

## Interview Answer

The four primary caching strategies are Cache-Aside (lazy loading — app reads Redis first, falls back to DB, then populates cache), Write-Through (app writes to cache and DB simultaneously, cache is always up-to-date), Write-Behind/Write-Back (app writes to cache, cache asynchronously flushes to DB — higher performance but risk of data loss), and Read-Through (cache itself fetches from DB on miss, transparent to app). Cache-Aside is the most common and simplest to implement in Rust with `redis-rs`, as the application explicitly controls the read/write flow. TTL (Time-To-Live) is orthogonal and used with any strategy to ensure stale data expires automatically — set via `SET key value EX 300`.

---

## Follow-up Questions & Answers

### Q1. How does Cache-Aside work in practice with Rust?

**Interview Answer**

The pattern: (1) Read from Redis using `GET key`. (2) If hit, deserialize and return. (3) If miss, query PostgreSQL via `sqlx`, serialize the result, write to Redis with `SET key json EX 300`, then return. (4) On write to PostgreSQL, delete the Redis key with `DEL key` to invalidate the cache. In Rust, you wrap this in an async function:

```rust
async fn get_user(con: &mut redis::aio::Connection, pool: &sqlx::PgPool, id: i64) -> Result<User> {
    let key = format!("user:{}", id);
    if let Ok(Some(json)) = con.get::<_, Option<String>>(&key).await {
        return serde_json::from_str(&json).map_err(Into::into);
    }
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool).await?;
    let json = serde_json::to_string(&user)?;
    con.set_ex(&key, &json, 300).await?;
    Ok(user)
}
```

---

### Q2. What is the Thundering Herd problem and how do you solve it?

**Interview Answer**

When a popular cache key expires, many concurrent requests simultaneously hit the database to repopulate it, causing a spike in DB load. Solutions include: (1) Locking — use `SETNX` to ensure only one process repopulates the cache while others wait or return stale data. (2) Probabilistic early expiration — each read randomly refreshes the key before TTL expires, smoothing out expiration times. (3) Background refresh — a separate thread periodically refreshes hot keys before they expire. In Rust, use a Redis-based lock with `redis-rs` to implement the lock approach.

---

### Q3. When would you use Write-Through vs Write-Behind?

**Interview Answer**

Write-Through is safer because every write goes to both cache and DB synchronously, ensuring consistency at the cost of higher write latency. Use it when you cannot tolerate data loss, like user profile updates. Write-Behind decouples writes — the app writes to Redis, and a background worker flushes to the DB asynchronously. It offers higher throughput but risks data loss if Redis crashes before the flush. Use it for analytics events, counters, or activity logs where occasional data loss is acceptable and write throughput is critical.

---

### Q4. How do you handle cache stampede (thundering herd)?

**Interview Answer**

Cache stampede occurs when many requests simultaneously try to rebuild an expired cache key. Solutions: (1) `SETNX`-based locking — only one request populates the cache, others wait with exponential backoff. (2) Probabilistic early recomputation — each `GET` has a small chance of triggering refresh before TTL expires. (3) Stale-while-revalidate — return stale data while a background task refreshes. In Rust with `redis-rs`, implement lock-based stampede protection using `SET key value NX EX 5` as a distributed lock, and have other requests sleep briefly then retry the read.

---

### Q5. How do you implement TTL correctly?

**Interview Answer**

Always set TTL on cached data — use `SET key value EX <seconds>` or `EXPIRE key <seconds>`. For Cache-Aside, set TTL slightly randomized (e.g., `EX 270 + rand(0..30)`) to prevent key expiration synchronization. Use shorter TTLs for rapidly changing data (30s-5min) and longer TTLs for stable data (1hr-24hr). In Rust, compute a jittered TTL:

```rust
use rand::Rng;
let ttl = 300 + rand::thread_rng().gen_range(0..60);
con.set_ex(&key, &json, ttl).await?;
```

This distributes expiration times across keys, reducing stampede risk.

---

### Q6. What is Cache Warming and when do you need it?

**Interview Answer**

Cache warming is the process of pre-populating the cache before it is needed, typically at application startup or during deployment. Without warming, the first users after a restart experience cold-cache latency as every request hits the database. Warming strategies include: (1) Startup script that queries popular keys and populates Redis. (2) Background thread that pre-fetches top-N items. (3) Gradual traffic ramp-up with load balancers. In Rust, run a warming task during `main()` initialization using `tokio::spawn` to populate hot keys before accepting traffic.

---

### Q7. How do you handle cache invalidation on data updates?

**Interview Answer**

The simplest approach is to `DEL` the cache key immediately after a successful database write. For related keys (e.g., a user's profile and their posts), invalidate all related keys in the same transaction boundary. More advanced: use PostgreSQL triggers or logical replication to emit invalidation events that a worker consumes to invalidate Redis keys. The mantra "there are only two hard things in computer science: cache invalidation and naming things" applies — prefer simplicity (DEL after write) over complex invalidation systems unless scale demands it.

---

### Q8. What is a Cache Stampede and how does it differ from Thundering Herd?

**Interview Answer**

They are essentially the same concept — many concurrent requests competing to rebuild a cache key that just expired. The term "Cache Stampede" emphasizes the stampede of requests to the DB, while "Thundering Herd" emphasizes the sudden spike. Both are solved the same way: distributed locking (`SETNX`), probabilistic early expiration, or stale-while-revalidate. In Rust, use `redis-rs` to implement a `SETNX`-based lock where only one request rebuilds the cache and others return stale data or wait.

---

### Q9. How do you choose the right TTL for different data types?

**Interview Answer**

Session tokens: match session duration (e.g., `EX 86400` for 24 hours). User profiles: 5-15 minutes (moderate staleness tolerance). Product catalog: 1-24 hours (rarely changes). Real-time data (stock prices): 1-5 seconds or no caching. Search results: 30-60 seconds. Rate limit counters: match the rate window (e.g., `EX 60` for per-minute limits). The general principle: shorter TTL for data that changes frequently or where staleness has high cost; longer TTL for stable data where cache misses are expensive.

---

### Q10. How do you implement Cache-Aside with Rust error handling?

**Interview Answer**

Handle Redis failures gracefully — if Redis is down, fall back to PostgreSQL directly (cache miss behavior). Use `match` on Redis `GET` results: `Ok(Some(v))` = hit, `Ok(None)` = miss, `Err(e)` = Redis failure (log the error, query DB directly). Never let a Redis failure cascade into application failure. In Rust:

```rust
match con.get::<_, Option<String>>(&key).await {
    Ok(Some(json)) => return Ok(serde_json::from_str(&json)?),
    Ok(None) => {},
    Err(e) => tracing::warn!("Redis miss: {}", e),
}
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    .fetch_one(pool).await?;
con.set_ex(&key, serde_json::to_string(&user)?, 300).await.ok();
Ok(user)
```

The `.ok()` on `set_ex` ensures a cache-write failure doesn't fail the request.

