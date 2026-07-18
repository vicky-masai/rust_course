# Rate Limiting for Security

## Interview Question

How do you implement rate limiting in a Rust Axum backend to protect against brute force and abuse?

## Interview Answer

Rate limiting restricts the number of requests a client can make within a time window, protecting against brute force attacks, credential stuffing, and API abuse. In a Rust Axum backend, I implement rate limiting using tower::limit::RateLimitLayer for simple per-IP limits, Redis-backed sliding window counters for distributed rate limiting, and tiered limits for different endpoints (stricter for auth endpoints, relaxed for read endpoints). The sliding window algorithm tracks requests in a Redis sorted set, providing smooth rate limiting without the burst issues of fixed windows. Rate limits should return HTTP 429 Too Many Requests with Retry-After headers so clients can back off gracefully.

---

## Follow-up Questions & Answers

### Q1. What is the difference between fixed window and sliding window rate limiting?

**Interview Answer**

Fixed window divides time into discrete windows (e.g., 1-minute intervals) and counts requests per window. A client could send 100 requests at 11:59:59 and 100 more at 12:00:00, getting 200 requests in 2 seconds despite a 100/minute limit. Sliding window tracks requests in a rolling time period, preventing this boundary burst. It stores timestamps of each request and counts those within the last N seconds. Sliding window is more accurate but uses more memory. In Redis, use sorted sets with timestamps for efficient sliding window implementation.

---

### Q2. How do you implement rate limiting with Redis in Rust?

**Interview Answer**

Use the rate-limiter algorithm with Redis sorted sets: for each request, add the current timestamp to a sorted set with the client's IP as the key, remove entries older than the window, and count remaining entries. If the count exceeds the limit, reject with 429. In Rust, use the redis crate with Lua scripts for atomic operations. Example: Redis EVAL script removes old entries, counts remaining, and adds the new entry atomically. Use tower middleware to apply this to Axum routes. The Redis approach works across multiple server instances, providing distributed rate limiting.

---

### Q3. How do you handle rate limiting for authenticated vs unauthenticated users?

**Interview Answer**

Unauthenticated users are rate-limited by IP address, with stricter limits (e.g., 10 requests/minute for login). Authenticated users are rate-limited by user ID, with more generous limits (e.g., 100 requests/minute for API endpoints). This prevents brute force attacks on login while allowing legitimate usage. In Axum, use a middleware that checks for an authenticated user context — if present, rate limit by user ID; if not, rate limit by IP. For the login endpoint specifically, use a very strict limit (5 attempts/minute) regardless of authentication status.

---

### Q4. What should the rate limit response include?

**Interview Answer**

Return HTTP 429 Too Many Requests with headers: Retry-After (seconds until the client can retry), X-RateLimit-Limit (maximum requests per window), X-RateLimit-Remaining (requests remaining), and X-RateLimit-Reset (UTC time when the window resets). The response body should be a JSON error with a descriptive message. In Axum, implement this as a custom IntoResponse for the rate limit error. The Retry-After header is critical — well-behaved clients will wait before retrying. Logging rate limit violations helps detect attack patterns.

---

### Q5. How do you protect authentication endpoints specifically?

**Interview Answer**

Authentication endpoints (login, password reset, MFA verification) need aggressive rate limiting because they are prime targets for brute force. Implement per-IP limits (5 attempts/minute), per-account limits (10 attempts/hour per email), progressive delays (increase delay after each failure), and account lockout after repeated failures (lock for 15 minutes). Use a combination of IP-based and account-based limiting. In Rust, track both IP and account in Redis with separate keys. After lockout, send an email notification to the user. Never reveal whether an email exists in the system.

---

### Q6. How do you implement token bucket rate limiting?

**Interview Answer**

The token bucket algorithm allows burst traffic while maintaining average rate. The bucket has a capacity (max burst size) and fills at a fixed rate (tokens per second). Each request consumes one token; if no tokens remain, the request is rejected. This allows legitimate burst traffic (loading a page with many API calls) while preventing sustained abuse. In Rust, implement using a Semaphore with permits and a refill task that adds permits at the configured rate. For distributed rate limiting with Redis, use a Lua script that atomically checks and decrements tokens.

---

### Q7. How do you handle rate limiting in a load-balanced environment?

**Interview Answer**

In a load-balanced environment, rate limiting must be distributed — otherwise, a client hitting different servers gets separate limits. Use Redis as the central rate limit store, with all servers reading and writing to the same Redis keys. The atomicity of Redis operations ensures accurate counting across servers. Without Redis, implement consistent hashing to route the same client to the same server, but this is fragile. Another approach is using a dedicated rate-limiting service (like Kong or Envoy) at the edge. For Rust backends, Redis-backed rate limiting is the most reliable solution.

---

### Q8. How do you rate limit by different criteria (IP, user, API key)?

**Interview Answer**

Use composite rate limit keys that combine the criteria: rate_limit:{ip}:{endpoint} for IP-based, rate_limit:{user_id}:{endpoint} for user-based, rate_limit:{api_key}:{endpoint} for API key-based. Apply multiple limits simultaneously — an API call might be limited by both IP (100/min) and API key (1000/min), with the stricter limit winning. In Rust, create a RateLimitKey struct that generates the appropriate Redis key based on the request context. The middleware checks all applicable limits and rejects with 429 if any limit is exceeded.

---

### Q9. How do you handle legitimate traffic spikes without blocking real users?

**Interview Answer**

Use tiered rate limits based on user tier (free vs paid), implement burst allowances (token bucket allows temporary spikes), add a Retry-After header so clients back off gracefully, and use adaptive rate limiting that adjusts based on server load. Monitor rate limit rejection rates — high rejection rates may indicate either an attack or a misconfigured limit. In Rust, implement circuit breakers that reduce the rate limit threshold when the server is overloaded. For critical APIs, use a sliding window with a generous burst limit that accommodates normal usage patterns while catching abuse.

---

### Q10. What is the relationship between rate limiting and DDoS protection?

**Interview Answer**

Rate limiting is a first line of defense against application-layer (Layer 7) DDoS attacks, limiting how many requests a single IP or user can make. However, rate limiting alone cannot stop volumetric DDoS attacks (Layer 3/4) that flood the network. DDoS protection requires: CDN-level filtering (Cloudflare, AWS Shield), network-level rate limiting at the load balancer, geographic filtering, and application-level rate limiting. In a Rust backend, rate limiting at the Axum layer catches application abuse, while upstream infrastructure handles volumetric attacks. Rate limiting is one component of a layered DDoS defense strategy.
