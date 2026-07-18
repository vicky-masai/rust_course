# database sharding

## Interview Question

Explain database sharding.

## Interview Answer

Split data across multiple databases by shard key such as tenant ID or user ID.

---

## Follow-up Questions & Answers

### Q1. What are common shard key strategies?

**Interview Answer**

Hash-based sharding distributes data uniformly using `hash(key) % num_shards`. Range-based sharding partitions by value ranges (e.g., A-M on shard 1, N-Z on shard 2). Directory-based sharding uses a lookup table. Hash-based is simplest and most common; range-based supports range queries but can cause hotspots.

---

### Q2. How do you handle cross-shard queries?

**Interview Answer**

Cross-shard queries (joins across shards) are expensive—scatter the query to all shards and merge results in the application layer. Avoid them by denormalizing data so related data lives on the same shard. For aggregations, use map-reduce patterns: compute partial results on each shard, then combine. Some proxies like Vitess handle this transparently.

---

### Q3. How do you choose the number of shards?

**Interview Answer**

Start with more shards than you think you need—splitting shards later is expensive (requires data migration). A common approach: plan for 2-3 years of growth, shard count should be a power of 2 for even distribution. If each shard holds ~10GB-50GB of data, you have room to grow. Monitor shard sizes and split hot shards.

---

### Q4. What is resharding and how do you do it online?

**Interview Answer**

Resharding redistributes data when you add/remove shards. Online resharding uses a dual-write approach: write to both old and new shards during migration, backfill historical data, then cut over. Tools like Vitess automate this. In Rust, implement a routing layer that abstracts shard selection, making resharding transparent to application code.

---

### Q5. How do you implement shard routing in Rust?

**Interview Answer**

Create a `ShardRouter` that maps shard keys to database connections. Use a `HashMap<u64, PgPool>` for shard-to-pool mapping. The routing function: `fn route(&self, key: &str) -> &PgPool { self.pools[hash(key) % self.pools.len()] }`. Wrap this in an Axum extension or middleware for transparent access in handlers.

---

### Q6. What is a hot shard and how do you mitigate it?

**Interview Answer**

A hot shard receives disproportionate traffic due to non-uniform key distribution (e.g., a celebrity user). Mitigations include: splitting the hot shard, adding read replicas, caching hot data in Redis, or using consistent hashing to redistribute. Monitor per-shard query latency and connection count to detect hotspots early.

---

### Q7. How do foreign keys work across shards?

**Interview Answer**

Foreign keys don't work across shards because referential integrity can't be enforced. You must handle this at the application level. Strategies include: co-locating related data on the same shard (e.g., shard by user ID so all user data is together), using application-level joins, or replacing foreign keys with eventual consistency.

---

### Q8. What is consistent hashing and why is it useful for sharding?

**Interview Answer**

Consistent hashing maps keys to a ring of positions, and shards own segments of the ring. When you add/remove a shard, only neighboring keys need to move (not all keys). This minimizes data migration during resharding. The `hashring` crate provides this in Rust. It's essential for distributed caches and databases that need elastic scaling.

---

### Q9. How do transactions work across shards?

**Interview Answer**

Distributed transactions across shards require two-phase commit (2PC), which is complex and slow. Most systems avoid them by designing shard boundaries so transactions stay within a single shard. If cross-shard transactions are unavoidable, use saga patterns with compensating actions instead of 2PC, trading strict consistency for availability.

---

### Q10. How does sharding interact with database replication?

**Interview Answer**

Each shard can have its own primary and read replicas. Writes go to the shard's primary, reads can go to replicas. The routing layer must know which replica is up-to-date (usually the primary for reads-after-writes). Connection pooling per-shard with `sqlx::PgPool` handles the connection management, while the shard router directs queries to the appropriate pool.

---
