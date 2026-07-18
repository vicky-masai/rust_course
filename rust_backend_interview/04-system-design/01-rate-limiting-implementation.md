# Rate Limiting Implementation

## Interview Question

How would you implement rate limiting in a Rust backend system?

## Interview Answer

I would implement rate limiting using a Redis-backed sliding window algorithm integrated into an Axum middleware layer. The middleware intercepts every incoming request, extracts an API key or IP address, and checks Redis for the request count within the current time window using sorted sets. If the count exceeds the configured threshold, the server responds with HTTP 429 Too Many Requests and a Retry-After header. The implementation uses Tokio for async I/O so the Redis round-trip never blocks the runtime, and I structure it as a reusable tower Layer so it can be applied selectively to routes. For single-node deployments, I also offer an in-memory token bucket as a fallback to avoid the Redis dependency entirely.

---

## Follow-up Questions & Answers

### Q1. What rate limiting algorithms have you used and which do you prefer?

**Interview Answer**

I have worked with four main algorithms: fixed window, sliding window log, sliding window counter, and token bucket. The sliding window counter is my default choice because it balances accuracy with memory efficiency — it approximates the previous window's contribution and combines it with the current window's count, avoiding the burst problem of fixed windows without the memory overhead of storing every timestamp. Token bucket is preferable when you want to allow controlled bursts while enforcing an average rate, such as in API quota systems. In Axum, I implement token bucket by storing the bucket state in a Tokio Mutex protected struct shared across request handlers via `Extension`.

---

### Q2. How do you handle rate limiting across multiple application instances?

**Interview Answer**

When running multiple Axum instances behind a load balancer, in-memory counters are useless because each instance only sees its own traffic. I centralize the counter in Redis, using the `INCR` command with a TTL on the key to create an atomic sliding window. Each instance connects to the same Redis cluster through `fred`, a Tokio-native Redis client. The trade-off is that every request now incurs a Redis network hop, which adds 0.5–2ms of latency, but this is acceptable for correctness. To mitigate this, I batch-check rate limits for endpoints that receive high traffic by using Redis pipelines.

---

### Q3. How would you implement rate limiting in Rust with Axum middleware?

**Interview Answer**

I create a custom tower Layer and Service that wraps the inner Axum service. The Layer holds configuration like the Redis pool and the request limit, and the Service's `call` method extracts the client identifier from request headers, performs a Redis check, and either passes the request through or returns a 429 response. The key Rust detail is implementing `Clone` on the Layer so Axum can clone it per connection, and using `Pin<Box<dyn Future>>` or async traits for the service's future. I register the layer on the Axum Router using `.layer(RateLimitLayer::new(redis_pool, 100, Duration::from_secs(60)))`, making it composable and per-route configurable.

---

### Q4. What happens when Redis goes down while rate limiting is active?

**Interview Answer**

This is a critical failure mode. I implement a fail-closed strategy by default — if Redis is unreachable, all requests are rejected with 503 Service Unavailable, which protects upstream services from a sudden flood. However, for less critical endpoints, I offer a configurable fail-open mode that allows traffic through but logs an alert. In Rust, I detect Redis failures by setting a short connect timeout (e.g., 50ms) on the `fred` client and catching connection errors in the middleware. I also deploy a local in-memory fallback that enforces a stricter rate limit during Redis outages, giving partial protection without centralization.

---

### Q5. How do you prevent race conditions in a distributed sliding window?

**Interview Answer**

Race conditions occur when two instances simultaneously read a count of 99 (under the limit of 100) and both allow the request. I prevent this using Redis Lua scripts, which execute atomically on the server side. The script reads the sorted set count, checks it against the limit, and either adds the new request or rejects it — all in a single atomic operation. Alternatively, I use the `MULTI/EXEC` transaction pipeline, but Lua scripts are preferable because they avoid the read-modify-write race window entirely. In the Axum middleware, I call the Lua script via `redis.eval()` and handle the script's return value to decide the response.

---

### Q6. How do you test rate limiting in Rust?

**Interview Answer**

I write integration tests using `tokio::test` with a real or embedded Redis instance (via the `redis-test` crate). The test creates an Axum test server with the rate limiting layer configured to allow 5 requests per second, then fires 10 requests using `reqwest` and asserts that the first 5 return 200 while the remaining 5 return 429. I also verify the `Retry-After` header is present and accurate. For unit testing the algorithm logic in isolation, I mock Redis responses with `mockall` and test the sliding window calculation independently from the HTTP layer.

---

### Q7. How do you rate-limit different API endpoints with different thresholds?

**Interview Answer**

I use a configuration struct that maps endpoint patterns or route names to their rate limit settings. For example, public search endpoints might get 30 requests per minute while authenticated endpoints get 600. The Axum middleware extracts the matched route from the request extensions and looks up the corresponding limit in an `Arc<HashMap<String, RateLimitConfig>>`. In Rust, I structure this as a `HashMap` populated at startup from a TOML or YAML config file. This approach lets operators adjust limits without redeploying code, and the `Arc` wrapper ensures the lookup is lock-free in the hot path.

---

### Q8. How do you handle rate limiting for authenticated vs. unauthenticated users?

**Interview Answer**

For authenticated users, the rate limit key is the user ID or API key extracted from a JWT or session token, which I get from an Axum `Extension` set by auth middleware. For unauthenticated users, I fall back to the client IP address, but I also account for proxied requests by checking the `X-Forwarded-For` header with a trusted proxy list to prevent IP spoofing. Each category can have different limits — unauthenticated requests are more restrictive (e.g., 20 per minute) while authenticated users get higher limits (e.g., 1000 per minute). The Redis key format reflects this: `ratelimit:auth:{user_id}:{window}` vs. `ratelimit:anon:{ip}:{window}`.

---

### Q9. How do you implement rate limiting that scales to millions of requests per second?

**Interview Answer**

At extreme scale, a single Redis instance becomes a bottleneck. I shard rate limit keys across a Redis Cluster using consistent hashing, so each shard handles a fraction of the traffic. The Redis Cluster's built-in slot mechanism distributes keys automatically. On the Rust side, I use connection pooling with the `fred` client and enable pipelining to batch multiple rate limit checks into a single network round-trip. I also implement client-side token buckets as a first-pass filter, reducing unnecessary Redis calls. For globally distributed systems, I replicate Redis to each region and accept slight inconsistency in exchange for sub-millisecond local checks.

---

### Q10. What metrics do you track for rate limiting?

**Interview Answer**

I track three primary metrics using `metrics` crate in Rust and export them to Prometheus: the total number of requests allowed, the total rejected (tagged by endpoint and reason), and the current rate limit headroom (how close each key is to its limit). I also monitor Redis latency for rate limit checks, since spikes indicate a potential bottleneck. The 429 responses include structured headers (`X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`) so clients can proactively throttle. In Grafana, I set up alerts for when rejection rates exceed 5% of total traffic, which may indicate a misconfigured client or a potential DDoS.
