# Design an API Handling 1 Million Requests Per Second

## Interview Question

How would you design a Rust API capable of handling 1 million requests per second?

## Interview Answer

I would architect a horizontally scalable system with stateless Axum application servers deployed across multiple availability zones behind a global load balancer. The critical path uses Redis Cluster for caching hot data with sub-millisecond reads, and Kafka decouples async work like analytics events and background processing from the request path. PostgreSQL handles persistence with a primary for writes and multiple read replicas for queries, connected via connection-pooled `sqlx` clients. A CDN (CloudFront or Cloudflare) sits in front for static assets and cacheable responses, reducing origin traffic by 60-70%. Capacity estimation: 1M RPS ÷ 20K RPS per instance = 50 minimum instances, scaled to 65 with headroom, distributed across 3 AZs with connection pools sized to avoid database saturation.

---

## Follow-up Questions & Answers

### Q1. How do you estimate the infrastructure needed for 1M RPS?

**Interview Answer**

I benchmark a single Axum instance using `wrk` with 100 concurrent connections against the actual endpoint logic (including Redis and database calls). If each instance handles 15,000 RPS at p99 under 50ms, I need 1,000,000 / 15,000 = 67 instances minimum, rounded to 80 with 20% headroom for traffic spikes. Each instance gets a database connection pool of 20, so PostgreSQL needs at least 80 × 20 = 1,600 connections, requiring PgBouncer in transaction mode to multiplex. Redis Cluster needs 6+ shards to handle 1M+ operations per second. The total estimated monthly cost for compute, database, and cache is roughly $15,000–25,000 on AWS, which I validate against the budget.

---

### Q2. How do you design the load balancing architecture?

**Interview Answer**

I use a two-tier approach: an AWS Network Load Balancer (NLB) for L4 TCP forwarding to Axum instances (minimal latency overhead), and optionally a global Anycast load balancer like Cloudflare for geographic distribution. Within each region, the NLB distributes connections across Axum instances using a round-robin or least-connections algorithm. For sticky sessions (rarely needed), I use consistent hashing on the client IP. The NLB performs health checks on `/healthz` every 5 seconds and removes unhealthy instances within 15 seconds. I also configure connection draining with a 30-second deregistration delay to handle in-flight requests during deployments. The key is that Axum servers are completely stateless — all state lives in Redis or PostgreSQL.

---

### Q3. How do you handle database scaling for 1M RPS?

**Interview Answer**

At this scale, a single PostgreSQL instance cannot handle the write load, so I implement a multi-tier strategy. Reads (typically 80% of traffic) go to 3-5 read replicas using `sqlx` with separate read and write pools — the `PgPool` for writes connects to the primary, while a second pool routes SELECT queries to replicas via PgBouncer. Writes are batched: instead of individual INSERT statements, I accumulate events in a Tokio channel and flush them in batches of 500 using PostgreSQL's `COPY` protocol, which is 5-10x faster than individual inserts. For critical-path data, I use Redis as the source of truth for frequently updated counters (like view counts) and periodically flush to PostgreSQL in the background.

---

### Q4. How do you implement horizontal scaling with Kubernetes?

**Interview Answer**

I deploy Axum as a Kubernetes Deployment with a HorizontalPodAutoscaler (HPA) configured to scale on custom metrics — specifically requests per second per pod from Prometheus, rather than just CPU. The HPA targets 70% of the per-pod RPS capacity and scales pods between 20 and 200 replicas. Each pod runs the Axum binary in a container with resource limits (2 CPU, 2GB memory) and a liveness probe on `/healthz` and a readiness probe on `/readyz`. I use PodDisruptionBudgets to ensure at least 80% of pods remain available during rolling updates. The KEDA (Kubernetes Event-driven Autoscaler) can also scale based on Kafka consumer lag, ensuring pods scale up before the queue backs up.

---

### Q5. How do you design caching at this scale?

**Interview Answer**

