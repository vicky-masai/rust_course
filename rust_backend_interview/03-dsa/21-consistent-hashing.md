# Consistent Hashing

## Interview Question

What is Consistent Hashing and why is it essential for distributed systems?

## Interview Answer

Consistent Hashing is a technique for distributing data across nodes such that when a node is added or removed, only a minimal fraction of keys need to be remapped — approximately `K/n` keys where K is total keys and n is nodes. Traditional modular hashing (`hash(key) % n`) remaps ALL keys when n changes. Consistent Hashing maps both keys and nodes to positions on a virtual ring (0 to 2^32-1). A key is assigned to the first node encountered clockwise from its position. This ensures minimal disruption during scaling. **Virtual nodes** (replicas) improve load balancing by mapping each physical node to multiple positions on the ring.

**Time Complexity**: O(log n) for lookup (n = number of virtual nodes)
**Space Complexity**: O(n × v) where v = virtual nodes per physical node

---

## Follow-up Questions & Answers

### Q1. How does Consistent Hashing minimize key redistribution?

**Interview Answer**

When a node is removed, only keys assigned to that node's range on the ring need to move — they shift to the next clockwise node. Approximately K/n keys are affected (where K = total keys, n = nodes). With traditional `hash(key) % n`, changing n remaps every key. For example, with 1000 keys across 10 nodes: consistent hashing moves ~100 keys per node change vs 1000 keys with modular hashing. This is critical for: auto-scaling (adding/removing servers), failover (node crashes), and rolling deployments. The ring representation ensures only adjacent nodes are affected by changes.

---

### Q2. How would you implement Consistent Hashing in Rust?

**Interview Answer**

```rust
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct ConsistentHash {
    ring: BTreeMap<u64, String>,
    virtual_nodes: usize,
}

impl ConsistentHash {
    fn new(virtual_nodes: usize) -> Self {
        ConsistentHash { ring: BTreeMap::new(), virtual_nodes }
    }

    fn hash(&self, key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn add_node(&mut self, node: &str) {
        for i in 0..self.virtual_nodes {
            let key = format!("{}#{}", node, i);
            self.ring.insert(self.hash(&key), node.to_string());
        }
    }

    fn remove_node(&mut self, node: &str) {
        for i in 0..self.virtual_nodes {
            let key = format!("{}#{}", node, i);
            self.ring.remove(&self.hash(&key));
        }
    }

    fn get_node(&self, key: &str) -> Option<&String> {
        if self.ring.is_empty() { return None; }
        let hash = self.hash(key);
        self.ring.range(hash..).next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, node)| node)
    }
}
```

---

### Q3. What are virtual nodes and why are they necessary?

**Interview Answer**

Without virtual nodes, each physical node maps to a single position on the ring, creating uneven key distribution — some nodes get 2× the average load. Virtual nodes (vnodes) map each physical node to multiple positions (typically 100-200 per node). This distributes each node's key range across multiple segments, smoothing out imbalances. With enough virtual nodes, each physical node handles approximately K/n keys. Virtual nodes also enable **heterogeneous nodes** — powerful servers get more virtual nodes. Used in: Amazon DynamoDB (256 vnodes per node), Apache Cassandra (256 vnodes), and Riak (64 vnodes). The tradeoff: more virtual nodes means larger routing tables and more memory.

---

### Q4. How is Consistent Hashing used in distributed caches?

**Interview Answer**

**Redis Cluster** uses Consistent Hashing (with 16,384 hash slots) to distribute keys across nodes. Each key maps to a slot, and slots are distributed across nodes. When a node is added, some slots migrate to it. **Memcached** clients use Consistent Hashing to determine which server holds a cached key. **CDNs** use Consistent Hashing to route requests to edge servers. **DynamoDB** uses it for partition management. The key benefit: when a cache node fails, only its keys need to be re-cached (evicted and re-fetched from the database), minimizing cache miss storms. In Rust, the `ketama` crate provides consistent hashing for Memcached clients.

---

### Q5. What is the difference between Consistent Hashing and key-space partitioning?

**Interview Answer**

