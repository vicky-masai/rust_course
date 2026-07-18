# Why Redis?

## Interview Question

Why should you use Redis in a backend application, and what problems does it solve?

## Interview Answer

Redis is an in-memory data store that solves critical performance bottlenecks by keeping frequently accessed data close to the application, eliminating repeated database round-trips. It sub-millisecond latency because all data lives in RAM, which is orders of magnitude faster than disk-based databases. Beyond caching, Redis provides rich data structures, atomic operations, pub/sub messaging, and built-in expiration — making it a versatile tool for sessions, rate limiting, distributed locks, real-time leaderboards, and job queues. In a Rust backend, the `redis-rs` crate gives you async/await support with Tokio, connection pooling, and pipeline batching out of the box.

---

## Follow-up Questions & Answers

### Q1. What is the difference between Redis and a traditional in-memory cache like Memcached?

**Interview Answer**

Memcached is a simple key-value cache with string-only values and no built-in data structures, while Redis supports strings, lists, sets, sorted sets, hashes, HyperLogLogs, and more. Redis also supports persistence (RDB snapshots and AOF), pub/sub messaging, Lua scripting, and atomic transactions — none of which Memcached offers. Memcached uses a multi-threaded architecture, whereas Redis is single-threaded but uses I/O multiplexing for high throughput. For most new projects, Redis is the default choice due to its richer feature set.

---

### Q2. When would you choose NOT to use Redis?

**Interview Answer**

Redis is not ideal when your data exceeds available RAM, since everything must fit in memory, or when you need complex relational queries with JOINs and ACID transactions across multiple tables. If your access pattern is purely sequential writes with no read-heavy hot path — like append-only audit logs — a disk-based store may be more cost-effective. Also, if you need guaranteed durability without tuning persistence options, PostgreSQL or similar databases are safer defaults.

---

### Q3. How does Redis achieve sub-millisecond latency?

**Interview Answer**

Redis stores all data in RAM, which eliminates disk I/O for reads. It uses a single-threaded event loop with I/O multiplexing (epoll/kqueue) so there is no thread contention or locking overhead for command execution. Data is organized in efficient C structs and hash tables with O(1) or O(log N) lookups. Network latency between app and Redis is often the bottleneck, not Redis itself — which is why deploying Redis on the same host or availability zone matters.

---

### Q4. What are the main use cases for Redis in a Rust backend?

**Interview Answer**

Common use cases include caching database query results, storing user session tokens, implementing rate limiters with sliding window counters, acquiring distributed locks for critical sections, pub/sub for real-time notifications, and job/task queues using Redis lists. In Rust, you would use `redis-rs` with `tokio` for async operations, connection pooling via `RedisConnectionManager`, and pipelines for batching commands to reduce round-trips.

---

### Q5. Is Redis single-threaded? How does it handle concurrency?

**Interview Answer**

Redis command execution is single-threaded, meaning one command runs at a time with no preemption, which eliminates race conditions within Redis itself. However, Redis uses I/O threads (configurable via `io-threads`) to handle network reads and writes in parallel, and it uses non-blocking I/O multiplexing internally. This design gives Redis very high throughput (100k+ ops/sec) without the complexity of multi-threaded locking. Clients can send commands concurrently, but Redis serializes their execution.

---

### Q6. What is the `redis-rs` crate and why use it?

**Interview Answer**

`redis-rs` is the most popular Rust client for Redis, available on crates.io. It provides both synchronous and async (via the `tokio-comp` feature) APIs, supports all Redis data types, pub/sub, streams, pipelines, Lua scripting, and cluster mode. It also integrates with `tokio` for non-blocking I/O, supports `RedisConnectionManager` for connection pooling, and implements `serde` integration for automatic struct serialization. It is the de facto standard for Rust-Redis interaction.

---

### Q7. What is the memory footprint tradeoff of using Redis?

**Interview Answer**

Since Redis holds all data in RAM, storing 1 GB of data in Redis costs ~1 GB of memory, which is more expensive per GB than SSD storage. Redis does support optional compression for values (via `rdb-compression` or application-level compression) and uses efficient encodings like ziplist and listpack for small collections. You should set `maxmemory` and configure an eviction policy to prevent Redis from consuming all available RAM and crashing the host.

---

### Q8. How do you decide between Redis and Memcached in 2025?

**Interview Answer**

Choose Redis if you need data structures beyond simple strings, persistence, pub/sub, Lua scripting, or cluster mode — which covers most modern applications. Choose Memcached only if you need a very simple, multithreaded key-value cache with minimal operational overhead and you will never need richer features. In practice, Redis has essentially replaced Memcached as the default choice in most tech stacks due to its versatility and ecosystem support.

---

### Q9. Can Redis be used as a primary database?

**Interview Answer**

Redis can technically serve as a primary store for specific workloads like session management, real-time leaderboards, or feature flags, especially with Redis Streams and RediSearch for indexing. However, it is not designed to replace a relational database for complex queries, joins, or guaranteed durability. The recommended pattern is to use Redis as a cache or fast-access layer in front of PostgreSQL or another durable database, not as the sole source of truth for critical business data.

---

### Q10. What happens if Redis crashes — do you lose data?

**Interview Answer**

By default, Redis uses RDB snapshots, which save data to disk at intervals (e.g., every 5 minutes), so you could lose up to that interval of data. You can enable AOF (Append-Only File) persistence for near-zero data loss at the cost of write performance. In a caching-only use case, data loss is acceptable since the cache is rebuilt from the database. For durability-critical use cases like distributed locks or queues, you must configure AOF with `appendfsync everysec` or `always` and monitor disk latency.

