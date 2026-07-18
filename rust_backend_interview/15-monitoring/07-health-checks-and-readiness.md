# Health Checks and Readiness

## Interview Question

How do you implement health checks and readiness probes for a Rust backend service?

## Interview Answer

Health checks verify that a service is running and capable of handling requests. Implement two endpoints: `/health` (liveness) checks if the process is alive, and `/ready` (readiness) checks if the service can handle traffic. Liveness should be lightweight — return 200 if the process is running. Readiness checks dependencies (database connection, cache availability, configuration loaded). Kubernetes uses liveness probes to restart stuck containers and readiness probes to remove unhealthy pods from load balancers. Return structured JSON responses with dependency status for debugging.

---

## Follow-up Questions & Answers

### Q1. What is the difference between liveness and readiness probes?

**Interview Answer**

Liveness probes check if the service is alive and should be restarted if they fail. They must be independent of external dependencies — checking database connectivity in a liveness probe can cause unnecessary restarts during database issues. Readiness probes check if the service can handle requests and should be removed from load balancers if they fail. They CAN check dependencies because they only affect traffic routing, not process lifecycle.

---

### Q2. How do you implement health check endpoints in axum?

**Interview Answer**

Create separate routes for `/health` and `/ready` using `axum::Router`. The `/health` handler returns `StatusCode::OK` immediately. The `/ready` handler checks each dependency by attempting a lightweight operation (ping database, check cache). Use `tokio::join!` to check dependencies concurrently. Return a JSON body with each dependency's status: `{"status": "ready", "database": "ok", "cache": "ok"}`. Set appropriate timeout (1-2 seconds) so slow dependencies don't block the check.

---

### Q3. How do you configure Kubernetes health checks for a Rust service?

**Interview Answer**

In your Kubernetes deployment YAML, define `livenessProbe` and `readinessProbe` with `httpGet` pointing at your health endpoints. Set `initialDelaySeconds` to allow startup time (10-30s), `periodSeconds` for check frequency (10-15s), and `failureThreshold` for restart/removal triggers (3 failures). Use `startupProbe` for slow-starting services. Configure different timeouts for liveness (must be fast) and readiness (can be slightly slower).

---

### Q4. What should you check in a readiness probe?

**Interview Answer**

Check database connectivity with a lightweight query (SELECT 1). Verify cache connectivity (Redis PING). Confirm configuration is loaded and valid. Check that critical external service connections are established. Verify that the application has finished initialization. Don't check every dependency — focus on those that prevent the service from handling requests. Keep checks fast (<1 second total) to avoid the probe itself becoming a bottleneck.

---

### Q5. How do you handle graceful shutdown alongside health checks?

**Interview Answer**

When Kubernetes sends SIGTERM, immediately fail the readiness probe (return 503) to stop new traffic. Continue serving in-flight requests for the termination grace period. Close database connections and flush logs during shutdown. Use `tokio::signal::ctrl_c()` or `hyper::signal::graceful_shutdown()` to coordinate. This ensures zero-downtime deployments by draining connections before the process exits.

---

### Q6. How do you test health check endpoints?

**Interview Answer**

Write unit tests that verify the health endpoint returns 200 under normal conditions. Mock dependency failures and verify readiness returns 503 with the correct dependency status. Integration tests should hit the actual endpoints after starting the full service. Test that health checks respond quickly under load. Verify that health checks work during graceful shutdown. Test that unhealthy dependencies cause correct degraded states.

---

### Q7. How do you implement health checks for dependent services?

**Interview Answer**

Create a `HealthChecker` trait with a `check_health()` method. Implement it for each dependency (PostgresHealthChecker, RedisHealthChecker, ExternalApiHealthChecker). Run all checkers concurrently in the readiness endpoint. Return aggregated status with individual dependency details. Log health check failures with enough context for debugging. Consider caching health check results briefly (5-10 seconds) to avoid hammering dependencies during cascading failures.

---

### Q8. What are common pitfalls with health checks?

**Interview Answer**

Common mistakes include: making liveness checks depend on external dependencies (causes unnecessary restarts), setting timeouts too high (health checks themselves become a bottleneck), not checking during startup (accepting traffic before ready), caching results too aggressively (stale health status), and not monitoring health check failures as an alert. Also, avoid heavy computation in health checks — they should be lightweight operations that don't consume significant resources.

---

### Q9. How do you implement startup probes for slow-starting services?

**Interview Answer**

Kubernetes startup probes run only during container startup and are disabled once they succeed. They allow longer timeouts than liveness probes, giving slow-starting services time to initialize. Configure with `startupProbe.initialDelaySeconds`, `periodSeconds`, and a higher `failureThreshold`. A Rust service might need startup probes if it performs database migrations, loads large datasets, or establishes many connections at boot. Once the startup probe passes, liveness and readiness probes take over.

---

### Q10. How do you monitor health check availability itself?

**Interview Answer**

Track health check response times and failure rates as Prometheus metrics. Alert on sustained health check failures, which indicate systemic issues. Monitor the frequency and duration of readiness state transitions. Log health check results with enough context to diagnose failures. Create a dashboard showing all services' health status with historical trends. If health checks themselves are failing, you have an observability gap that needs immediate attention.
