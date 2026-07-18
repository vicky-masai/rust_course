# How would you build a database connection pool from scratch?

## Interview Question

How would you build a database connection pool from scratch?

## Interview Answer

"Maintain reusable connections in a synchronized queue, allocate on demand, return connections after use, enforce maximum pool size, idle timeout, health checks, and waiting queues."

---

## Follow-up Questions & Answers

### Q1. What data structure would you use for the connection pool?

**Interview Answer**

A `VecDeque<Connection>` protected by a `tokio::sync::Mutex` for async access, or a `crossbeam::ArrayQueue` for lock-free bounded storage. For async pools, `tokio::sync::Semaphore` controls maximum connections while `Mutex<VecDeque>` manages the available connections. The semaphore approach avoids holding a mutex while waiting for a connection.

---

### Q2. How do you handle connection health checks?

**Interview Answer**

On checkout, execute a lightweight query like `SELECT 1` or use the database's ping mechanism. For PostgreSQL, `sqlx::PgPool` does this automatically. Alternatively, track connection age and proactively retire connections older than a configured lifetime. Health checks add latency, so make them fast and consider running them on a background task.

---

### Q3. What is connection idle timeout and how do you implement it?

**Interview Answer**

Idle timeout closes connections that haven't been used for a configured duration. Use a background task that periodically sweeps the pool and drops connections older than the timeout. In Tokio, spawn a task with `tokio::time::interval` that checks timestamps. This prevents holding too many connections to the database during low-traffic periods.

---

### Q4. How do you handle connection errors during checkout?

**Interview Answer**

If a borrowed connection fails, mark it as broken and create a new one. Use `Result<Connection, PoolError>` as the return type. Some pools implement a retry loop: if checkout fails, try creating a new connection up to N times. The `deadpool` crate handles this with automatic connection recycling on error.

---

### Q5. What is the difference between connection pooling and multiplexing?

**Interview Answer**

Pooling maintains a set of reusable connections, while multiplexing sends multiple queries over a single connection using PostgreSQL's pipeline protocol. `sqlx` supports both via `PgPool` (pooling) and `PgConnection` with `COPY` or prepared statements. Multiplexing reduces connections but increases query latency under load.

---

### Q6. How do you implement a waiting queue for pool exhaustion?

**Interview Answer**

When all connections are checked out, new requests wait on a `tokio::sync::Notify` or channel. Use `tokio::time::timeout` to fail fast if no connection becomes available within a deadline. The pattern: acquire semaphore permit, check out connection, on drop return connection and release permit. This naturally queues waiters.

---

### Q7. How do you measure pool health in production?

**Interview Answer**

Track metrics: connections active, connections idle, wait time histogram, checkout failures, and connection errors. Use `prometheus` or `metrics` crate to expose these. Alert when utilization stays above 80% or when wait times exceed thresholds. Dashboard visibility into pool behavior is essential for capacity planning.

---

### Q8. What is connection recycling and when should you do it?

**Interview Answer**

Recycling means returning a connection to the pool after verifying it's still valid. On return, run a health check or reset session state (clear temp tables, reset transaction isolation). For PostgreSQL, resetting the connection state is important if the previous user changed session settings. Some pools use a `CREATE` query as a lightweight validation.

---

### Q9. How do you handle pool sizing for different workloads?

**Interview Answer**

For CPU-bound work, pool size should match CPU cores (connections waste memory and context-switch). For I/O-bound work, more connections than cores keep the CPU busy. A common formula: `pool_size = (2 * num_cores) + num_disks`. Benchmark with realistic load—too few connections cause queuing, too many overwhelm the database.

---

### Q10. How does `sqlx::PgPool` implement connection pooling internally?

**Interview Answer**

`sqlx::PgPool` uses a semaphore-based approach: a `tokio::sync::Semaphore` limits concurrent connections, and a `Mutex<VecDeque>` manages available connections. On checkout, acquire a permit and pop a connection. On drop, push the connection back and release the permit. Background tasks handle connection recycling and idle timeout.

---
