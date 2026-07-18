# Design Rate Limiter

## Interview Question

How would you design a rate limiter for a distributed system?

## Interview Answer

I would design a rate limiter supporting four algorithms — token bucket, leaky bucket, fixed window, and sliding window — with a pluggable backend (Redis for distributed mode, in-memory for single-node). The system exposes a Rust library crate with a `RateLimiter` trait and implementations for each algorithm, plus an Axum middleware layer for easy integration. Redis provides centralized state using Lua scripts for atomic operations, ensuring consistency across multiple service instances. The rate limiter supports per-user, per-IP, and per-API-key limits with configurable time windows and burst allowances. It includes a fallback mechanism for Redis outages and exposes Prometheus metrics for monitoring rejection rates and throughput.

---

## Follow-up Questions & Answers

### Q1. Explain the token bucket algorithm and its use cases.

**Interview Answer**

The token bucket algorithm refills tokens at a fixed rate (e.g., 10 tokens/second) up to a maximum capacity (e.g., 100 tokens). Each request consumes one token. If the bucket is empty, the request is rejected. The bucket allows bursts up to the capacity — a user who hasn't made requests in a while gets a burst of up to 100 requests instantly. In Redis, I implement this with two keys: `bucket:{key}:tokens` (current token count) and `bucket:{key}:last_refill` (timestamp of last refill). A Lua script atomically calculates the tokens to add since the last refill, adds them (capped at max), and either decrements for an allowed request or rejects. Token bucket is ideal for APIs where burst tolerance is desired but sustained rate must be capped.

---

### Q2. How does the leaky bucket algorithm differ from token bucket?

**Interview Answer**

The leaky bucket algorithm processes requests at a constant rate, regardless of arrival rate. Requests enter a FIFO queue (the "bucket"), and are processed one at a time at a fixed interval (e.g., 1 request every 100ms). If the bucket is full, new requests are rejected. Unlike token bucket, there are no bursts — traffic is smoothed to a constant output rate. In Redis, I implement this with a sorted set where the score is the request timestamp. A Lua script checks the oldest request in the window and calculates whether the queue would overflow. Leaky bucket is ideal for scenarios requiring constant output rate, like network packet shaping or API rate smoothing where downstream services can't handle bursts.

---

### Q3. What is the difference between fixed window and sliding window algorithms?

**Interview Answer**

Fixed window divides time into fixed intervals (e.g., 1-minute windows: 00:00-01:00, 01:00-02:00) and counts requests per window. The problem is burst at window boundaries — a user can send 100 requests at 00:59 and 100 at 01:00, getting 200 requests in 2 seconds despite a 100/minute limit. Sliding window solves this by considering the actual time of each request within a rolling window. I implement the sliding window counter variant in Redis using two keys (previous window count and current window count) combined with a time-weighted formula. This eliminates boundary bursts with minimal memory overhead — just two integers per user instead of storing every timestamp.

---

### Q4. How do you implement rate limiting in Rust as a library crate?

**Interview Answer**

I structure the crate with a `RateLimiter` trait defining `async fn check(&self, key: &str) -> Result<RateLimitResult, RateLimitError>`. Implementations include `TokenBucket`, `SlidingWindowCounter`, and `FixedWindow`, each holding their configuration and backend connection. The Redis backend uses `fred` for async operations, and the in-memory backend uses `moka`. The crate exports an Axum `Layer` that wraps the inner service and applies rate limiting. I use generics: `RedisRateLimiter<L: Algorithm>` so the algorithm is compile-time configurable. The crate is published with feature flags: `redis` (default), `in-memory`, enabling users to choose their backend. Documentation includes examples for both standalone and Axum integration.

---

### Q5. How do you handle rate limiting across multiple regions?

**Interview Answer**

