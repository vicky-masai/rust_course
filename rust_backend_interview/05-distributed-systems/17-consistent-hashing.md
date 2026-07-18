# Consistent Hashing

## Interview Question

Explain consistent hashing and why it is important in distributed systems.

## Interview Answer

Consistent hashing maps both keys and servers to positions on a virtual ring (hash ring), minimizing the number of keys that need to be remapped when servers are added or removed. In traditional hash-based distribution (`hash(key) % N`), adding or removing a server changes N, causing nearly all keys to be remapped. Consistent hashing ensures that only keys between the affected server and its predecessor on the ring are moved — approximately `K/N` keys where K is total keys and N is servers. This is critical for distributed caches (Memcached, Redis Cluster), load balancers, and distributed databases where minimizing data movement during scaling is essential.

---

## Follow-up Questions & Answers

### Q1. How does consistent hashing work step by step?

**Interview Answer**

Both servers and keys are hashed to positions on a ring (e.g., 0 to 2^32 - 1). To find which server a key belongs to, hash the key, walk clockwise on the ring, and the first server encountered is responsible for that key. When a server is added, it takes over the segment between itself and its counterclockwise neighbor — only keys in that segment are remapped. When a server is removed, its keys are passed to the next server clockwise. In practice, each server is mapped to multiple virtual nodes (100-200) on the ring for better load distribution. In Rust, use `std::collections::BTreeMap<u64, ServerId>` for O(log n) ring lookups.

---

### Q2. What is the virtual node (vnode) problem and how do you solve it?

**Interview Answer**

With a single hash position per server, the ring is unevenly distributed — some servers get many more keys than others. Virtual nodes solve this by mapping each server to multiple positions on the ring (typically 100-200). This distributes keys more evenly because each server's multiple positions create smaller, more uniform segments. With 200 virtual nodes per server, the standard deviation of load drops to about 5% of mean load. In Rust, implement virtual nodes by hashing `(server_id, vnode_index)` for each server. The trade-off is increased memory (the ring stores all virtual node positions) and slightly more complex rebalancing when servers join or leave.

---

### Q3. How is consistent hashing used in Redis Cluster?

**Interview Answer**

Redis Cluster divides the key space into 16,384 hash slots. Each server is responsible for a subset of slots, and keys are assigned to slots using `CRC16(key) % 16384`. When a server is added, slots are migrated from existing servers to the new one. When a server is removed, its slots are distributed among remaining servers. This is consistent hashing with a fixed number of virtual nodes (16,384 slots). The `redis-cli --cluster` command manages slot assignment. In a Rust client, compute `CRC16(key) % 16384` to determine which node to contact, and handle MOVED/ASK redirections when slots are being migrated.

---

### Q4. What are the limitations of basic consistent hashing?

**Interview Answer**

Basic consistent hashing can still produce uneven load distribution, especially with a small number of servers. It does not account for heterogeneous servers (a server with 2x capacity should get 2x keys). It does not handle server failures gracefully without a rebalancing protocol. Virtual nodes help with even distribution but add complexity. Load-aware consistent hashing (adjusting the ring based on server load) is more complex but handles heterogeneous environments. In practice, systems like Cassandra and DynamoDB combine consistent hashing with virtual nodes, load-aware routing, and hinted handoff to handle these limitations.

---

### Q5. How would you implement consistent hashing in Rust?

**Interview Answer**

Use a `BTreeMap<u64, ServerId>` as the hash ring. For each server, insert N virtual nodes by hashing `(server_name, i)` for i in 0..N. To find a server for a key, hash the key and use `btree.range(key_hash..).next()` to find the next server clockwise (wrapping to the first entry if at the end). Use a fast hash function like `xxhash` or `murmur3` for hashing keys and server positions. Wrap the implementation in a `ConsistentHash` struct with `add_server`, `remove_server`, and `get_server` methods. Use `Arc<RwLock<ConsistentHash>>` for concurrent access in an Axum application. The entire implementation is typically under 100 lines of Rust code.

---

### Q6. How does consistent hashing handle server failures?

**Interview Answer**

When a server fails, its keys must be remapped to the next server clockwise on the ring. In a distributed system, this is handled by: the load balancer detecting the failure and removing the server from the ring, keys that were on the failed server now mapping to the next server, and the next server loading or fetching those keys from a backup (replication). For caches, this means a cache miss for the failed server's keys until they are reloaded. For databases, replication ensures the keys are available on another server. Hinted handoff (used by DynamoDB and Riak) temporarily stores writes for the failed server and replays them when it recovers.

---

### Q7. What is the difference between consistent hashing and range-based partitioning?

**Interview Answer**

Consistent hashing distributes keys based on their hash value, providing uniform distribution but no ordering guarantees — keys with similar values may land on different servers. Range-based partitioning assigns contiguous key ranges to servers (e.g., keys A-M on server 1, N-Z on server 2), preserving ordering and enabling efficient range queries. Consistent hashing is better for key-value stores and caches where individual lookups dominate. Range-based partitioning is better for time-series data and ordered scans. Many systems combine both: Cassandra uses consistent hashing for distribution but supports range-based queries within a partition through clustering keys.

---

### Q8. How does consistent hashing interact with replication?

**Interview Answer**

In a replicated consistent hashing system, each key is stored on multiple consecutive servers clockwise from its position on the ring. For example, with replication factor 3, a key maps to servers A, B, and C (the next three clockwise). When a server fails, its replicas are available on the other servers in the replica set. Writes go to the primary (first server clockwise) and are replicated to the secondaries. Cassandra uses this approach with a configurable replication factor. The ring position determines the primary, and the next N-1 positions are replicas. This ensures that even if a server fails, at least one replica is available.

---

### Q9. What are consistent hashing load balancing algorithms used in production?

**Interview Answer**

Production systems use variations of consistent hashing with load awareness. Rendezvous hashing (Highest Random Weight) is an alternative that assigns keys to the server with the highest hash score, providing better load distribution without virtual nodes. Jump consistent hashing (used by Google) provides O(1) space and perfect balance but does not support arbitrary server removal. Amazon's DynamoDB uses consistent hashing with virtual nodes and load-aware routing, adjusting the ring based on server capacity and current load. In a Rust load balancer, implement consistent hashing with virtual nodes and add a health check layer that removes unhealthy servers from the ring.

---

### Q10. How do you handle hotspots with consistent hashing?

**Interview Answer**

Hotspots occur when a disproportionate number of keys map to a single server, even with virtual nodes. Causes include: popular keys (celebrity tweets, trending products), hash function non-uniformity, and uneven virtual node distribution. Solutions include: increasing the number of virtual nodes, using a better hash function (xxhash, murmur3), adding hotkey replication (store popular keys on multiple servers), and implementing client-side caching for hot keys. In a Rust backend, detect hotspots by monitoring per-server request rates, and dynamically adjust by adding virtual nodes or redirecting traffic. Some systems use "virtual server splitting" where a hot server spawns additional virtual nodes on the ring to distribute its load.
