# Describe a backend architecture you have built

## Interview Question

Describe a backend architecture you have built.

## Interview Answer

"I built a modular backend using Rust, Axum, PostgreSQL, Redis, JWT authentication, RBAC authorization, background workers, Docker, and event-driven processing. I separated handlers, services, repositories, middleware, and domain logic, added structured logging, centralized error handling, and caching, and designed the system to scale horizontally."

---

## Follow-up Questions & Answers

### Q1. How did you structure the service layer?

**Interview Answer**

I used a three-layer architecture: handlers (HTTP layer), services (business logic), and repositories (data access). Each layer depends only on the one below it via trait abstractions. This allowed me to swap PostgreSQL for SQLite in tests by implementing the same repository trait differently, keeping business logic completely decoupled from infrastructure.

---

### Q2. How did you handle database migrations?

**Interview Answer**

I used `sqlx::migrate!()` macro which embeds SQL migrations at compile time and validates them against the database schema. Migrations run automatically on startup. Each migration is a numbered SQL file in a `migrations/` directory. The `sqlx` CLI generates migrations and checks them against the running database to catch syntax errors early.

---

### Q3. How did you implement authentication and authorization?

**Interview Answer**

JWT tokens were issued on login and validated via Axum middleware. I used `jsonwebtoken` crate for token creation/validation and `axum-extra::middleware::from_fn` for the auth layer. RBAC was implemented as a permission check in the service layer—each endpoint required specific roles, and the middleware extracted claims into request extensions for handlers to access.

---

### Q4. How did you structure error handling across the service?

**Interview Answer**

Each layer had its own error type using `thiserror`: `RepoError`, `ServiceError`, `AppError`. Service errors converted from repo errors via `From` impls. The top-level `AppError` implemented `IntoResponse` for Axum, mapping specific variants to HTTP status codes. This gave structured errors at every layer while presenting clean HTTP responses.

---

### Q5. How did you implement background job processing?

**Interview Answer**

I used a dedicated worker binary that polled a `jobs` table in PostgreSQL. Jobs were enqueued by the API handlers using `sqlx`. The worker used `tokio::select!` to handle shutdown signals gracefully. For higher throughput, I switched to `Redis Streams` with consumer groups, which provided better visibility into job queues and retry semantics.

---

### Q6. How did you approach caching strategy?

**Interview Answer**

I used Redis with a cache-aside pattern: check cache first, on miss query DB and populate cache. Cache keys were namespaced by entity type with TTLs based on data freshness requirements. For list endpoints, I used Redis sorted sets with score-based pagination. Cache invalidation happened on writes via a publish/subscribe channel to keep caches consistent.

---

### Q7. How did you handle graceful shutdown?

**Interview Answer**

I used `tokio::signal::ctrl_c()` with `tokio::select!` to race against the server future. On signal, I set a shutdown flag, stopped accepting new connections, and waited for in-flight requests to complete with a timeout. Database connections and Redis clients were dropped in the shutdown sequence, ensuring clean resource cleanup.

---

### Q8. How did you implement structured logging?

**Interview Answer**

I used `tracing` with `tracing-subscriber` for structured, context-aware logging. Each request got a span with `request_id`, `method`, and `path`. The middleware injected these fields automatically. In production, logs were formatted as JSON with `tracing-json` for easy parsing by ELK stack. Error logs included the full error chain with `.context()`.

---

### Q9. How did you test the architecture?

**Interview Answer**

I used a test database with `sqlx::test` for repository tests, `axum::test` helpers for handler integration tests, and `mockall` for service unit tests with mocked repositories. The trait-based architecture made mocking straightforward. End-to-end tests spun up the full server with `tower::ServiceExt` and verified complete request flows.

---

### Q10. How did you design the system for horizontal scaling?

**Interview Answer**

State was externalized to PostgreSQL and Redis, so any instance could serve any request. Sessions and caches lived in Redis, not in-memory. Background workers were distributed via Redis Streams consumer groups. Load balancers distributed traffic across instances. The only stateful component was the database, which I scaled with read replicas and connection pooling.

---
