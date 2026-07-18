# Redis Distributed Locks

## Interview Question

How do you implement distributed locks using Redis, and what are the pitfalls of naive implementations?

## Interview Answer

A distributed lock ensures that only one process in a cluster can execute a critical section at a time. The naive approach uses `SET key value NX EX 30` — if the key is set, the lock is acquired; when done, delete the key. The problem is that if the lock holder crashes without releasing, the lock blocks all other processes until TTL expires. Redlock (proposed by Redis creator Antirez) uses multiple independent Redis instances to achieve stronger mutual exclusion by acquiring locks on a majority of N instances. In Rust, use the `redis-rs` crate to issue `SET NX EX` commands and implement lock renewal with a background Tokio task.

---

## Follow-up Questions & Answers

### Q1. What is the Redlock algorithm?

**Interview Answer**

Redlock acquires locks on N (typically 5) independent Redis master instances with no replication. The client: (1) gets the current time. (2) Tries to acquire a lock on each instance with `SET key uuid NX EX ttl`. (3) If lock is acquired on at least N/2+1 instances and total time is less than TTL, the lock is valid. (4) If lock validity expires before completing work, the client releases all locks. Redlock provides stronger safety than single-instance locking but requires careful implementation. Criticisms include vulnerability to clock skew, GC pauses, and network partitions.

---

### Q2. What is lock renewal and why is it needed?

**Interview Answer**

Lock renewal extends a lock's TTL before it expires, preventing other processes from acquiring it while work is still in progress. Without renewal, if a task takes longer than the lock's TTL (e.g., TTL is 10s but task takes 30s), the lock auto-expires and another process can acquire it, causing concurrent execution. Implement renewal by spawning a Tokio task that periodically (e.g., every TTL/3 seconds) issues `EXPIRE key ttl` to refresh the lock. The renewal task must verify the lock is still owned (check the UUID value) before renewing.

---

### Q3. How do you implement a safe lock release in Rust?

**Interview Answer**

Never use `DEL key` to release a lock — it releases a lock held by another process if your lock already expired. Instead, use a Lua script to atomically check the UUID value and delete only if it matches:

```lua
if redis.call("GET", KEYS[1]) == ARGV[1] then
    return redis.call("DEL", KEYS[1])
else
    return 0
end
```

In Rust with `redis-rs`:

```rust
let script = redis::Script::new(LUA_SCRIPT);
script.key(&lock_key).arg(&uuid).invoke_async(&mut con).await?;
```

This ensures you only release your own lock, preventing accidental release of another process's lock.

---

### Q4. What are the pitfalls of `SETNX`-based locking?

**Interview Answer**

(1) Lock expiration without completion — if the task runs longer than the TTL, the lock expires and another process acquires it. Solve with lock renewal. (2) Accidental release — `DEL` releases any lock, even if another process holds it. Solve with UUID + Lua script. (3) Single point of failure — if the Redis instance crashes, no process can acquire the lock. Solve with Redlock. (4) Clock skew — TTL-based expiration is vulnerable to clock differences between nodes. (5) Blocking — naive implementations spin-wait, wasting CPU. Use exponential backoff instead.

---

### Q5. When should you use Redlock vs a single-instance lock?

**Interview Answer**

Use a single-instance lock (`SET NX EX`) when you have a single Redis instance and brief critical sections where TTL expiry is unlikely, or when your application can tolerate rare double-execution. Use Redlock when you need stronger mutual exclusion guarantees across multiple processes/nodes and the cost of double-execution is high (e.g., financial transactions, unique resource allocation). Redlock adds complexity — it requires N independent Redis instances and careful timing logic. For most applications, a single-instance lock with UUID-based release and renewal is sufficient.

---

### Q6. How do you handle lock contention in a high-traffic Rust backend?

**Interview Answer**

Avoid blocking by using non-blocking lock attempts: `SET lock_key uuid NX EX 5` returns `OK` if acquired, `None` if not. If not acquired, either retry with exponential backoff (10ms, 20ms, 40ms...) up to a max attempts, return an error to the client, or queue the task for later processing. In Rust, use `tokio::time::sleep` for backoff delays. For extremely high contention, consider sharding the lock (e.g., `lock:resource:{id % 10}`) so multiple processes can work on different partitions of the resource simultaneously.

---

### Q7. What is the fencing token pattern?

**Interview Answer**

Fencing tokens are monotonically increasing tokens issued with each lock acquisition. When the lock holder writes to a downstream resource (e.g., database), it includes the token. The resource rejects writes with tokens lower than the last seen token, preventing stale lock holders from making conflicting writes. Redlock alone does not provide fencing tokens — you need to generate them at the lock acquisition layer. In practice, PostgreSQL sequences or Redis `INCR` on a counter can serve as fencing tokens. This adds a safety layer beyond lock mutual exclusion.

---

### Q8. How do you implement a lock renewal task in Rust with Tokio?

**Interview Answer**

Spawn a background task that periodically renews the lock:

```rust
use tokio::time::{interval, Duration};

let lock_key = "my_lock".to_string();
let uuid = uuid::Uuid::new_v4().to_string();
let ttl = 10u64;

con.set_ex(&lock_key, &uuid, ttl).await?;

let renewal = tokio::spawn({
    let con = con.clone();
    let lock_key = lock_key.clone();
    let uuid = uuid.clone();
    async move {
        let mut ticker = interval(Duration::from_secs(ttl / 3));
        loop {
            ticker.tick().await;
            let script = redis::Script::new(RENEW_SCRIPT);
            let result: bool = script.key(&lock_key).arg(&uuid).invoke_async(&mut con.clone()).await.unwrap_or(false);
            if !result { break; }
        }
    }
});

// ... do work ...

renewal.abort();
drop_lock(&mut con, &lock_key, &uuid).await;
```

The renewal task stops when the lock is released or the task completes.

---

### Q9. How does Redis lock compare to PostgreSQL advisory locks?

**Interview Answer**

PostgreSQL advisory locks (`SELECT pg_advisory_lock(id)`) are database-level locks that survive connection loss and are tied to the database session. They work well when all lock operations go through the same PostgreSQL instance. Redis locks are faster (sub-millisecond) and work across services that share a Redis instance but not a database connection. PostgreSQL locks don't require a separate system, while Redis locks require Redis availability. Use PostgreSQL advisory locks for DB-centric workflows; use Redis locks for distributed service coordination.

---

### Q10. What is the minimum viable distributed lock in production Rust?

**Interview Answer**

The minimum viable production lock requires: (1) `SET lock_key uuid NX EX ttl` for acquisition with a unique UUID. (2) Lua script for safe release (check UUID before DEL). (3) Background renewal task to prevent premature expiration. (4) Exponential backoff on acquisition failure. (5) Proper error handling — if Redis is unavailable, fail gracefully. This is sufficient for 95% of use cases. Add Redlock only if you have strict consistency requirements and multiple Redis instances. The `redis-rs` crate supports all of these patterns with its `Script` and `AsyncCommands` APIs.

