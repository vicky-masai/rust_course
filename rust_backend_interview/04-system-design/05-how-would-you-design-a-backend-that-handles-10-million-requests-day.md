# How Would You Design a Backend That Handles 10 Million Requests/Day?

## Interview Question

How would you design a Rust backend system capable of handling 10 million requests per day?

## Interview Answer

10 million requests/day translates to roughly 115 requests per second at average load, with peaks around 500-1000 RPS. I would deploy stateless Axum servers behind a load balancer, scaling horizontally to 3-4 instances for redundancy. PostgreSQL serves as the primary database with one read replica, connected via `sqlx` connection pools. Redis handles caching and session storage with a 30-50% cache hit rate reducing database load significantly. Kafka manages async tasks like email notifications, analytics event processing, and background jobs. A CDN offloads static assets and cacheable API responses. Prometheus and Grafana provide observability with alerts for latency spikes and error rate increases. The total estimated infrastructure cost is around $500-1500/month on AWS.

---

## Follow-up Questions & Answers

### Q1. How do you perform capacity estimation for 10M requests/day?

**Interview Answer**

I break down the traffic pattern: 10M/day ÷ 86,400 seconds = 115 RPS average. Peak traffic is typically 5-10x average, so I plan for 600-1000 RPS at peak. Each Axum instance on a 2-core instance handles approximately 3,000 RPS for simple CRUD endpoints, so 2 instances handle peak load with headroom. For storage: if each request generates a 1KB log, that's 10GB/day or 300GB/month. PostgreSQL storage depends on data model — a 10M-row table with proper indexing occupies roughly 2-5GB. Redis memory depends on cache working set size — 1GB of Redis memory can hold approximately 500,000 cached objects. I validate these estimates with load testing using `wrk` before deployment.

---

### Q2. How do you design the database schema and connection management?

**Interview Answer**

I normalize the schema to 3NF for transactional data, using integer primary keys (not UUIDs) for better index performance and smaller storage. Foreign keys are enforced at the database level, and indexes are created based on actual query patterns from `EXPLAIN ANALYZE` output. For connection management, I use `sqlx::PgPool` with `max_connections(10)` per Axum instance, giving a total of 20-30 connections across all instances. I deploy PgBouncer in transaction pooling mode to handle any burst that exceeds the configured pool size. Migrations are managed with `sqlx migrate run` as a Kubernetes init container, ensuring the schema is always up to date before the application starts serving traffic.

---

### Q3. How do you implement async processing for non-critical tasks?

**Interview Answer**

I use Kafka as the message broker for decoupling non-critical work from the request path. When a request triggers a side effect (like sending a welcome email or updating search indexes), the Axum handler publishes a message to a Kafka topic and returns immediately, keeping the response latency low. Separate Tokio-based consumer services subscribe to these topics and process messages at their own pace. I implement dead letter queues (DLQ) for failed messages, with a retry policy of 3 attempts with exponential backoff. In Rust, I use `rdkafka` crate with a consumer loop in `tokio::spawn`, processing messages in batches of 100 for efficiency. The DLQ consumer logs failures to Sentry for manual investigation.

---

### Q4. How do you handle caching strategy for this scale?

**Interview Answer**

I implement a multi-tier caching strategy. The first tier is in-process caching using `moka` with a 30-second TTL for extremely hot data, which eliminates Redis calls for repeated requests to the same instance. The second tier is Redis with a 5-minute TTL for user profiles, product data, and frequently accessed lists. The third tier is PostgreSQL materialized views for complex aggregation queries. Cache invalidation follows the cache-aside pattern: the application checks the cache first, and on a miss, queries the database and populates the cache. For write operations, I invalidate the relevant cache keys immediately using Redis `DEL` and accept brief staleness. I monitor the cache hit rate via a `metrics::counter` and aim for 80%+ for read-heavy endpoints.

---

### Q5. How do you deploy and manage this system?

**Interview Answer**

