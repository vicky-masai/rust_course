# How Would You Design a High-Performance API?

## Interview Question

How would you design a high-performance API in Rust?

## Interview Answer

I would build the API using Axum on top of Tokio, leveraging Rust's zero-cost abstractions and fearless concurrency to achieve high throughput. The architecture includes connection pooling via `sqlx` for PostgreSQL, Redis caching with `fred` for frequently accessed data, and efficient database indexing to minimize query latency. I would deploy the service behind a load balancer (like Nginx or an ALB) with multiple stateless Axum instances for horizontal scaling, and use Kafka for async processing of heavy tasks like email sending or report generation. Observability is built in from day one with structured logging via `tracing`, metrics export to Prometheus, and distributed tracing with OpenTelemetry. Compression (gzip/brotli), response caching, and pagination round out the performance strategy.

---

## Follow-up Questions & Answers

### Q1. Why did you choose Axum over Actix-web or Rocket?

**Interview Answer**

Axum is built by the Tokio team and uses tower Service and tower Layer abstractions, which means middleware and extensions compose in a type-safe, ergonomic way. It leverages `tokio::spawn` natively, so every request handler runs on the multi-threaded Tokio runtime with minimal overhead. Actix-web is faster in raw benchmarks but uses its own actor model, which adds conceptual complexity. Rocket is more opinionated and has a heavier macro system. Axum's design philosophy of "no magic" makes it easier to debug and extend, and its ecosystem of tower-compatible middleware (rate limiting, tracing, auth) integrates seamlessly with the rest of the Rust async ecosystem.

---

### Q2. How do you implement connection pooling for PostgreSQL in Rust?

**Interview Answer**

I use `sqlx::PgPool`, which creates a pool of async PostgreSQL connections managed by Tokio. At startup, I configure the pool with `PgPoolOptions::new().max_connections(20).connect(&database_url)`, where the max is tuned based on the database's `max_connections` setting and the number of application instances. Each handler acquires a connection from the pool using `pool.acquire().await?` or uses `sqlx::query!` macros for compile-time checked queries. The pool automatically handles connection reuse, health checks, and reconnection on failure. I also use `sqlx::Acquire` for transactional operations to ensure connections are returned to the pool even if a query fails mid-transaction.

---

### Q3. How do you structure Redis caching to maximize hit rates?

**Interview Answer**

I implement a cache-aside pattern where the application first checks Redis before querying PostgreSQL. The cache key is derived from a deterministic hash of the query parameters — for example, `cache:user:{user_id}` or `cache:products:category:{cat}:page:{pg}`. I set TTLs based on data volatility: user profiles get 5-minute TTLs, product catalogs get 15 minutes, and static configuration gets 1 hour. For Rust implementation, I use the `fred` crate's async API and handle cache misses by querying PostgreSQL, writing the result to Redis with `SET EX`, and returning it. I also implement cache warming for critical paths by preloading popular data into Redis on startup using a background Tokio task.

---

### Q4. What strategies do you use to minimize latency at the application layer?

**Interview Answer**

I use several strategies: first, I enable response compression with `tower-http::CompressionLayer` to reduce payload sizes by 60-80% for JSON responses. Second, I implement request deduplication for identical concurrent requests using a `DashMap` of in-flight Tokio JoinHandles, so a flood of identical queries only hits the database once. Third, I use `tokio::select!` to implement timeouts on downstream calls, returning cached fallback data if a dependency is slow. Fourth, I apply efficient serialization with `simd-json` or `serde_json` with `#[serde(rename_all)]` to minimize JSON size. Finally, I use `tower::buffer` to bound concurrency and prevent resource exhaustion under load.

---

### Q5. How do you handle database query performance?

**Interview Answer**

