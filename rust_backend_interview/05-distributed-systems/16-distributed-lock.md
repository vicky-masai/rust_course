# Distributed Lock

## Interview Question

How would you implement a distributed lock and what are the key considerations?

## Interview Answer

A distributed lock provides mutual exclusion across multiple nodes, preventing concurrent access to shared resources. The implementation must handle lock acquisition, hold detection, expiration, and release. Redis with `SET key value NX EX ttl` provides a fast but eventually consistent lock suitable for non-critical use cases. For strong consistency, use etcd or ZooKeeper with lease-based locks and fencing tokens. Critical considerations include: TTL to prevent deadlocks from crashed holders, fencing tokens to prevent stale holders from corrupting state, lock reentrancy (whether the same holder can acquire it multiple times), and the performance-correctness trade-off between Redis and consensus-based solutions.

---

## Follow-up Questions & Answers

### Q1. What is the difference between Redis-based and etcd-based distributed locks?

**Interview Answer**

Redis locks use `SET NX EX` for atomic acquisition with TTL, providing sub-millisecond latency but eventual consistency — a lock might be lost if Redis crashes without persistence or during a failover. etcd locks use Raft consensus, providing strong consistency with ~1-2ms latency. etcd automatically provides fencing tokens via its revision number, while Redis requires application-level fencing. Redis locks are suitable for cache invalidation, rate limiting, and session management where occasional safety violations are acceptable. etcd locks are required for leader election, distributed job scheduling, and any scenario where a lost lock could cause data corruption. The choice depends on your correctness requirements.

---

### Q2. How do you handle lock extension (watchdog) in Redis?

**Interview Answer**

A watchdog task periodically extends the lock TTL while the lock is held. In Rust, spawn a Tokio task that sleeps for half the TTL (e.g., 5 seconds for a 10-second TTL), then executes `EXPIRE lock_key 10` in Redis. If the extension fails (lock stolen or Redis unreachable), the watchdog signals the main task via a `tokio::sync::watch` channel to abort. The watchdog is cancelled when the lock is released. This pattern is used by Redisson (Java) and can be implemented in Rust with the `redis` crate. Without a watchdog, a long-running operation might exceed the TTL, release the lock, and allow another process to acquire it simultaneously.

---

### Q3. What is a fencing token and how do you implement one with etcd?

**Interview Answer**

A fencing token is a monotonically increasing number assigned when a lock is acquired, used to prevent stale lock holders from making changes. With etcd, each transaction gets a unique revision number that serves as the fencing token. When acquiring a lock, the client receives the revision. Every subsequent write to the shared resource includes this revision. The storage layer (database, cache) rejects writes with a revision lower than the latest seen. In Rust, use etcd's `txn` API: `client.txn().if(compare).then(ops).await` to acquire the lock and get the revision. Store the revision in the write payload and validate it at the storage layer.

---

### Q4. How do you prevent deadlocks in a distributed lock system?

**Interview Answer**

Deadlocks occur when processes hold locks while waiting for other locks that are held by processes waiting for the first. Prevention strategies: always acquire multiple locks in a consistent global order (e.g., alphabetical by resource name), use lock timeouts (TTL) so locks are automatically released after a maximum duration, implement try-lock with timeout instead of blocking indefinitely, and use deadlock detection with a wait-for graph. In a Rust Axum backend, wrap lock acquisition with `tokio::time::timeout(Duration::from_secs(10), acquire_lock()).await` to prevent indefinite blocking. For multi-resource locking, use a centralized lock service that can detect cycles.

---

### Q5. What are the ABA problems with distributed locks?

**Interview Answer**

The ABA problem occurs when a lock is acquired by Process A, released, acquired by Process B, released, and then acquired by Process A again — Process A thinks it still holds the original lock, but the state has changed (B modified the resource in between). Fencing tokens prevent this: each lock acquisition gets a unique token, and the storage layer tracks the latest token. Even if A reacquires the lock, it gets a new token and cannot use the old token to make changes. In Redis, this is harder to prevent because Redis does not natively provide fencing tokens. Use etcd's revision numbers or add application-level version tracking to prevent ABA.

---

### Q6. How do you implement a fair distributed lock (FIFO ordering)?

**Interview Answer**

A fair lock grants access in FIFO order — the first process to request the lock is the first to receive it. In Redis, this is achieved with a sorted set: clients add themselves with their timestamp as the score, and the lock is granted to the client with the lowest score. In etcd or ZooKeeper, use sequential ephemeral nodes — each client creates a sequential node, and the lock is held by the client with the lowest sequence number. Other clients watch the node immediately before theirs and are notified when it is deleted. In Rust, implement the sorted set approach with Redis `ZADD` and `ZRANGEBSCORE` commands for FIFO ordering.

---

### Q7. How do you implement distributed locks for leader election?

**Interview Answer**

Leader election uses a distributed lock where only one node can hold the lock at a time. The lock holder is the leader. Use etcd or ZooKeeper for strong consistency: each candidate creates a key with a lease, and only one succeeds (using a compare-and-swap). Non-leaders watch the key and attempt to acquire it when it is released (leader failure). The leader must periodically renew its lease (heartbeat) — if it fails, the lease expires and another candidate can acquire the lock. In a Rust service, implement leader election by attempting lock acquisition in a loop with retry logic, and running leader-specific tasks only while the lock is held.

---

### Q8. What happens when a Redis master fails while holding a distributed lock?

**Interview Answer**

If the Redis master fails before replicating the lock to a slave, the slave is promoted to master and the lock is lost. Another client can now acquire the lock on the new master, causing two clients to believe they hold the lock simultaneously. This is the fundamental safety issue with Redis distributed locks. Mitigations include: using Redis Sentinel with `min-replicas-to-write` to ensure the lock is replicated before acknowledging, using Redlock across multiple independent Redis instances (though this has its own controversies), or using etcd/ZooKeeper which provide strong consistency through Raft consensus and do not have this problem.

---

### Q9. How do you handle lock contention in high-throughput systems?

**Interview Answer**

High contention occurs when many processes compete for the same lock, causing latency spikes and reduced throughput. Strategies: reduce lock scope (hold the lock for the minimum necessary time), use fine-grained locking (multiple locks for different resources instead of one global lock), implement lock-free algorithms (CAS operations, MVCC) where possible, use read-write locks (multiple readers, single writer) for read-heavy workloads, and implement backoff strategies (exponential backoff with jitter) to reduce contention storms. In a Rust backend, use `tokio::sync::RwLock` for in-process locking and move to distributed locks only when cross-process coordination is required.

---

### Q10. How do you test distributed lock implementations?

**Interview Answer**

Test at multiple levels: unit test the lock acquisition/release logic with mock Redis, integration test with a real Redis/etcd instance, and fault-injection test with network partitions. Verify: mutual exclusion (two processes cannot hold the lock simultaneously), liveness (a lock is eventually released after TTL), safety (stale lock holders cannot modify state with fencing tokens), and fault tolerance (locks are released when holders crash). Use chaos testing tools (Toxiproxy) to simulate Redis failures during lock operations. In Rust, use testcontainers to spin up Redis and etcd, and `tokio::spawn` concurrent tasks to verify mutual exclusion under contention.
