# Tell me about your backend project

## Interview Question

Tell me about your backend project.

## Interview Answer

"I built a multi-module backend using Rust and Axum. The system exposes REST APIs, uses PostgreSQL for storage, Redis for caching, JWT authentication, RBAC authorization, background workers for asynchronous processing, Docker for deployment, and follows a modular architecture with repository, service, and handler layers. My focus was on performance, maintainability, and scalability."

---

## Follow-up Questions & Answers

### Q1. Why did you choose Rust and Axum specifically for this project?

**Interview Answer**

I needed low latency and high throughput for real-time data processing, and Rust's zero-cost abstractions delivered that without garbage collector pauses. Axum provided a modern, type-safe framework with excellent Tokio integration and a clean extractor pattern. Compared to Actix-web, Axum's design aligned better with my preference for composable middleware and explicit state management.

---

### Q2. Can you walk me through the request flow in your Axum backend?

**Interview Answer**

A request hits the load balancer, passes through CORS and rate-limiting middleware, then reaches the Axum router which matches the path to a handler. The handler extracts parameters using Axum's type-safe extractors, calls the service layer for business logic, and the service calls the repository for database operations. The response flows back as JSON with proper status codes and error handling at each layer.

---

### Q3. How did you implement JWT authentication and RBAC in your project?

**Interview Answer**

JWT tokens are issued on login with user ID and role claims signed with HMAC-SHA256. A custom Axum extractor validates the token from the `Authorization` header on protected routes and returns the decoded claims. RBAC is implemented as a middleware that checks the user's role against a permission map defined per route, returning `403 Forbidden` if unauthorized.

---

### Q4. What was the most difficult technical challenge in this project?

**Interview Answer**

The hardest part was implementing reliable background workers for processing async tasks like email sending and data aggregation. I used a Tokio channel-based worker pool with retry logic and dead-letter handling. Ensuring tasks weren't lost during deployments required persisting them to PostgreSQL before spawning the worker, which added complexity but guaranteed reliability.

---

### Q5. How do you structure your database migrations and schema?

**Interview Answer**

I use `sqlx migrate` with timestamped SQL files in a `migrations/` directory. Each migration is idempotent and reversible where possible. I run migrations as part of the application startup with a dedicated migration step before the server begins accepting requests. This ensures the schema is always up to date without requiring separate deployment steps.

---

### Q6. How did you test your backend system?

**Interview Answer**

I wrote unit tests for service logic using mock repositories, integration tests for API endpoints using `axum::test`, and end-to-end tests against a test database. For integration tests, I used `sqlx::test` which automatically provisions a temporary PostgreSQL database per test. I also added load testing with `wrk` or `k6` to validate performance under concurrent traffic.

---

### Q7. What would you do differently if you rebuilt this project?

**Interview Answer**

I would add OpenTelemetry tracing from the start instead of bolting it on later, and I would implement event sourcing for the audit trail instead of a simple log table. I would also invest more in contract testing between services using Pact, and set up a proper staging environment that mirrors production traffic patterns for more realistic testing.

---

### Q8. How did you handle configuration management across environments?

**Interview Answer**

I used environment variables with the `config` crate that supports layered configuration from files, environment variables, and defaults. Each environment has a `.env` file for development and environment-specific variables injected through Docker or Kubernetes for staging and production. Secrets like database passwords and JWT keys are stored in AWS Secrets Manager and injected at runtime, never committed to code.

---

### Q9. How did you ensure your backend was production-ready?

**Interview Answer**

I added health check endpoints, graceful shutdown handling with Tokio signal traps, structured logging, metrics collection, and rate limiting. I set up CI/CD pipelines with linting, testing, and security scanning. Load testing validated the system handled expected traffic, and I documented runbooks for common operational scenarios like database failover and cache invalidation.

---
