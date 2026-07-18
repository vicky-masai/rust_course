# Redis Cluster vs Sentinel

## Interview Question

Explain the difference between Redis Cluster and Redis Sentinel, and when to use each architecture.

## Interview Answer

Redis Sentinel is a high-availability solution for single-instance Redis — it monitors master/slave instances, detects failures, and automatically promotes a replica to master. It does not shard data across instances. Redis Cluster is a distributed architecture that partitions data across multiple master nodes using hash slots (16,384 slots), enabling horizontal scaling of both reads and writes. Sentinel is simpler and better for small datasets that fit on one node; Cluster is necessary when your dataset exceeds single-node RAM or you need write scalability across shards. In Rust, `redis-rs` supports both via `SentinelCluster` and `cluster` features with automatic node discovery and slot routing.

---

## Follow-up Questions & Answers

### Q1. How does Redis Cluster partition data across nodes?

**Interview Answer**

Redis Cluster divides the 16,384 hash slots among master nodes using `CRC16(key) % 16384`. Each key maps to exactly one slot, and each slot is owned by exactly one master. When a client sends a command for a key on the wrong node, Redis responds with a `MOVED` redirect pointing to the correct node. Clients with cluster support (like `redis-rs` with `cluster` feature) cache the slot-to-node mapping and redirect automatically. Multi-key operations must use keys in the same slot — use hash tags `{tag}` to force keys into the same slot (e.g., `user:{123}:profile` and `user:{123}:posts` both hash on `123`).

---

### Q2. What happens when a Redis Cluster node fails?

**Interview Answer**

Cluster detects failure through gossip protocol — nodes periodically ping each other, and if a majority of masters agree a node is down, it is marked as failing. A replica of the failed master is automatically promoted via cluster failover (similar to Sentinel but automatic). During failover, the affected slots are unavailable briefly (typically 1-10 seconds). If no replica is available, those slots become unavailable until you manually assign a new node. Redis Cluster does not perform automatic replica reassignment — you must provision new replicas. Monitor with `CLUSTER INFO` and `CLUSTER NODES`.

---

### Q3. How does Sentinel detect master failure?

**Interview Answer**

Sentinel uses a quorum-based system: multiple Sentinel instances monitor the master. Each Sentinel sends `PING` to the master every second. If the master does not respond within `down-after-milliseconds` (typically 5000ms), the Sentinel marks it as `SDOWN` (subjectively down). When a quorum of Sentinels agree the master is SDOWN, it becomes `ODOWN` (objectively down), triggering a failover election. A Sentinel is elected leader via Raft consensus, and it promotes the most up-to-date replica to master and updates other Sentinels and clients.

---

### Q4. What is the maximum dataset size for Redis Sentinel?

**Interview Answer**

With Sentinel, the dataset must fit entirely on one master node's RAM, typically up to 25-50 GB with modern hardware. Beyond that, performance degrades due to fork() overhead during RDB snapshots, increased memory fragmentation, and slower command execution. If your dataset exceeds single-node capacity, you need Redis Cluster to shard data across multiple nodes. Sentinel is ideal for caching layers, session stores, and small-to-medium datasets where simplicity is preferred over horizontal scaling.

---

### Q5. How do you connect to Redis Cluster from Rust using `redis-rs`?

**Interview Answer**

Enable the `cluster` feature in `redis-rs` and use `ClusterClient`:

```rust
use redis::cluster::ClusterClient;

let nodes = vec![
    "redis://10.0.0.1:6379",
    "redis://10.0.0.2:6379",
    "redis://10.0.0.3:6379",
];
let client = ClusterClient::new(nodes)?;
let mut con = client.get_async_connection().await?;

// Use like a normal connection — cluster routing is automatic
con.set("key", "value").await?;
let val: String = con.get("key").await?;
```

The `ClusterClient` handles `MOVED` and `ASK` redirections automatically, caches slot mappings, and reconnects to available nodes.

---

### Q6. What are the tradeoffs between Cluster and Sentinel?

**Interview Answer**

Sentinel: simpler to operate, supports pub/sub across all instances, no hash tag constraints, and is sufficient for most caching use cases. Limitation: single master bottleneck, dataset limited by one node's RAM. Cluster: horizontal read/write scaling, datasets can span multiple nodes, automatic shard failover. Limitation: multi-key operations require same-slot keys (hash tags), pub/sub is per-shard (not global), more complex operations and monitoring. Sentinel is the right default for small teams; Cluster is necessary at scale (100k+ ops/sec or datasets >25 GB).

---

### Q7. How do you handle cross-slot transactions in Redis Cluster?

**Interview Answer**

Redis Cluster does not support multi-key transactions across different slots. If you need atomic multi-key operations, all keys must be in the same slot using hash tags: `{user:123}:profile`, `{user:123}:settings`. Use `{tag}` prefix to ensure co-location. Alternatively, use Redis Cluster's `MULTI`/`EXEC` within a single slot, or redesign the data model to avoid cross-slot dependencies. For operations that truly need cross-shard atomicity, consider a Lua script on a single shard or use an application-level saga pattern.

---

### Q8. How do you monitor Redis Cluster health?

**Interview Answer**

Use `CLUSTER INFO` to check `cluster_state` (should be `ok`), `cluster_slots_assigned`, `cluster_slots_ok`, and `cluster_known_nodes`. Use `CLUSTER NODES` to see the full topology with master/replica relationships and slot assignments. Monitor `INFO memory` per node, `INFO replication` for lag, and `CLUSTER SLOTS` for slot distribution. In Rust, periodically query `redis::cmd("CLUSTER").arg("INFO")` and parse the response. Alert on `cluster_state:fail`, high replication lag, or uneven slot distribution.

---

### Q9. Can you mix Sentinel and Cluster?

**Interview Answer**

Not directly — they serve different purposes. Sentinel provides HA for a single master-replica setup; Cluster provides sharding across multiple masters. Within a Redis Cluster, you can configure replicas for each master (providing HA within the cluster), which replaces Sentinel's role. You should not run Sentinel on top of a Cluster. If you need both sharding and HA, use Redis Cluster with replicas — it provides both. Use Sentinel only when you have a single-master topology and need automatic failover without horizontal scaling.

---

### Q10. What is the recommended architecture for a high-availability Rust backend?

**Interview Answer**

For most applications: Redis Sentinel with 1 master, 2 replicas, and 3 Sentinel instances. This provides HA with automatic failover for datasets under 25 GB. For larger scale: Redis Cluster with 6+ master nodes (3 masters with 3 replicas minimum), providing both sharding and HA. Your Rust application connects via `redis-rs`'s `SentinelClient` or `ClusterClient`, which handle failover transparently. Use connection pooling (`RedisConnectionManager`) for resilience. Deploy Redis on dedicated nodes with sufficient RAM and SSDs for persistence. Monitor with Prometheus + Grafana using `redis_exporter`.

