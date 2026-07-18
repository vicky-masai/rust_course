# Rate Limiter Algorithms

## Interview Question

Explain different rate limiting algorithms and their tradeoffs for backend systems.

## Interview Answer

Rate limiting controls the rate of requests a client can make to a service. Four primary algorithms exist: **Token Bucket** — tokens are added at a fixed rate; each request consumes a token. Allows bursts up to bucket size. **Leaky Bucket** — requests enter a queue (bucket) and are processed at a fixed rate. Smooths out bursts. **Fixed Window Counter** — counts requests in fixed time windows (e.g., per second). Simple but allows boundary bursts. **Sliding Window Log** — timestamps of all requests in a window are stored; count those within the window. Most accurate but memory-intensive. The choice depends on: burst tolerance, memory budget, and accuracy requirements.

**Time Complexity**: O(1) for all algorithms per request
**Space Complexity**: O(1) to O(n) depending on algorithm

---

## Follow-up Questions & Answers

### Q1. How would you implement Token Bucket in Rust?

**Interview Answer**

```rust
use std::time::{Instant, Duration};

struct TokenBucket {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    fn new(max_tokens: f64, refill_rate: f64) -> Self {
        TokenBucket {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    fn allow(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;
    }
}
```

Token Bucket allows bursts (up to `max_tokens`) while maintaining a steady average rate. Used by: AWS API Gateway, Stripe API, and most cloud rate limiters.

---

### Q2. What is the difference between Token Bucket and Leaky Bucket?

**Interview Answer**

**Token Bucket**: Allows bursts (up to bucket capacity) while maintaining average rate. Tokens accumulate when idle, enabling catch-up. Best for: APIs that allow short bursts (e.g., burst of 10 requests then normal rate). **Leaky Bucket**: Processes requests at a fixed rate regardless of input rate. Requests queue up and are processed one at a time. If the queue is full, new requests are rejected. Best for: smoothing traffic to protect downstream services. **Key difference**: Token Bucket is more flexible (allows controlled bursts). Leaky Bucket is stricter (constant output rate). Most production systems use Token Bucket because real-world traffic is bursty.

---

### Q3. How does Sliding Window Log differ from Fixed Window?

**Interview Answer**

**Fixed Window**: Divide time into fixed intervals (e.g., 1-second windows). Count requests in the current window. Problem: allows 2× the rate at window boundaries (e.g., 100 requests at 11:59:00.9 and 100 at 11:59:01.0 = 200 in 1 second). **Sliding Window Log**: Store timestamps of all requests. When a new request arrives, count how many timestamps fall within the last N seconds. No boundary problem — the window slides smoothly. **Sliding Window Counter** (hybrid): Weight the previous window's count by how much it overlaps with the current window. Approximates sliding window with O(1) memory. Used by: Redis `MULTI` + `ZREMRANGEBYSCORE` for sliding window, Envoy proxy for rate limiting.

---

### Q4. How is rate limiting implemented in distributed systems?

**Interview Answer**

Distributed rate limiting requires synchronization across nodes. Approaches: **Centralized**: Use Redis as the rate limiter. Each request checks/increments a Redis key with TTL. Fast but single point of failure. **Local + Global**: Each node maintains local rate limiting (Token Bucket) and periodically syncs with a central counter. Allows temporary over-limiting but reduces latency. **Sticky sessions**: Route all requests from a client to the same node (via consistent hashing). Local rate limiting per node. No coordination needed. **Quorum-based**: Require approval from majority of nodes before allowing a request. Used in: Redis `INCR` + `EXPIRE` for simple rate limiting, Envoy's `rate_limit` filter for distributed limiting, and Istio's `quota` system for service mesh rate limiting.

---

### Q5. How would you implement rate limiting with Redis in Rust?

**Interview Answer**

