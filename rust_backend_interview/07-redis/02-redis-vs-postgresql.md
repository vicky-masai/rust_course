# Redis vs PostgreSQL

## Interview Question

What are the key differences between Redis and PostgreSQL, and when would you use each?

## Interview Answer

Redis is an in-memory key-value store optimized for speed and simple access patterns, while PostgreSQL is a disk-based relational database optimized for complex queries, ACID transactions, and data integrity. Redis stores data in RAM with optional persistence, giving sub-millisecond reads, whereas PostgreSQL writes to WAL and data files with transactional guarantees but higher latency. Use Redis for caching, session storage, rate limiting, and real-time counters; use PostgreSQL as the authoritative data store for business logic that requires JOINs, constraints, and complex aggregations. In a typical Rust backend, Redis sits in front of PostgreSQL as a cache layer, with the database as the source of truth.

---

## Follow-up Questions & Answers

### Q1. Can Redis and PostgreSQL replace each other?

**Interview Answer**

No — they serve fundamentally different purposes. Redis cannot replace PostgreSQL for complex relational queries, multi-table JOINs, or strong ACID guarantees across transactions. PostgreSQL cannot replace Redis for sub-millisecond caching, real-time counters, or pub/sub messaging. The idiomatic architecture uses both: PostgreSQL as the durable source of truth and Redis as a fast access layer in front of it, with cache invalidation when data changes.

---

### Q2. How do you handle cache invalidation between Redis and PostgreSQL?

**Interview Answer**

The most common strategies are TTL-based expiration, write-through invalidation, and event-driven invalidation. With TTL, you set `EX` on Redis keys so stale data expires automatically (e.g., `SET user:123 {...} EX 300`). Write-through invalidation deletes the Redis key immediately after a PostgreSQL write using `DEL user:123`. For event-driven invalidation, you can use PostgreSQL logical replication or a message queue to notify the app to invalidate relevant keys. The `Cache-aside` pattern is the most common: read from Redis first, fall back to PostgreSQL on miss, then populate Redis.

---

### Q3. What is the data model difference?

**Interview Answer**

PostgreSQL uses tables with rows and columns, enforcing schemas with constraints (NOT NULL, UNIQUE, CHECK) and supporting complex types like arrays, JSONB, and ranges. Redis supports strings, lists, sets, sorted sets, hashes, streams, and HyperLogLogs — all accessible via simple commands like `SET`, `HGET`, `ZADD`, and `LPUSH`. Redis has no schema enforcement at the database level; the application is responsible for data structure. PostgreSQL joins data across tables; Redis stores denormalized data in a single key.

---

### Q4. How do transactions differ?

**Interview Answer**

PostgreSQL provides full ACID transactions with `BEGIN`, `COMMIT`, `ROLLBACK`, row-level locks, and isolation levels (READ COMMITTED, REPEATABLE READ, SERIALIZABLE). Redis transactions via `MULTI`/`EXEC` guarantee atomic execution of a command block (no command interleaves) but do not support rollback or conditional logic — if one command in a `MULTI` block fails, others still execute. For atomic operations requiring rollback in Redis, you use Lua scripting. Redis transactions are fundamentally simpler than PostgreSQL's.

---

### Q5. What about performance benchmarks?

**Interview Answer**

Redis can handle 100k-1M+ operations per second on a single instance for simple key lookups, with latency under 1ms. PostgreSQL typically handles 5k-20k complex queries per second depending on query complexity, indexes, and hardware. These numbers are not directly comparable since they measure different things — Redis does O(1) key lookups while PostgreSQL executes relational queries. The practical takeaway: use Redis to absorb read traffic that would otherwise hit PostgreSQL, reducing database load by 80-95% in read-heavy applications.

---

### Q6. How do you serialize data between Rust, Redis, and PostgreSQL?

**Interview Answer**

For Redis with `redis-rs`, use the `serde` feature to serialize Rust structs to JSON strings via `serde_json` and store them with `redis::cmd("SET").arg(key).arg(json_string)`. For PostgreSQL with `sqlx`, derive `FromRow` and use query macros with bind parameters. A common pattern is to serialize the same struct to both formats: JSON for Redis and parameterized inserts for PostgreSQL. You can also use `redis-rs`'s built-in hash serialization with `redis::from_json` for structured Redis hashes.

---

### Q7. When should you NOT use Redis alongside PostgreSQL?

**Interview Answer**

If your application is read-light or write-heavy with no hot data path, adding Redis introduces unnecessary operational complexity and potential consistency bugs. If your data is write-once and rarely read (like audit logs), PostgreSQL alone is sufficient. If your dataset is larger than available RAM and you cannot partition it, Redis becomes impractical since all data must fit in memory. Also, if your team lacks experience with cache invalidation patterns, a misconfigured Redis cache can serve stale data and cause subtle bugs.

---

### Q8. How does Redis handle persistence compared to PostgreSQL's WAL?

**Interview Answer**

PostgreSQL uses Write-Ahead Logging (WAL) to guarantee durability: every change is written to WAL before the data page, enabling crash recovery and point-in-time recovery. Redis offers two persistence modes: RDB snapshots (periodic full dumps, fast restore but potential data loss between snapshots) and AOF (append-only file logging every write, slower restore but near-zero data loss). You can combine both for a hybrid approach. Neither Redis persistence mode matches PostgreSQL's durability guarantees, which is why Redis is a cache layer, not a primary store.

---

### Q9. How do you handle failover for each?

**Interview Answer**

PostgreSQL supports streaming replication with a primary-replica setup, automatic failover via Patroni or pg_auto_failover, and synchronous replication for zero data loss. Redis supports Sentinel for automatic failover with master-slave replication, and Redis Cluster for horizontal sharding with automatic slot migration. In both cases, the Rust application should handle connection retries and reconnection logic — `redis-rs`'s `RedisConnectionManager` handles this transparently, while `sqlx` handles PostgreSQL reconnection via its pool.

---

### Q10. What is the recommended architecture for using both?

**Interview Answer**

The standard architecture is: PostgreSQL as the primary durable store, Redis as a caching and fast-access layer in front of it, with the application implementing cache-aside or write-through patterns. Writes go to PostgreSQL first, then invalidate or update Redis. Reads check Redis first, falling back to PostgreSQL on cache miss. Use Redis for hot data (frequently read, rarely written) and PostgreSQL for cold data (complex queries, archival). This pattern is well-supported by `redis-rs` and `sqlx` in Rust, with connection pooling managed by each library independently.