**Consistent Hashing**: Maps keys to positions on a ring, with nodes owning ranges. Keys near node boundaries may be unevenly distributed without virtual nodes. **Key-space partitioning**: Divides the key range into equal-sized chunks (e.g., 0-999 → Node A, 1000-1999 → Node B). Simple but requires resharding all data when nodes change. Consistent Hashing is preferred for dynamic systems (frequent scaling). Key-space partitioning is preferred for static systems (fixed node count) or when you need ordered data (range queries). Many systems combine both: Cassandra uses Consistent Hashing for token ranges, but also supports ordered partitioning for time-series data.

---

### Q6. How does Consistent Hashing handle node failures?

**Interview Answer**

When a node fails, its keys are automatically served by the next node clockwise on the ring. No immediate action is needed — the ring naturally redirects traffic. However, the successor node now handles extra load. To restore balance: add a replacement node (which takes some load from the successor) or rebalance virtual nodes. For **read-replica** systems: if the primary node fails, reads continue from replicas that also map to the ring. For **write systems**: use quorum writes across virtual nodes to tolerate failures. In Rust, implement health checks that remove failed nodes from the ring and trigger gradual rebalancing.

---

### Q7. What is Jump Consistent Hash and how does it differ?

**Interview Answer**

**Jump Consistent Hash** (Google, 2014) is a faster, simpler alternative that maps keys to buckets with minimal movement. It uses a single hash and a "jump" function: `bucket = hash(key) mod n`. When n changes, only keys that "jump" to the new bucket are moved. Time: O(ln n) per key. Space: O(1) — no ring data structure needed. Limitation: only works with contiguous bucket indices (0 to n-1) and doesn't support arbitrary node IDs. Use Jump Consistent Hash when nodes are numbered sequentially (common in sharded databases). Use standard Consistent Hashing when nodes have arbitrary IDs (common in microservices).

---

### Q8. How is Consistent Hashing used in load balancing?

**Interview Answer**

**Client-side load balancing**: Each client maintains a consistent hash ring and routes requests directly to the appropriate server. **Proxy-based load balancing**: Envoy, Nginx, and HAProxy use Consistent Hashing for sticky sessions — all requests from a client go to the same backend. **Database sharding**: Each shard owns a range of the hash ring. Consistent hashing ensures even distribution and minimal resharding. **P2P networks**: BitTorrent uses Consistent Hashing to distribute file chunks across peers. In Rust, `hyper` and `axum` can implement consistent-hash-based routing using custom middleware that hashes a request attribute (IP, user ID, session token) to select the backend.

---

### Q9. What are the limitations of basic Consistent Hashing?

**Interview Answer**

**Load imbalance**: Without enough virtual nodes, some nodes get more keys. **Heterogeneous nodes**: Basic Consistent Hashing treats all nodes equally — powerful servers get the same load as weak ones. **Hotspots**: Popular keys (celebrity tweets, viral content) can overload a single node. **Range queries**: Consistent Hashing doesn't preserve key ordering, making range queries impossible. **Memory overhead**: Large virtual node counts increase ring size. Mitigations: virtual nodes (for imbalance), weighted virtual nodes (for heterogeneity), key replication (for hotspots), and range-aware Consistent Hashing (for ordered data). In production, Cassandra uses 256 vnodes with consistent hashing and token-aware routing to address these limitations.

---

### Q10. How would you implement a production-ready Consistent Hash ring in Rust?

**Interview Answer**

For production: use `BTreeMap<u64, NodeId>` for the ring (O(log n) lookup). Store virtual node mappings separately for easy removal. Add **replication** — each key maps to r nodes (primary + r-1 replicas clockwise). Implement **node health tracking** — mark nodes as unhealthy and skip them during lookup. Add **load monitoring** — periodically rebalance virtual nodes based on actual load. Use a **background task** (tokio::spawn) for ring updates. Cache lookup results for frequently accessed keys. For high throughput, use `Arc<RwLock<ConsistentHash>>` for concurrent access. The `hashring` and `ketama` crates provide production-ready implementations.