```rust
use redis::Client;

async fn is_rate_limited(client: &Client, key: &str, limit: u64, window_secs: u64) -> bool {
    let mut conn = client.get_async_connection().await.unwrap();
    let now = chrono::Utc::now().timestamp_millis() as u64;
    let window_start = now - (window_secs * 1000);

    // Sliding window using sorted set
    redis::cmd("ZREMRANGEBYSCORE")
        .arg(key).arg(0).arg(window_start)
        .execute_async(&mut conn).await.unwrap();

    let count: u64 = redis::cmd("ZCARD")
        .arg(key).arg(now).arg(now).arg("NX")
        .execute_async(&mut conn).await.unwrap();

    if count >= limit { return true; }

    redis::cmd("ZADD")
        .arg(key).arg(now).arg(now)
        .execute_async(&mut conn).await.unwrap();
    redis::cmd("EXPIRE").arg(key).arg(window_secs)
        .execute_async(&mut conn).await.unwrap();

    false
}
```

This implements sliding window rate limiting using Redis sorted sets.

---

### Q6. What are the tradeoffs between rate limiting algorithms?

**Interview Answer**

| Algorithm | Memory | Accuracy | Burst | Complexity |
|-----------|--------|----------|-------|------------|
| Token Bucket | O(1) | Good | Allows | Low |
| Leaky Bucket | O(1) | Good | No | Low |
| Fixed Window | O(1) | Poor (boundary) | Allows | Very Low |
| Sliding Window Log | O(n) | Excellent | No | Medium |
| Sliding Window Counter | O(1) | Good | No | Medium |

**Choose Token Bucket** for APIs with bursty traffic. **Choose Sliding Window Counter** for accurate limiting with O(1) memory. **Choose Sliding Window Log** when exact accuracy matters (billing, quotas). **Choose Fixed Window** only for rough limiting where simplicity is paramount.

---

### Q7. How does rate limiting relate to circuit breakers?

**Interview Answer**

**Rate limiting** protects the service from too many requests (inbound protection). **Circuit breakers** protect the service from too many failures to downstream dependencies (outbound protection). They're complementary: rate limiting prevents overload by capping request rate; circuit breakers prevent cascade failures by stopping calls to failing services. In Rust backend systems: use `tower` middleware for rate limiting (Token Bucket per client) and circuit breaking (using `tower::limit::ConcurrencyLimit` or custom implementations). Production systems like Envoy proxy implement both: rate limits at the edge, circuit breakers for internal service calls.

---

### Q8. How do you handle rate limiting across multiple API endpoints?

**Interview Answer**

**Per-endpoint limiting**: Each endpoint has its own limit (e.g., 100 req/s for `/api/users`, 10 req/s for `/api/reports`). **Global limiting**: All endpoints share a single limit (e.g., 1000 total req/s per client). **Hierarchical**: Global limit + per-endpoint limits. A request must pass both. In Redis, use separate keys: `rate:{client_id}:{endpoint}` for per-endpoint and `rate:{client_id}:global` for global. Check both and reject if either is exceeded. **Token bucket with multiple levels**: Maintain separate buckets at each level. In Rust, use a `HashMap<String, TokenBucket>` for per-client-per-endpoint limiting, with a shared global bucket.

---

### Q9. How do you handle rate limiting for WebSocket connections?

**Interview Answer**

WebSocket connections are long-lived, so rate limiting applies differently: **Message rate limiting**: Limit messages per second per connection (Token Bucket per connection). **Connection rate limiting**: Limit new WebSocket connections per IP per minute (Fixed Window per IP). **Payload size limiting**: Limit message size to prevent memory exhaustion. **Fan-out limiting**: Limit how many channels a client can subscribe to. In Rust with `axum::ws`, implement rate limiting in the message handler using per-connection Token Buckets. Store connection state in `Extension` or a shared `DashMap<ConnectionId, TokenBucket>`. Reject messages when the bucket is empty and optionally close the connection.

---

### Q10. How do you test rate limiting implementations?

**Interview Answer**

**Unit tests**: Verify Token Bucket allows exactly N requests per window. Test burst behavior (send N+1 requests, expect last rejection). Test refill behavior (wait, then verify more requests allowed). **Load testing**: Use `wrk`, `hey`, or `k6` to send traffic at various rates. Verify the rate limiter rejects excess requests without affecting valid traffic. **Chaos testing**: Simulate Redis failures (for distributed rate limiting). Verify graceful degradation (either fail-open or fail-closed). **Property-based testing**: Use `proptest` in Rust to verify invariants (e.g., "never allows more than limit requests in window"). **Integration testing**: Deploy rate limiter with a real Redis instance and verify behavior under concurrent load using `tokio::spawn` with multiple tasks.
