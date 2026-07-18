# Redis Lua Scripting

## Interview Question

Why and how would you use Lua scripting with Redis, and what atomicity guarantees does it provide?

## Interview Answer

Redis executes Lua scripts atomically — the entire script runs as a single uninterruptible command, eliminating race conditions between multiple Redis operations. This is critical for operations like conditional updates (check-and-set), distributed locks with safe release, rate limiters, and any multi-step logic that must be consistent. Lua scripts run server-side, reducing network round-trips between your Rust app and Redis. Use `redis-rs`'s `Script` type to define and invoke Lua scripts with `KEYS` and `ARGV` parameters, and always use `KEYS` for key names (required for cluster mode) and `ARGV` for values.

---

## Follow-up Questions & Answers

### Q1. How does Redis ensure Lua script atomicity?

**Interview Answer**

Redis processes commands one at a time in its single-threaded event loop. When a Lua script is executing, no other client commands can run — the script has exclusive access to the Redis instance. This means if your script does a `GET` followed by a conditional `SET`, no other client can modify the key between those two operations. The tradeoff is that long-running scripts block all other commands — keep scripts under 5ms of execution time. Use `redis-cli --latency` to monitor script impact on response times.

---

### Q2. How do you invoke a Lua script from Rust with `redis-rs`?

**Interview Answer**

Use `redis::Script` to define and invoke scripts:

```rust
use redis::Script;

let script = Script::new(r#"
    local current = redis.call('GET', KEYS[1])
    if current == ARGV[1] then
        redis.call('DEL', KEYS[1])
        return 1
    else
        return 0
    end
"#);

let result: bool = script
    .key("lock:resource_1")
    .arg("my-uuid-123")
    .invoke_async(&mut con)
    .await?;

if result {
    println!("Lock released successfully");
} else {
    println!("Lock not owned by this process");
}
```

`Script` handles `EVALSHA` caching automatically — it sends the script once, then uses the SHA hash for subsequent calls, reducing bandwidth.

---

### Q3. What are KEYS and ARGV in Lua scripts?

**Interview Answer**

`KEYS` is an array of Redis key names the script operates on — required for Redis Cluster to route the script to the correct node. Always use `KEYS` for key names. `ARGV` is an array of arbitrary arguments (values, UUIDs, thresholds, etc.). Access them as `KEYS[1]`, `ARGV[1]`, etc. In `redis-rs`:

```rust
script.key("user:123").arg("value").arg(300)
// KEYS[1] = "user:123", ARGV[1] = "value", ARGV[2] = 300
```

Using `KEYS` for key names is mandatory in cluster mode — scripts that use hardcoded key names will fail on clusters.

---

### Q4. What are common Lua scripting patterns for Redis?

**Interview Answer**

(1) Check-and-set: `GET key` → compare → `SET` or `DEL`. (2) Conditional increment: check value before `INCR` (e.g., rate limiting). (3) Rate limiter with sliding window: `ZRANGEBYSCORE` → `ZCARD` → conditionally `ZADD`. (4) Safe lock release: `GET key` → compare UUID → `DEL`. (5) Atomic batch operations: multiple `GET`/`SET` in one script. (6) Rate limiting with token bucket: calculate token availability and consume atomically. All these patterns would have race conditions without Lua script atomicity.

---

### Q5. How do you debug Lua scripts in Redis?

**Interview Answer**

Use `redis.log(redis.LOG_DEBUG, message)` inside scripts to log to the Redis server log. Test scripts interactively with `redis-cli EVAL "script" 0 args`. Use `redis-cli --ldb` for step-by-step debugging of Lua scripts. Check for errors by inspecting the return value — `redis-rs` returns `RedisError` for script errors. Add `redis.log(redis.LOG_WARNING, ...)` for production debugging. Use `redis-cli SCRIPT EXISTS sha1` to verify script caching. Monitor `INFO commandstats` for `eval` and `evalsha` call counts and timing.

