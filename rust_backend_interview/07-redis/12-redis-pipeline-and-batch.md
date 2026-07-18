# Redis Pipelining and Batch Operations

## Interview Question

Explain Redis pipelining, how it improves performance, and how to implement it in a Rust backend.

## Interview Answer

Pipelining sends multiple Redis commands in a single network round-trip without waiting for each response individually. Without pipelining, each command requires a round-trip (send command → wait → receive response → send next), costing ~0.5-2ms per command due to network latency. With pipelining, the client sends all commands at once and reads all responses at once, reducing overhead from N round-trips to 1. In Rust, `redis-rs` supports pipelining via `pipe()` which returns a `Pipeline` object — chain commands with `.cmd()` and execute with `.query_async()`. For large batches (10k+ commands), chunk pipelines to avoid memory pressure.

---

## Follow-up Questions & Answers

### Q1. How does pipelining differ from transactions (MULTI/EXEC)?

**Interview Answer**

Pipelining batches commands for network efficiency — commands are sent together but executed immediately in order, with no atomicity guarantee between them. Transactions (`MULTI`/`EXEC`) guarantee atomic execution — commands in a transaction block are queued and executed as a unit, with no interleaving from other clients, but there is no rollback. Pipelining improves throughput; transactions provide atomicity. You can combine both: pipeline `MULTI` → commands → `EXEC` for atomic batch execution with network efficiency.

---

### Q2. How do you implement pipelining in Rust with `redis-rs`?

**Interview Answer**

```rust
use redis::pipe;

async fn batch_get(con: &mut redis::aio::Connection, keys: &[String]) -> redis::RedisResult<Vec<Option<String>>> {
    let mut pipeline = pipe();
    for key in keys {
        pipeline.get(key);
    }
    pipeline.query_async(con).await
}

async fn batch_set(con: &mut redis::aio::Connection, pairs: &[(String, String)], ttl: u64) -> redis::RedisResult<()> {
    let mut pipeline = pipe();
    for (key, value) in pairs {
        pipeline.set_ex(key, value, ttl);
    }
    pipeline.query_async(con).await
}
```

The `pipe()` function creates a pipeline. Commands are buffered and sent in one batch when `query_async` is called. Responses come back as a `Vec<T>` in the same order as the commands.

---

### Q3. What is the performance difference between pipelined and non-pipelined commands?

**Interview Answer**

Without pipelining, each command costs one network round-trip (~0.5ms local, ~2-5ms cross-datacenter). For 100 commands: 100 × 1ms = 100ms. With pipelining: 1ms for all 100 commands sent + 1ms for all responses received = ~2ms total. That's a 50x improvement. Redis can handle 1M+ ops/sec with pipelining vs 100k ops/sec without. The bottleneck shifts from network latency to Redis processing time. For database queries (e.g., fetching 100 user records), pipelining is essential.

---

### Q4. How do you handle errors in pipelined commands?

**Interview Answer**

By default, pipelined commands are atomic — if any command fails, the entire pipeline returns an error. To handle individual command errors, use `.ignore()` on the pipeline to skip failed commands:

```rust
let result: (Option<String>, Option<String>) = pipe()
    .get("existing_key")
    .get("nonexistent_key")
    .ignore()
    .query_async(&mut con).await?;
```

`.ignore()` prevents errors from propagating and returns default values (`None`) for failed commands. For mixed pipelines where some commands may fail (e.g., `SET` on some keys, `DEL` on others), use `.ignore()` on commands that may fail and check results individually.

---

### Q5. When should you NOT use pipelining?

**Interview Answer**

Avoid pipelining for: (1) Single commands (no benefit). (2) Commands that depend on previous results (e.g., use the result of `GET` to decide the next `SET` — you need the response before deciding). (3) Very large batches (100k+ commands) which can cause Redis to buffer excessive data and increase latency for other clients — chunk into smaller batches. (4) Time-sensitive operations where you need immediate responses. (5) Mixed read/write workflows where reads must complete before writes. Use pipelining for independent batch reads, batch writes, and bulk cache operations.

