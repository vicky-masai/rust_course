# How Would You Design a Distributed Lock?

## Interview Question

How would you design a distributed lock and what are the trade-offs?

## Interview Answer

A distributed lock provides mutual exclusion across multiple nodes in a distributed system. The simplest implementation uses Redis with `SET key value NX EX timeout`, which atomically sets a key only if it does not exist, with an expiration to prevent deadlocks from crashed lock holders. For stronger consistency guarantees, use etcd or ZooKeeper with ephemeral nodes and watch mechanisms. The key design decisions are: lock acquisition protocol (spin-lock vs blocking vs callback), lock expiration (to handle crashed holders), fencing tokens (to prevent stale lock holders from corrupting state), and the trade-off between performance and correctness. Redis-based locks are fast but have edge cases; etcd/ZooKeeper locks are correct but slower.

---

## Follow-up Questions & Answers

### Q1. What is the Redlock algorithm and what are its controversies?

**Interview Answer**

Redlock is Redis's algorithm for distributed locks across multiple independent Redis instances. It acquires the lock by sending `SET NX` to N (typically 5) Redis instances, succeeding on a majority (3), and calculates the validity time. Martin Kleppmann famously critiqued Redlock in "How to do distributed locking," arguing that clock drift and GC pauses can cause safety violations — a lock holder might believe it holds the lock while another node has already acquired it. The Redis author (Salvatore Sanfilippo) responded with counterarguments. The controversy highlights that Redlock is not suitable for strong correctness requirements — use etcd or ZooKeeper for those, and Redlock only when performance matters more than absolute safety.

---

### Q2. What are fencing tokens and why are they essential?

**Interview Answer**

A fencing token is a monotonically increasing number assigned each time a lock is acquired. Every write operation includes the token, and the storage layer rejects writes with a stale (lower) token. This prevents a classic split-brain scenario: Node A acquires the lock, gets partitioned, Node B acquires the same lock, and then Node A (unaware it lost the lock) writes stale data. With fencing tokens, Node A's write is rejected because its token is lower than Node B's. In practice, implement fencing tokens using etcd's revision numbers or ZooKeeper's zxid. Redis does not natively support fencing tokens, which is one reason etcd/ZooKeeper are preferred for correctness-critical locks.

---

### Q3. How do you implement a distributed lock in Redis using Rust?

**Interview Answer**

In Rust, use the `redis` crate with the `SET` command: `redis::cmd("SET").arg(&lock_key).arg(&lock_value).arg("NX").arg("EX").arg(ttl_seconds)`. The `lock_value` is a UUID generated on acquisition, used to verify ownership when releasing. Release the lock with a Lua script: `if redis.call('GET', KEYS[1]) == ARGV[1] then return redis.call('DEL', KEYS[1]) else return 0 end` — this atomically checks the value before deleting. Wrap this in an `async` function using `tokio` that returns a `DistributedGuard` struct implementing `Drop` to auto-release the lock. Handle Redis connection failures by returning an error rather than proceeding without the lock.

---

### Q4. What are the failure modes of Redis-based distributed locks?

**Interview Answer**

If the lock holder crashes while holding the lock, the TTL ensures the lock is eventually released — but there is a window where no one holds the lock and another process might acquire it, leading to two processes believing they hold the lock. If Redis crashes and restarts without persistence, all locks are lost. If the lock holder's clock drifts, it might release the lock early or hold it too long. If the lock release (DEL) fails due to a network issue, the lock might be held by a stale client. Mitigations include: using Redis Sentinel/Cluster for high availability, implementing fencing tokens at the storage layer, and using extension mechanisms (watchdog patterns) to renew locks before they expire.

---

### Q5. How does an etcd-based distributed lock differ from Redis?

**Interview Answer**