---

### Q6. What are the limitations of Redis Lua scripting?

**Interview Answer**

(1) No I/O operations — scripts cannot call external APIs, read files, or access the network. (2) Single-threaded execution — long scripts block all other commands (keep under 5ms). (3) No random access to KEYS/ARGV beyond what's passed. (4) Scripts must be deterministic for AOF/RDB persistence (the `lua-time-limit` setting, default 5 seconds, triggers a warning but doesn't kill the script). (5) In cluster mode, all keys must be in the same slot (passed via KEYS). (6) Debugging is harder than application-level code.

---

### Q7. How do you use Lua scripts for atomic batch operations?

**Interview Answer**

Lua scripts let you execute multiple operations atomically without `MULTI`/`EXEC`:

```lua
local user = redis.call('HGETALL', KEYS[1])
local posts = redis.call('LRANGE', KEYS[2], 0, 9)
return {user, posts}
```

This fetches a user hash and their 10 most recent posts atomically — no other client can modify either key between the two operations. In Rust:

```rust
let script = redis::Script::new(r#"
    local user = redis.call('HGETALL', KEYS[1])
    local posts = redis.call('LRANGE', KEYS[2], 0, 9)
    return {user, posts}
"#);

let result: (Vec<String>, Vec<String>) = script
    .key("user:123")
    .key("user:123:posts")
    .invoke_async(&mut con).await?;
```

---

### Q8. How do you cache and reuse Lua scripts efficiently?

**Interview Answer**

Redis supports `SCRIPT LOAD` to cache scripts by SHA1 hash, then `EVALSHA sha1 KEYS ARGV` to invoke without sending the full script text. `redis-rs`'s `Script` type handles this automatically — it sends `SCRIPT LOAD` on first invocation, then uses `EVALSHA` for subsequent calls. If the SHA is evicted from Redis's script cache, `redis-rs` transparently falls back to `EVAL`. Monitor script cache with `SCRIPT EXISTS` and `SCRIPT FLUSH` to clear. For frequently used scripts, pre-load them at application startup.

---

### Q9. When should you NOT use Lua scripts?

**Interview Answer**

Avoid Lua scripts for simple atomic operations that can be done with single commands (`SETNX`, `INCR`, `GETDEL`). Avoid for operations that should not block other clients — if your script takes >1ms, reconsider. Avoid for logic that belongs in the application layer (business rules, validation). Avoid if you need rollback capabilities — Lua scripts have no undo. Avoid for operations that access multiple Redis instances (scripts are per-instance). Use application-level logic when the operation is simple enough that atomicity is guaranteed by a single Redis command.

---

### Q10. How do you test Lua scripts in Rust?

**Interview Answer**

Test Lua scripts against a real Redis instance using `testcontainers`:

```rust
#[tokio::test]
async fn test_safe_lock_release() {
    let redis = testcontainers::clients::Cli::default();
    let node = redis.run(testcontainers::images::redis::Redis::default()).await;
    let addr = format!("redis://127.0.0.1:{}", node.get_host_port(6379).await);
    let client = redis::Client::open(addr.as_str()).unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    // Acquire lock
    redis::cmd("SET").arg("lock:r1").arg("uuid-1").arg("NX").arg("EX").arg(30)
        .execute_async(&mut con).await.unwrap();

    let script = redis::Script::new(SAFE_RELEASE_SCRIPT);
    let result: bool = script.key("lock:r1").arg("uuid-1").invoke_async(&mut con).await.unwrap();
    assert!(result);

    // Try releasing with wrong UUID
    redis::cmd("SET").arg("lock:r1").arg("uuid-2").arg("EX").arg(30)
        .execute_async(&mut con).await.unwrap();
    let result: bool = script.key("lock:r1").arg("uuid-1").invoke_async(&mut con).await.unwrap();
    assert!(!result);
}
```

This validates script correctness with real Redis behavior.

