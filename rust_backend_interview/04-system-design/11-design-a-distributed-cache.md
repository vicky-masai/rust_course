# Design a Distributed Cache

## Interview Question

How would you design a distributed cache using Rust?

## Interview Answer

I would design a distributed cache using consistent hashing to distribute keys across cache nodes, with each node running a Tokio-based key-value store backed by an in-process LRU eviction policy. The client library (a Rust crate) connects to a ring of cache nodes via the hash ring, automatically routing requests to the correct node based on the key's hash. Each node stores data in memory with configurable TTL and LRU eviction, and optionally persists to disk for warm restarts. Gossip protocol handles cluster membership and failure detection, so nodes can join or leave without client reconfiguration. Replication provides fault tolerance — each key is stored on 3 nodes, and reads can be served from any replica.

---

## Follow-up Questions & Answers

### Q1. How does consistent hashing work and why is it important?

**Interview Answer**

Consistent hashing maps both keys and cache nodes to positions on a virtual ring (0 to 2^32). A key is assigned to the nearest node clockwise on the ring. When a node is added or removed, only the keys that mapped to that node need to be redistributed — in a ring with N nodes, only 1/N of keys are affected. Without consistent hashing, adding a node would redistribute all keys, causing a cache stampede. In Rust, I implement the ring using a `BTreeMap<u32, NodeId>` where I look up the successor of a key's hash using `btree.range(key_hash..).next()`. I use virtual nodes (150 per physical node) to ensure even distribution. The `xxhash` crate provides fast, well-distributed hashing for the ring.

---

### Q2. How do you handle cache node failures?

**Interview Answer**

Each key is replicated to 3 nodes (primary + 2 replicas), stored on consecutive positions on the hash ring. If a node fails, the gossip protocol detects the failure after 3 missed heartbeats (5-second intervals) and marks the node as down. Client reads fall back to the next replica on the ring. A background repair task runs on surviving nodes, scanning their local store for keys that originally hashed to the failed node and re-replicating them to new nodes on the ring. In Rust, I use `tokio::sync::watch` to broadcast the cluster topology to all client connections, so they immediately know which nodes are down. The trade-off is that during a node failure, 1/N of keys have reduced redundancy until the repair completes.

---

### Q3. How do you implement LRU eviction across the cluster?

**Interview Answer**

Each cache node maintains a local LRU policy using the `moka` crate, which provides a concurrent, high-performance LRU cache with TTL support. The cache capacity per node is configured (e.g., 10GB), and `moka` automatically evicts least-recently-used entries when capacity is reached. The eviction is probabilistic (using TinyLFU admission), which provides near-optimal hit rates with O(1) operations. When a node evicts a key, it does not propagate the eviction — replicas may still hold the key. This is acceptable because cache is a performance optimization, not a source of truth. I implement a `cache_evicted` metric counter to monitor eviction rates and adjust capacity. Cross-node eviction coordination would add too much latency and is not worth the consistency guarantee.

---

### Q4. How do you implement cache invalidation?

**Interview Answer**

I implement three invalidation mechanisms: (1) TTL-based expiration, where each key has a configurable time-to-live and is automatically evicted by `moka`'s background cleanup; (2) Explicit invalidation via a `DELETE /cache/{key}` API endpoint that removes the key from all replicas; (3) Event-driven invalidation, where the application publishes cache invalidation events to Kafka when the underlying data changes, and a cache consumer processes these events and removes affected keys. The Kafka approach ensures that when a database record is updated, the cache is invalidated within 1-2 seconds. I also implement a "versioned key" pattern where the cache key includes a version number, allowing the application to invalidate by incrementing the version rather than deleting the key.

---

### Q5. How do you implement the client library in Rust?

**Interview Answer**