I use Docker to containerize the Axum binary, building a minimal image with a multi-stage Dockerfile (build in `rust:slim`, run in `debian:bookworm-slim`). The application is deployed to AWS EKS (Kubernetes) with 3 nodes. The Kubernetes manifests define Deployments (3 Axum replicas), Services (ClusterIP for internal, LoadBalancer for external), ConfigMaps for non-secret configuration, and Secrets for database credentials and API keys. Helm charts manage environment-specific overrides. I use GitHub Actions for CI/CD: `cargo test` → `cargo clippy` → `cargo build --release` → Docker build → ECR push → `kubectl apply`. ArgoCD handles GitOps-style deployments with automatic sync.

---

### Q6. How do you implement observability and monitoring?

**Interview Answer**

I instrument the Axum application with three pillars of observability. **Metrics**: The `metrics` crate exports request count, latency histograms, error rates, and custom business metrics to Prometheus, scraped every 15 seconds. **Logs**: The `tracing` crate with `tracing-subscriber` outputs structured JSON logs to stdout, collected by Fluent Bit and shipped to Elasticsearch for Kibana dashboards. **Traces**: OpenTelemetry with `tracing-opentelemetry` creates distributed traces for every request, exported to Jaeger via the OpenTelemetry Collector. I set up Grafana dashboards showing p50/p95/p99 latency, error rate by endpoint, database query duration, Redis hit rate, and Kafka consumer lag. Alerts fire on PagerDuty when error rates exceed 1% or p99 latency exceeds 500ms.

---

### Q7. How do you secure the backend?

**Interview Answer**

I implement defense in depth across all layers. At the network level, the Kubernetes NetworkPolicy restricts pod-to-pod communication — only the Axum pods can reach PostgreSQL and Redis on their respective ports. At the application level, JWT tokens are validated using `jsonwebtoken` crate with RS256 signatures, and all endpoints require authentication unless explicitly marked as public. Input validation uses `validator` derive macros on all request structs, rejecting malformed input before it reaches business logic. SQL injection is prevented by `sqlx`'s parameterized queries. Secrets are stored in AWS Secrets Manager and injected as environment variables. I also run `cargo audit` in CI to detect known vulnerabilities in dependencies.

---

### Q8. How do you handle data backup and disaster recovery?

**Interview Answer**

PostgreSQL is configured with continuous WAL archiving to S3, enabling point-in-time recovery to any moment in the last 30 days. I run daily `pg_dump` backups to a separate S3 bucket with 90-day retention. Redis uses RDB snapshots every 5 minutes and AOF persistence for durability, with cross-region replication to a warm standby in us-west-2. The RTO (Recovery Time Objective) target is 15 minutes: promote the PostgreSQL replica to primary, update DNS, and scale up the standby Redis. The RPO (Recovery Point Objective) is 5 minutes (Redis) or 1 hour (PostgreSQL daily backup). I test disaster recovery quarterly by running a full failover drill in a staging environment and measuring the actual RTO.

---

### Q9. How do you handle API versioning and backward compatibility?

**Interview Answer**

I use URL-based versioning (`/api/v1/`, `/api/v2/`) to keep versions explicit and independently routeable. Each version is a separate Axum `Router` mounted on the main router, allowing me to evolve endpoints independently. When adding fields to responses, I use `#[serde(default)]` and `#[serde(skip_serializing_optionals)]` to ensure old clients ignore new fields without errors. When deprecating endpoints, I return the `Deprecation: true` and `Sunset: Sat, 01 Jan 2027` headers 6 months before removal. I maintain backward compatibility by never removing or renaming existing fields — only adding new ones. An OpenAPI spec is auto-generated with `utoipa` for each version and published to a developer portal.

---

### Q10. How do you implement zero-downtime deployments?

**Interview Answer**

I configure the Kubernetes Deployment with a `RollingUpdate` strategy: `maxSurge: 1` and `maxUnavailable: 0` ensures new pods start before old pods are terminated. The Axum server uses `tokio::signal::ctrl_c()` for graceful shutdown, which stops accepting new connections and waits for in-flight requests to complete within a 30-second deadline. The readiness probe returns 503 during shutdown, causing the Service to remove the pod from the endpoint list before termination begins. Database connections are properly returned to the pool during shutdown. I also implement a startup probe that waits for the Axum server to be ready before marking the pod as ready, preventing premature traffic routing during cold starts. The entire deploy takes 2-3 minutes with zero dropped requests.