I start with proper indexing: composite indexes for multi-column WHERE clauses, partial indexes for filtered queries, and covering indexes to avoid table lookups. In Rust with `sqlx`, I use the `EXPLAIN ANALYZE` output to validate query plans and add indexes where sequential scans appear. I implement cursor-based pagination instead of OFFSET/LIMIT to avoid the O(n) scan problem — the cursor is an encrypted token containing the last seen ID and timestamp. For hot-path queries, I use prepared statements via `sqlx::query_as::<_, Model>()` which `sqlx` caches at the connection level. I also implement read replicas for read-heavy workloads, routing SELECT queries to replicas while keeping writes on the primary.

---

### Q6. How do you design the API for versioning and backward compatibility?

**Interview Answer**

I use URL path versioning (`/api/v1/users`, `/api/v2/users`) because it's explicit and easy to route at the load balancer level. In Axum, I create separate Router instances for each version, allowing independent handler evolution. For response compatibility, I use `#[serde(skip_serializing_optionals)]` and `#[serde(default)]` attributes to ensure new fields don't break old clients. When deprecating endpoints, I return the `Sunset` HTTP header with a date and the `Deprecation: true` header, giving clients time to migrate. I never remove old versions until analytics confirm zero traffic, and I document version lifecycles in an OpenAPI spec that I generate with `utoipa`.

---

### Q7. How do you implement observability in a Rust API?

**Interview Answer**

I use the `tracing` crate as the foundation, configuring structured log output with `tracing-subscriber` and `EnvFilter` for log-level control per module. Every request is wrapped in a span with a generated trace ID, and I propagate this ID across services via the `traceparent` header (OpenTelemetry W3C format). For metrics, I use the `metrics` crate with `metrics-exporter-prometheus` to expose `/metrics` endpoints on each Axum instance. I track request latency histograms, error rates, active connections, and custom business metrics. The combination of `tracing` for logs, `metrics` for Prometheus, and `opentelemetry` for distributed tracing gives me full visibility without runtime overhead when spans are disabled.

---

### Q8. How do you handle graceful shutdown and zero-downtime deployments?

**Interview Answer**

I use `tokio::signal::ctrl_c()` in the main function and pass it to `axum::Server::with_graceful_shutdown()`, which stops accepting new connections and waits for in-flight requests to complete. For Kubernetes deployments, I implement a SIGTERM handler with a 30-second timeout that matches the pod termination grace period. During shutdown, the Axum server stops listening, the Tokio runtime drains remaining tasks, database connections are returned to the pool, and Redis subscriptions are unsubscribed. I also implement health check endpoints (`/healthz` for liveness, `/readyz` for readiness) that Kubernetes uses to manage traffic routing — the readiness probe returns 503 during shutdown, telling the load balancer to stop sending traffic.

---

### Q9. How do you secure a high-performance Rust API?

**Interview Answer**

I implement defense in depth: TLS termination at the load balancer, JWT validation as Axum middleware using `jsonwebtoken`, rate limiting per IP and per API key, input validation with `validator` crate macros on request structs, and SQL injection prevention through `sqlx`'s parameterized queries. For CORS, I use `tower-http::CorsLayer` with a configurable allowlist. I never store secrets in code — they come from environment variables or AWS Secrets Manager at startup. I also implement request size limits via `tower-http::limit::RequestBodyLimitLayer` to prevent memory exhaustion attacks, and I use `tokio::time::timeout` on all external calls to prevent slow-loris attacks from tying up resources.

---

### Q10. What capacity planning do you do for a high-performance API?

**Interview Answer**

I start with target throughput — for example, 10,000 requests per second with p99 latency under 50ms. I benchmark a single Axum instance on the target hardware (typically a 4-core AWS c6i instance) using `wrk` or `vegeta` to find the maximum sustainable RPS. If one instance handles 3,000 RPS, I need at least 4 instances with a 20% headroom, so 5 instances behind an ALB. I then size the database pool accordingly: 20 connections per instance means 100 total PostgreSQL connections, requiring a `db.r6g.xlarge` instance. Redis needs enough memory for the working set of cached keys and enough CPU for the operations per second. I use `cargo flamegraph` to profile Rust hot paths and optimize before scaling horizontally.
