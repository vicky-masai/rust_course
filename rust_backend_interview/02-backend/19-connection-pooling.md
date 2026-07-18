# connection pooling

## Interview Question

Explain connection pooling.

## Interview Answer

"A connection pool reuses database connections instead of creating a new connection for every request, reducing latency and database overhead."

---

## Follow-up Questions & Answers

### Q1. What pool size should you configure for a production Axum application?

**Interview Answer**

Start with `max_connections` set to the number of CPU cores times 2, then tune based on observed latency and database connection limits. Monitor pool wait times with sqlx metrics and increase only if requests are waiting for connections. Always set `min_idle` and `connect_timeout` to prevent exhausted pools under load.

---

### Q2. How does sqlx's connection pool differ from r2d2?

**Interview Answer**

sqlx's pool is built-in with `sqlx::PgPool` and is async-native, while r2d2 is synchronous and requires a separate adapter like `sqlx::r2d2`. sqlx pools are simpler to configure and integrate directly with Axum handlers via `Extension<PgPool>`. r2d2 is more general-purpose but adds unnecessary complexity for async Rust backends.

---

### Q3. What happens when all connections in the pool are exhausted?

**Interview Answer**

Requests block until a connection becomes available or the configured timeout expires, returning an error. In Axum, this surfaces as a 500 error if not handled explicitly. You should monitor `pool.size()`, `pool.idle()`, and `pool.waiters()` metrics and scale up the pool or database connections accordingly.

---

### Q4. How do you handle connection pool cleanup on application shutdown?

**Interview Answer**

Call `pool.close().await` during graceful shutdown in Axum using a shutdown signal like `tokio::signal::ctrl_c()`. This ensures all in-flight queries complete before connections are released. Without explicit cleanup, idle connections may linger and cause resource leaks.

---

### Q5. Should you use one PgPool per Axum handler or share a single pool?

**Interview Answer**

Share a single `PgPool` across all handlers using `Extension<PgPool>` in the Axum router. The pool internally manages multiple connections and is designed for concurrent use. Creating separate pools per handler wastes database connections and defeats the purpose of pooling.

---

### Q6. How does connection pooling interact with database transaction handling in sqlx?

**Interview Answer**

sqlx transactions borrow a connection from the pool for the duration of the transaction and return it on commit or rollback. If a transaction hangs open, the connection is unavailable to other requests. Always use `sqlx::Transaction` with explicit commit/rollback and avoid holding transactions across HTTP boundaries.

---

### Q7. What metrics should you monitor for a connection pool in production?

**Interview Answer**

Monitor active connections, idle connections, wait time for connections, connection errors, and pool timeout rates. Use Prometheus exporters or sqlx's built-in `PoolOptions` stats to track these. Alert on high wait times or connection exhaustion as they indicate scaling or configuration issues.

---

### Q8. How does async connection pooling differ from synchronous pooling in terms of performance?

**Interview Answer**

Async pools allow the runtime to handle other tasks while waiting for a connection, avoiding thread blocking. Synchronous pools tie up a thread per waiting request, limiting throughput under high concurrency. In Axum with Tokio, async pools let you serve thousands of concurrent requests with minimal threads.