etcd uses the Raft consensus protocol, providing strong consistency guarantees that Redis cannot match. Lock acquisition uses a lease with a key — create a key with a lease, and the key is automatically deleted when the lease expires. Other clients watch the key and are notified when it is released, allowing efficient blocking instead of polling. etcd provides fencing tokens via its revision number — each transaction gets a monotonically increasing revision. The trade-off is latency: etcd lock acquisition takes 1-2ms (due to Raft consensus), while Redis takes sub-millisecond. For correctness-critical locks (leader election, distributed job scheduling), etcd is preferred. For performance-critical locks with relaxed correctness (cache invalidation), Redis is sufficient.

---

### Q6. How do you prevent deadlocks in a distributed lock system?

**Interview Answer**

Distributed deadlocks occur when two or more processes each hold a lock the other needs. Prevention strategies include: always acquire locks in a consistent global order (e.g., alphabetical by resource name), use lock timeouts so locks are automatically released after a maximum duration, implement deadlock detection using wait-for graphs (each process reports which locks it holds and which it is waiting for), and use try-lock with timeout instead of blocking indefinitely. In a Rust backend, use `tokio::time::timeout` around lock acquisition to prevent indefinite blocking. For lock ordering, document and enforce that all services acquire locks in the same order for the same set of resources.

---

### Q7. What is the difference between a reentrant lock and a non-reentrant lock in a distributed system?

**Interview Answer**

A reentrant lock allows the same holder to acquire it multiple times without deadlocking — the lock maintains a count of acquisitions and releases. In a distributed system, reentrant locks are complex because the lock service must track which client holds the lock and how many times. Redis-based locks are typically non-reentrant — if the same client tries to acquire a lock it already holds, it overwrites the lock value and loses ownership. Implementing reentrancy requires storing the lock holder's identity and a counter, or using a session-based approach where each client has a unique session ID that tracks all held locks. In most distributed systems, non-reentrant locks with careful design are sufficient.

---

### Q8. How do you implement a distributed lock with TTL watchdog?

**Interview Answer**

A TTL watchdog is a background task that periodically extends the lock's TTL while the lock is held, preventing premature expiration during long operations. In a Rust service, start a Tokio task that sleeps for half the TTL duration, then extends the lock using `EXPIRE key new_ttl` in Redis. If the extension fails (lock was stolen or Redis is down), the watchdog signals the main task to abort. The `redis-rs` crate provides `expire` for TTL extension. Libraries like `redlock` in Java implement this pattern. The watchdog should be stopped when the lock is released, and the `Drop` implementation for the lock guard should cancel the watchdog task.

---

### Q9. When would you use ZooKeeper over Redis for distributed locks?

**Interview Answer**

Use ZooKeeper when you need strong consistency and correct behavior — ZooKeeper provides sequential ephemeral nodes that give you both locking and fencing semantics. The sequential node ensures that each lock acquisition gets a unique, ordered ID (like a fencing token), and the ephemeral node ensures automatic cleanup if the client crashes. ZooKeeper's watch mechanism allows efficient notification when a lock is released, avoiding polling. Redis is preferred when latency and throughput are the primary concerns and the application can tolerate occasional safety violations. For leader election in a Kubernetes operator or distributed coordination in a database cluster, ZooKeeper or etcd is the correct choice.

---

### Q10. How do you choose between Redis, etcd, and ZooKeeper for distributed locks?

**Interview Answer**

**Redis** is the choice when you need maximum performance (sub-millisecond lock acquisition), the application can handle rare safety violations, and you do not need fencing tokens. Suitable for cache invalidation, rate limiting, and session management. **etcd** is the choice for Kubernetes-native systems, when you need strong consistency, automatic fencing via revision numbers, and efficient watch-based notification. Suitable for leader election, distributed job scheduling, and feature flags. **ZooKeeper** is the choice for existing Hadoop/Kafka ecosystems where ZooKeeper is already deployed, or when you need the mature sequential ephemeral node primitives. For a new Rust microservice, I would default to etcd for its simplicity, strong consistency, and natural integration with cloud-native infrastructure.
