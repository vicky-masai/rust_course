# How Would You Design a Distributed Rate Limiter?

## Interview Question

How would you design a distributed rate limiter for a multi-service Rust backend?

## Interview Answer

I would design a centralized distributed rate limiter using Redis as the coordination layer, with each Axum service instance delegating rate limit checks to Redis via atomic Lua scripts. The system uses the sliding window counter algorithm for accurate rate limiting without excessive memory usage, and Redis Cluster provides horizontal scalability for the rate limit store. Each service embeds the rate limiter as a tower Layer, so it integrates seamlessly into the Axum middleware pipeline without requiring changes to business logic. The architecture supports per-user, per-API-key, and per-endpoint limits with configurable windows, and includes a local fallback that enforces conservative limits during Redis outages.

---

## Follow-up Questions & Answers

### Q1. How does the sliding window counter algorithm work in a distributed setting?

**Interview Answer**

The sliding window counter combines the previous window's count with the current window's count using a weighted formula: `count = prev_count * (elapsed_ratio) + current_count`. In Redis, I store two keys per window — one for the previous minute and one for the current minute — using `INCR` and `EXPIRE`. A Lua script reads both keys, calculates the weighted count, and decides whether to allow or reject the request, all atomically. This eliminates the burst problem of fixed windows (where a user could send 2x the limit at window boundaries) without storing every individual request timestamp like a sliding window log would. The memory overhead is just two integers per user per window.

---

### Q2. How do you handle rate limiting across multiple microservices sharing the same Redis?

**Interview Answer**

Each microservice uses the same Redis Cluster but with different key prefixes to avoid collisions — for example, `rl:auth:{user_id}` and `rl:search:{user_id}`. The Lua script is bundled in each service's codebase and loaded at startup via `redis.evalsha()`. I use Redis Cluster's hash tags to ensure related keys land on the same shard, preventing cross-shard transactions. For cross-service coordination (e.g., a user's total API calls across all services), I maintain a global counter key like `rl:global:{user_id}` that all services increment. The trade-off is that this global counter adds a Redis operation per request across all services, but it provides accurate cross-service limiting.

---

### Q3. What is the latency impact of centralized rate limiting in Rust?

**Interview Answer**

Every rate limit check requires a Redis round-trip, which adds 0.5-2ms on a local Redis instance and 5-20ms for a cross-region Redis. In Axum, I minimize this by using `fred`'s connection pooling (default 10 connections per instance) and pipelining multiple checks into a single network call when multiple rate limits apply to one request. For ultra-low-latency endpoints, I implement a client-side token bucket that allows N free requests per second before hitting Redis, reducing Redis load by 90%. The benchmarked overhead of the full middleware stack (Redis check + Lua script execution) is typically under 2ms p99 on a well-provisioned Redis Cluster, which is acceptable for most APIs.

---

### Q4. How do you prevent rate limiter bypass attacks?

**Interview Answer**

The rate limiter must sit in front of all entry points, including the API gateway or load balancer level, to prevent clients from bypassing the application layer. I implement rate limiting at two levels: L4 (NLB level) using AWS WAF for IP-based throttling, and L7 (Axum middleware) for user-based limiting. For key identification, I use HMAC-based API keys rather than simple IP addresses, making it harder for attackers to rotate through IPs. I also implement rate limit key obfuscation — the Redis key is a SHA-256 hash of the API key, preventing key enumeration. Additionally, I detect and block clients that consistently hit the rate limit (indicating automated abuse) by tracking rejection rates in a separate Redis sorted set.

---

### Q5. How do you implement rate limiting for different tiers (free, pro, enterprise)?

**Interview Answer**

I store tier-specific limits in a configuration table in PostgreSQL, loaded at startup into a `DashMap<Tier, RateLimitConfig>`. The Axum auth middleware extracts the user's tier from their JWT claims and attaches it to the request extensions. The rate limiter middleware then looks up the limit for that tier: free users get 100 requests/hour, pro users get 10,000 requests/hour, and enterprise users get unlimited (or 1,000,000). The tier is included in the Redis key, like `rl:tier:free:{user_id}:{window}`, so each tier's limits are tracked independently. Tier changes take effect immediately because the middleware reads the tier from the JWT on every request rather than caching it.

---

### Q6. How do you test a distributed rate limiter?

**Interview Answer**

I write integration tests that spin up a Redis Docker container using `testcontainers-rs` and multiple Axum instances against it. The test sends 100 concurrent requests from 10 parallel tasks, each using a known API key, and asserts that the total allowed requests match the configured limit within a small tolerance (due to sliding window approximation). I also test failure modes: killing Redis mid-test to verify the fallback behavior, and testing Lua script atomicity by sending requests that would race without atomicity. For the local fallback, I write unit tests that verify the in-memory token bucket rejects requests after the limit is exceeded. All tests use `tokio::test` with `#[flaky(3)]` annotations since distributed tests can have timing sensitivity.

---

### Q7. How do you handle rate limit configuration changes without downtime?

**Interview Answer**

Rate limit configurations are stored in PostgreSQL and loaded into an in-memory `Arc<RwLock<HashMap>>` at startup. When an operator updates a limit via an admin API or database migration, a PostgreSQL notification channel (`LISTEN/NOTIFY`) triggers a reload in all service instances. In Rust, I use `tokio::spawn` to run a background task that listens for notifications and updates the configuration under a write lock. The lock is held only briefly (microseconds) to swap the config reference. Alternatively, I use `arc-swap` crate to atomically swap the config pointer without any locking. This ensures zero-downtime configuration changes, and the admin API validates that new limits are sane before applying them.

---

### Q8. How do you implement rate limiting for WebSocket connections?

**Interview Answer**

WebSocket connections are long-lived, so per-request rate limiting doesn't apply. Instead, I rate limit the message throughput: I track the number of messages per connection per time window using Redis, with the WebSocket connection ID as the key. In Axum, the WebSocket handler checks the rate limit before forwarding each message to the backend. I use a token bucket algorithm here because messages tend to arrive in bursts (e.g., typing indicators), and a strict window would be too restrictive. If the message rate exceeds the limit, I send a WebSocket close frame with a custom code (4001) and a JSON payload explaining the rate limit, allowing the client to reconnect after the cooldown period.

---

### Q9. How do you scale Redis for billions of rate limit keys?

**Interview Answer**

At this scale, I deploy a Redis Cluster with 24+ shards, each holding approximately 16,384 hash slots. Rate limit keys use hash tags like `{user_id}:rl:{window}` to ensure all keys for one user land on the same shard, enabling atomic operations. I implement aggressive key expiry: each rate limit key has a TTL of 2 × the window duration, so old keys are automatically cleaned up. The memory estimate is 16 bytes per key + 8 bytes per value × 2 windows × N users. For 100 million users, that's roughly 4.8GB of Redis memory, easily fitting in 6 shards. I also implement key compression by using short key prefixes and integer user IDs instead of UUIDs, reducing memory by 40%.

---

### Q10. What are the trade-offs between different distributed rate limiting approaches?

**Interview Answer**

The three main approaches are: (1) Centralized Redis — accurate but adds network latency and is a single point of failure; (2) Local rate limiting with periodic sync — low latency but slightly inaccurate during sync intervals; (3) Sticky sessions — routes all requests from one client to the same server, enabling local rate limiting without coordination but creating load imbalance. I typically choose centralized Redis for correctness (most APIs need accurate limits) and mitigate latency with connection pooling and pipelining. For globally distributed systems, I use a hybrid approach: each region has its own Redis, with asynchronous cross-region replication for eventually consistent global limits. This gives sub-millisecond local checks with 99.9% global accuracy.