Global rate limiting requires cross-region coordination, which adds latency. I implement a two-tier approach: each region has a local Redis instance for fast, region-level rate limiting (sub-millisecond), and a global Redis (or DynamoDB) for cross-region limits (5-20ms latency). The local tier handles 90% of rate limit checks, and only cross-region limits require the global store. For consistency, I accept eventual consistency on the global tier — a user might briefly exceed their global limit during a cross-region sync delay (typically under 1 second). In Rust, I use `tokio::select!` to race the local and global checks, returning the local result immediately and asynchronously updating the global counter. This keeps the hot path fast.

---

### Q6. How do you test each rate limiting algorithm?

**Interview Answer**

I write parameterized tests using `rstest` that run the same test cases against all algorithm implementations. For token bucket, I test: (1) requests within capacity are allowed, (2) requests exceeding capacity are rejected, (3) tokens refill over time. For fixed window, I test: (1) boundary behavior at window transitions, (2) exact limit enforcement. For sliding window, I test: (1) weighted calculation accuracy, (2) boundary burst prevention. I use `tokio::time::pause()` to simulate time advancement without real delays, and `tokio::time::advance()` to fast-forward through windows. Redis tests use `testcontainers-rs` to spin up real Redis instances, ensuring Lua script compatibility. Coverage target is 95%+ for the library crate, verified with `cargo-tarpaulin`.

---

### Q7. How do you handle rate limiting for different HTTP methods?

**Interview Answer**

I configure separate limits per HTTP method, since GET requests are typically read-only and can have higher limits than POST/PUT/DELETE which modify state. The Axum middleware extracts the request method from `req.method()` and looks up the corresponding limit in a configuration map. For example: GET gets 1000 requests/minute, POST gets 100 requests/minute, DELETE gets 20 requests/minute. The Redis key includes the method: `rl:{method}:{user_id}:{window}`. I also implement path-based limiting, where specific endpoints like `/api/v1/admin` have stricter limits regardless of method. The configuration is stored in a TOML file loaded at startup, allowing operators to adjust limits without code changes.

---

### Q8. How do you implement rate limiting for gRPC services?

**Interview Answer**

gRPC uses HTTP/2, so the rate limiting middleware intercepts at the HTTP/2 layer. I implement a tonic interceptor that extracts the gRPC method name and the client's API key from metadata, then checks Redis using the same Lua script as the HTTP rate limiter. The key format includes the gRPC service and method: `rl:grpc:{service}:{method}:{client_id}:{window}`. For unary RPCs, the rate limit check happens before the handler executes. For streaming RPCs, I rate limit the number of messages per stream rather than per request. In Rust, tonic provides a `Interceptor` trait that I implement to perform the rate limit check, returning a `Status::RESOURCE_EXHAUSTED` with a retry-after hint when the limit is exceeded.

---

### Q9. How do you implement rate limiting that accounts for request cost?

**Interview Answer**

Not all requests have equal cost — a search query is more expensive than a status check. I implement a weighted rate limiter where each request has a cost (1 for simple reads, 5 for complex queries, 10 for writes). The token bucket tokens represent "cost units" rather than individual requests, and each request consumes tokens equal to its cost. In Rust, I define a `RequestCost` trait that implementations provide: `fn cost(&self) -> u32`. The Axum middleware calls this to determine the cost before checking the rate limiter. Redis stores the token count as a float to support fractional costs, and the Lua script subtracts the exact cost rather than a fixed 1 per request.

---

### Q10. How do you handle rate limiting during system overload?

**Interview Answer**

During overload, rate limiting alone isn't sufficient — I implement adaptive rate limiting that adjusts limits based on system health. A background Tokio task monitors CPU usage, memory, and error rates using `sysinfo` crate metrics. When CPU exceeds 80%, the rate limiter dynamically reduces all limits by 50%. When CPU exceeds 95%, it rejects all non-critical requests with 503. I implement this by wrapping the rate limiter configuration in an `Arc<RwLock<RateLimitConfig>>` that the background task updates. The adaptive behavior protects the system from cascading failure during traffic spikes. I also implement request priority — authenticated requests get higher priority than anonymous ones during overload, ensuring paying users maintain access.