---

### Q6. How do you implement batch operations for cache warming?

**Interview Answer**

Use pipelining to pre-populate the cache at startup:

```rust
async fn warm_cache(con: &mut redis::aio::Connection, users: &[User]) -> redis::RedisResult<()> {
    let mut pipeline = pipe();
    for user in users {
        let json = serde_json::to_string(user)?;
        pipeline.set_ex(format!("user:{}", user.id), &json, 300);
    }
    pipeline.query_async(con).await
}
```

For 10,000 users, this sends all `SET` commands in one round-trip instead of 10,000. Chunk into batches of 1,000-5,000 to avoid memory pressure:

```rust
for chunk in users.chunks(1000) {
    let mut pipeline = pipe();
    for user in chunk {
        pipeline.set_ex(&key, &json, 300);
    }
    pipeline.query_async(con).await?;
}
```

---

### Q7. What is the difference between pipelining and `MGET`/`MSET`?

**Interview Answer**

`MGET key1 key2 key3` fetches multiple keys in one command — the most efficient way to batch reads. `MSET key1 val1 key2 val2` sets multiple keys in one command. Pipelining sends multiple individual `GET`/`SET` commands in one round-trip. `MGET`/`MSET` are single commands that Redis executes internally as batch operations, while pipelining is a client-side optimization for sending multiple commands together. Use `MGET`/`MSET` for simpler cases; use pipelining when you need mixed command types (GET + HSET + ZADD in one batch).

---

### Q8. How do you use `SCAN` efficiently with pipelining?

**Interview Answer**

`SCAN` iterates keys incrementally without blocking Redis. Combine with pipelining for efficient key scanning:

```rust
async fn scan_all_keys(con: &mut redis::aio::Connection) -> redis::RedisResult<Vec<String>> {
    let mut cursor = 0;
    let mut all_keys = Vec::new();
    loop {
        let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor).arg("COUNT").arg(1000)
            .query_async(con).await?;
        all_keys.extend(keys);
        cursor = next_cursor;
        if cursor == 0 { break; }
    }
    Ok(all_keys)
}
```

For bulk processing, pipeline the SCAN with the operation on each key:

```rust
for chunk in all_keys.chunks(500) {
    let mut pipeline = pipe();
    for key in chunk {
        pipeline.get(key);
    }
    let values: Vec<Option<String>> = pipeline.query_async(con).await?;
    // Process values...
}
```

---

### Q9. How do you handle large batch operations without blocking Redis?

**Interview Answer**

For large batches (millions of keys): (1) Chunk pipelines into 1,000-5,000 commands to avoid buffer bloat. (2) Add `tokio::time::sleep(Duration::from_millis(10))` between chunks to let Redis process other clients. (3) Use `SCAN` instead of `KEYS` for iteration. (4) Monitor Redis with `INFO clients` to check `blocked_clients` and `client_recent_max_output_buffer_size`. (5) Use `CLIENT NO-EVICT` for the batch client to prevent disconnection. (6) Schedule large batches during low-traffic periods. In Rust, use `tokio::spawn_blocking` for the sleep to avoid blocking the async runtime.

---

### Q10. How do you benchmark Redis pipelining performance?

**Interview Answer**

Use `redis-benchmark` (Redis built-in tool):

```bash
redis-benchmark -t set,get -n 1000000 -c 50 -P 100
```

The `-P 100` flag enables pipelining with 100 commands per batch. Compare with `-P 1` (no pipelining) to see the difference. For Rust-specific benchmarking, use `criterion`:

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_pipelined(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();

    c.bench_function("100_gets_pipelined", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut con = client.get_async_connection().await.unwrap();
                let mut pipeline = pipe();
                for i in 0..100 {
                    pipeline.get(format!("key:{}", i));
                }
                pipeline.query_async::<_, Vec<Option<String>>>(&mut con).await.unwrap();
            });
        });
    });
}
```

Compare pipelined vs individual `GET` to quantify the improvement. Typically 10-50x throughput gain.