I use a multi-level cache: L1 is an in-process cache using `moka` (a concurrent Rust cache library) with a 10-second TTL for extremely hot data, eliminating Redis calls entirely for repeated requests. L2 is Redis Cluster, sharded across 6 nodes with consistent hashing, holding data with TTLs of 1-5 minutes. L3 is the PostgreSQL database for cache misses. The cache key design uses short, fixed-format keys like `u:{id}:p` (user profile) to minimize Redis memory. I implement cache warming by subscribing to Kafka change events and proactively updating Redis when underlying data changes, reducing cache miss rates to under 1%. For cache stampede prevention, I use a Redis mutex pattern where only one instance repopulates a missed key while others wait or return stale data.

---

### Q6. How do you handle traffic spikes and auto-scaling?

**Interview Answer**

Auto-scaling is reactive by nature (pods take 30-60 seconds to start), so I combine it with proactive strategies. I use KEDA to scale based on Kafka queue depth and request rate, not just CPU. For sudden traffic spikes, I implement request shedding with priority queues — low-priority requests (like analytics) are dropped when the system is overloaded, protecting critical-path endpoints. The `tower::load_shed` middleware in Axum returns 503 when the load exceeds a threshold. I also maintain warm standby capacity — the minimum pod count is set to handle 2x average traffic, so the system can absorb a 2x spike without waiting for new pods. Finally, I use reserved capacity for known events (product launches, marketing campaigns).

---

### Q7. What are the main bottlenecks at 1M RPS and how do you address them?

**Interview Answer**

The primary bottlenecks are: (1) Database connections — addressed with PgBouncer connection pooling and read replicas; (2) Redis hot keys — addressed by key sharding, local caching, and avoiding single keys that all requests access; (3) TLS handshake overhead — addressed by TLS session resumption and connection keep-alive; (4) DNS resolution — addressed by using IP-based load balancer addresses internally; (5) TCP port exhaustion — addressed by running multiple Axum instances per host and using connection pooling on the client side; (6) Event loop saturation — addressed by ensuring Tokio worker threads match CPU cores (typically 4-8) and avoiding blocking operations in async code. I profile with `cargo flamegraph` and `pprof-rs` to identify actual hot paths rather than guessing.

---

### Q8. How do you implement idempotency for high-throughput APIs?

**Interview Answer**

Idempotency prevents duplicate processing when clients retry requests. I generate an idempotency key (usually a UUID v4) on the client side and send it in a header. On the server side, the Axum handler first checks Redis for the key — if it exists, return the cached response. If not, execute the operation, store the result in Redis with a 24-hour TTL, and return it. For write operations that affect PostgreSQL, I wrap the logic in a database transaction that includes an idempotency table with a unique constraint on the key, ensuring the operation executes exactly once even if the Redis check has a race condition. The Rust implementation uses `redis::SET NX` (set-if-not-exists) for atomicity.

---

### Q9. How do you implement distributed tracing across all services?

**Interview Answer**

I use OpenTelemetry Rust SDK with `tracing-opentelemetry` to instrument every Axum request handler. Each request gets a unique trace ID generated at the load balancer or API gateway, propagated via the `traceparent` header (W3C standard). The trace ID is injected into all downstream calls — Redis operations, PostgreSQL queries, Kafka messages — using OpenTelemetry context propagation. Spans are exported to Jaeger or AWS X-Ray via the OpenTelemetry Collector, which runs as a DaemonSet in Kubernetes. I use `tracing::info_span!("db_query", query_id = %id)` to create meaningful spans. The `axum-tracing` middleware automatically creates a root span for each request with the trace ID, making it easy to correlate logs and traces.

---

### Q10. How do you handle graceful degradation when dependencies fail?

**Interview Answer**

I implement a circuit breaker pattern using a Tokio-based state machine: when Redis failures exceed a threshold (e.g., 5 in 10 seconds), the circuit opens and requests fall through to a local cache or default response. After 30 seconds, the circuit enters half-open state and tests with a single request — if it succeeds, the circuit closes. In Rust, I use `tokio::sync::watch` to broadcast the circuit state to all handlers without locking. For PostgreSQL, I implement read-through to Redis when replicas are unavailable, accepting stale data rather than failing reads. For Kafka, I buffer messages in a local ring buffer and replay them when the broker recovers. Each degradation is logged and alerted so the on-call engineer can investigate.
