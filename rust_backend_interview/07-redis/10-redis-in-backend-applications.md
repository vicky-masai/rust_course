# Redis in Backend Applications

## Interview Question

How would you use Redis in a Rust backend for session management, rate limiting, and idempotency keys?

## Interview Answer

Redis is ideal for session management because sessions require fast reads/writes, automatic expiration, and shared access across multiple app instances — store session tokens as Redis keys with TTL matching session duration. Rate limiting uses Redis atomic counters (`INCR` + `EXPIRE`) with sliding window or token bucket algorithms to enforce per-user or per-IP request limits across distributed instances. Idempotency keys use `SET key uuid NX EX` to prevent duplicate processing of the same API request — the key is set before processing and checked to prevent re-execution. In Rust, `redis-rs` with async Tokio support provides the performance and ergonomics needed for all three patterns.

---

## Follow-up Questions & Answers

### Q1. How do you implement session management with Redis in Rust?

**Interview Answer**

Store sessions as Redis hash keys with TTL:

```rust
use redis::AsyncCommands;

async fn create_session(con: &mut redis::aio::Connection, user_id: i64, token: &str) -> redis::RedisResult<()> {
    let key = format!("session:{}", token);
    con.hset_multiple(&key, &[
        ("user_id", user_id.to_string()),
        ("created_at", chrono::Utc::now().to_rfc3339()),
    ]).await?;
    con.expire(&key, 86400).await?; // 24-hour TTL
    Ok(())
}

async fn validate_session(con: &mut redis::aio::Connection, token: &str) -> redis::RedisResult<Option<String>> {
    let key = format!("session:{}", token);
    con.hget(&key, "user_id").await
}
```

On logout, `DEL session:{token}`. The TTL auto-expires stale sessions. Use a connection pool (`RedisConnectionManager`) for concurrent requests.

---

### Q2. How do you implement a sliding window rate limiter?

**Interview Answer**

Use Redis sorted sets with timestamps as scores:

```rust
async fn check_rate_limit(con: &mut redis::aio::Connection, user_id: &str, limit: u64, window_secs: i64) -> redis::RedisResult<bool> {
    let key = format!("ratelimit:{}", user_id);
    let now = chrono::Utc::now().timestamp_millis();
    let window_start = now - (window_secs * 1000);

    // Remove expired entries
    redis::cmd("ZREMRANGEBYSCORE").arg(&key).arg(0).arg(window_start).execute_async(con).await;

    // Count current window requests
    let count: u64 = redis::cmd("ZCARD").arg(&key).query_async(con).await?;

    if count >= limit {
        return Ok(false); // rate limited
    }

    // Add current request
    redis::cmd("ZADD").arg(&key).arg(now).arg(format!("{}:{}", now, user_id)).execute_async(con).await;
    redis::cmd("EXPIRE").arg(&key).arg(window_secs).execute_async(con).await;
    Ok(true)
}
```

This gives precise sliding window counting with automatic cleanup via TTL.

---

### Q3. How do you implement idempotency keys?

**Interview Answer**

Generate a unique idempotency key per API request (e.g., `uuid::Uuid::new_v4()`). Before processing, attempt to set the key with `SET idem:{key} processing NX EX 300`:

```rust
async fn process_idempotent(con: &mut redis::aio::Connection, idem_key: &str) -> redis::RedisResult<bool> {
    let result: Option<String> = redis::cmd("SET")
        .arg(format!("idem:{}", idem_key))
        .arg("processing")
        .arg("NX").arg("EX").arg(300)
        .query_async(con).await?;

    match result {
        Some(_) => Ok(true),  // Lock acquired, process the request
        None => Ok(false),    // Duplicate request, skip processing
    }
}
```

After processing, store the result: `SET idem:{key} {result_json} EX 300`. On duplicate, return the cached result. Use `DEL idem:{key}` to clean up.

---

### Q4. How do you handle Redis connection pooling in Rust?

**Interview Answer**

Use `redis-rs`'s `RedisConnectionManager` with `bb8` or `deadpool` for connection pooling:

```rust
use redis::aio::ConnectionManager;
use redis::Client;

let client = Client::open("redis://127.0.0.1:6379")?;
let mut manager = ConnectionManager::new(client).await?;

// Each .get() call returns a managed connection that auto-reconnects
let mut con = manager.clone().get_async_connection().await?;
con.set("key", "value").await?;
```

`ConnectionManager` handles reconnection transparently. For higher concurrency, use `bb8::Pool` with `RedisConnectionManager`:

```rust
use bb8::{Pool, RunError};
use redis::RedisConnectionManager;

let pool = Pool::builder()
    .max_size(15)
    .build(RedisConnectionManager::new("redis://127.0.0.1:6379")?)
    .await?;

let mut con = pool.get().await?;
```

---

### Q5. How do you handle Redis connection failures gracefully?

**Interview Answer**