The client library is a Rust crate that wraps the cache operations. It maintains a connection pool to each cache node (using `tokio` channels), implements consistent hashing to route keys to the correct node, and handles retries and failover. The API mirrors a HashMap: `client.get(&key)`, `client.set(&key, &value, ttl)`, `client.delete(&key)`. All operations return `Result<T, CacheError>` with typed errors. The client uses `serde` for serialization/deserialization, supporting any type that implements `Serialize`/`Deserialize`. Connection pooling uses `bb8` (a Tokio-native pool) with configurable max connections per node. The client is async-first and integrates seamlessly with Axum handlers using `.await`.

---

### Q6. How do you handle hot keys that overload a single node?

**Interview Answer**

Hot keys (e.g., a viral video's metadata) can overwhelm a single node because all requests for that key route to the same position on the hash ring. I implement three strategies: (1) Local caching at the application layer using `moka`, so frequently accessed keys are served from Axum's process memory without any cache network call; (2) Key replication with random suffixes — the client writes the same value to `key:1`, `key:2`, `key:3` on different nodes and reads from any of them; (3) Read replicas — the cache node itself maintains an in-memory local cache of hot keys, avoiding disk reads. I detect hot keys by monitoring per-key access counts in a Redis sorted set, and automatically promote keys exceeding 1000 reads/minute to the local cache layer.

---

### Q7. How do you ensure data consistency between cache and database?

**Interview Answer**

I use the cache-aside (lazy loading) pattern as the primary consistency mechanism: the application writes to the database first, then invalidates the cache. This ensures the cache is eventually consistent with the database (within the invalidation propagation delay). For stronger consistency on critical reads, I implement read-through with a short TTL — the cache always serves data but refreshes from the database when the TTL expires. In Rust, I wrap database writes and cache invalidation in a function that guarantees both operations execute: the database write uses a transaction, and cache invalidation uses a Redis `DEL` that I retry on failure. The trade-off is brief staleness (1-2 seconds) during writes, which is acceptable for most use cases.

---

### Q8. How do you monitor and operate the distributed cache?

**Interview Answer**

Each cache node exposes Prometheus metrics via an HTTP `/metrics` endpoint: hit rate, miss rate, eviction count, memory usage, key count, and node latency. I use Grafana dashboards to visualize cluster health, with alerts for hit rates below 80% (indicating capacity issues) or node failures. The gossip protocol provides cluster membership info, which I expose via a `/cluster` admin endpoint showing all nodes and their status. For capacity planning, I monitor memory usage per node and plan to add nodes when any node exceeds 80% capacity. I also implement a `cache_warm` command that preloads critical data from the database into the cache during deployments, preventing cold-start stampedes.

---

### Q9. How do you handle serialization and data types?

**Interview Answer**

I use `serde` with `bincode` for binary serialization, which is 3-10x faster and more compact than JSON for Rust-native types. The cache API accepts any type implementing `Serialize`/`Deserialize`, and the client handles serialization transparently. For cross-language compatibility (if other services need to read the cache), I support both `bincode` (for Rust clients) and `serde_json` (for non-Rust clients) with a format byte prefix: `0x01` for bincode, `0x02` for JSON. I also implement compression for values larger than 1KB using `lz4` (fast compression/decompression), reducing memory usage and network transfer. The serialization overhead adds 1-5 microseconds per operation, which is negligible compared to network latency.

---

### Q10. How do you implement cache warming and preloading?

**Interview Answer**

Cache warming ensures the cache is populated before the application starts serving traffic, preventing a "thundering herd" of cache misses on deployment. I implement a `cache_warm` binary that connects to the database, queries for the top N most-accessed records (identified by a `last_accessed_at` column or analytics data), and bulk-inserts them into the cache using Redis pipeline commands (100 keys per pipeline for efficiency). For Axum deployments, this runs as a Kubernetes init container that must complete before the main container starts. I also implement predictive warming: a background Tokio task monitors access patterns and pre-loads keys that are likely to be accessed soon (e.g., pre-loading today's trending articles at midnight). The warming process is idempotent and safe to run multiple times.