Never let Redis failures cascade into application failures. Implement graceful degradation: (1) Cache-Aside: if Redis `GET` fails, fall back to PostgreSQL. (2) Rate limiting: if Redis fails, either allow the request (fail open) or block it (fail closed) depending on business requirements. (3) Sessions: if Redis is down, reject authentication or fall back to JWT validation. In Rust, use `match` on Redis results and log errors:

```rust
match con.get::<_, Option<String>>(&key).await {
    Ok(Some(val)) => return Ok(serde_json::from_str(&val)?),
    Ok(None) => {},
    Err(e) => tracing::warn!("Redis unavailable: {}", e),
}
// Fall back to database
```

Use circuit breakers (e.g., `tower::ServiceBuilder` with `concurrency_limit`) to prevent cascading failures.

---

### Q6. How do you use Redis for distributed session scaling?

**Interview Answer**

When running multiple Rust backend instances behind a load balancer, sessions must be shared. Store all sessions in Redis — any instance can validate any session token:

```rust
// Middleware
let session_token = extract_token_from_request(&req);
let user_id: Option<String> = con.get(&format!("session:{}", session_token)).await.unwrap_or(None);

match user_id {
    Some(uid) => {
        // Session valid, proceed
    }
    None => {
        // Session invalid or expired, return 401
    }
}
```

This scales horizontally — add more app instances without sticky sessions. Use `SET session:{token} {user_id} EX 86400` on login and `DEL session:{token}` on logout.

---

### Q7. How do you implement token bucket rate limiting with Redis?

**Interview Answer**

Token bucket is more flexible than sliding window — it allows bursts while maintaining a steady rate:

```rust
async fn token_bucket(con: &mut redis::aio::Connection, key: &str, capacity: u64, refill_rate: f64) -> redis::RedisResult<bool> {
    let script = redis::Script::new(r#"
        local tokens = tonumber(redis.call('HGET', KEYS[1], 'tokens')) or ARGV[1]
        local last = tonumber(redis.call('HGET', KEYS[1], 'last')) or ARGV[2]
        local now = tonumber(ARGV[2])
        local delta = math.max(0, now - last)
        tokens = math.min(ARGV[1], tokens + delta * ARGV[3])
        if tokens >= 1 then
            tokens = tokens - 1
            redis.call('HSET', KEYS[1], 'tokens', tokens, 'last', now)
            redis.call('EXPIRE', KEYS[1], math.ceil(ARGV[1] / ARGV[3]))
            return 1
        else
            return 0
        end
    "#);

    let result: bool = script.key(key).arg(capacity).arg(chrono::Utc::now().timestamp_millis()).arg(refill_rate)
        .invoke_async(con).await?;
    Ok(result)
}
```

This uses a Lua script for atomicity, refilling tokens based on elapsed time.

---

### Q8. How do you use Redis for distributed counting (e.g., page views)?

**Interview Answer**

Use `INCR` for atomic counters:

```rust
async fn increment_page_view(con: &mut redis::aio::Connection, page_id: &str) -> redis::RedisResult<i64> {
    let key = format!("views:{}", page_id);
    let count: i64 = redis::cmd("INCR").arg(&key).query_async(con).await?;
    // Set TTL only on first view to auto-cleanup
    if count == 1 {
        redis::cmd("EXPIRE").arg(&key).arg(86400).execute_async(con).await;
    }
    Ok(count)
}
```

For high-frequency counting, use `INCRBY` with periodic batch flushes, or Redis HyperLogLog (`PFADD`) for unique counts. Use `MGET` to fetch multiple counters at once for dashboard pages.

---

### Q9. How do you secure Redis in production?

**Interview Answer**

(1) Set a password via `requirepass` in `redis.conf` and authenticate with `AUTH password`. (2) Bind to specific interfaces via `bind 127.0.0.1`. (3) Disable dangerous commands via `rename-command FLUSHALL ""`, `rename-command CONFIG ""`. (4) Use TLS for encrypted connections (Redis 6.0+ with `tls-port`). (5) Set `maxmemory` to prevent OOM. (6) Run Redis as a non-root user. (7) Use firewall rules to restrict access. In Rust, authenticate with `redis::cmd("AUTH").arg(password)` or use the URL format `redis://:password@host:port`.

---

### Q10. How do you test Redis-dependent code in Rust?

**Interview Answer**

Use an embedded Redis for integration tests (e.g., `redis-test` crate or `testcontainers` with a Redis container):

```rust
#[tokio::test]
async fn test_rate_limit() {
    let redis = testcontainers::clients::Cli::default();
    let node = redis.run(testcontainers::images::redis::Redis::default()).await;
    let host_addr = format!("redis://127.0.0.1:{}", node.get_host_port(6379).await);
    let client = redis::Client::open(host_addr.as_str()).unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    assert!(check_rate_limit(&mut con, "user:1", 5, 60).await.unwrap());
    // Simulate 5 requests
    for _ in 0..5 {
        check_rate_limit(&mut con, "user:1", 5, 60).await.unwrap();
    }
    assert!(!check_rate_limit(&mut con, "user:1", 5, 60).await.unwrap());
}
```

This gives real Redis behavior without mocking, ensuring correctness of Lua scripts, TTLs, and atomic operations.

