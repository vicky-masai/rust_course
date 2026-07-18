# Database Connection Pooling

## Interview Question

What is database connection pooling and why is it necessary?

## Interview Answer

Database connection pooling is a technique where a limited number of pre-established database connections are shared among many application threads, avoiding the overhead of creating a new connection for each request. PostgreSQL forks a new process for every connection (unlike thread-based databases), which costs ~10MB of memory and ~100ms of setup time per connection. A connection pool maintains a set of reusable connections, borrowing one when needed and returning it when done. This dramatically reduces memory usage, avoids connection storms, and enables better resource management. Common poolers include PgBouncer (external), SQLx's built-in pool (application-level), and Tokio Postgres with deadpool.

---

## Follow-up Questions & Answers

### Q1. What is the difference between PgBouncer and SQLx's built-in connection pool?

**Interview Answer**

SQLx's PgPool is an application-level connection pool that manages connections within the Rust process. It handles connection lifecycle, health checks, idle timeouts, and connection borrowing within your application. PgBouncer is an external connection pooler that sits between the application and PostgreSQL, multiplexing many client connections over a small number of server connections. The key difference: SQLx's pool reduces connection creation overhead within your app, while PgBouncer reduces the number of actual PostgreSQL server processes. For production deployments, use both: SQLx's pool handles application-side connection management (thread safety, async borrowing), and PgBouncer handles server-side connection multiplexing. This two-tier approach provides the best performance and resource utilization.

```rust
// SQLx application-level pool
let pool = sqlx::PgPoolOptions::new()
    .max_connections(20)           // App-side pool size
    .min_connections(5)            // Maintain at least 5 idle connections
    .acquire_timeout(Duration::from_secs(3))  // Wait up to 3s for a connection
    .idle_timeout(Duration::from_secs(300))   // Close idle connections after 5min
    .max_lifetime(Duration::from_secs(1800))  // Recycle connections after 30min
    .connect("postgres://user:pass@localhost:6432/mydb")  // PgBouncer port
    .await?;
```

```ini
# pgbouncer.ini — server-side pool
[databases]
mydb = host=127.0.0.1 port=5432 dbname=mydb

[pgbouncer]
pool_mode = transaction          # Return connection after each transaction
max_client_conn = 1000          # Accept up to 1000 client connections
default_pool_size = 20          # But only 20 actual PostgreSQL connections
reserve_pool_size = 5           # Emergency pool for bursts
reserve_pool_timeout = 3        # Seconds before using reserve pool
server_idle_timeout = 600       # Close idle server connections after 10min
```

---

### Q2. How do you size a connection pool for a Rust web application?

**Interview Answer**

Pool sizing depends on your workload type and available resources. A common formula: `pool_size = (2 * number_of_cpu_cores) + effective_spindle_count`. For SSD-based systems with async I/O, `pool_size = 2 * CPU_cores` is a good starting point. Too many connections cause contention (lock manager overhead, context switching), while too few cause request queuing. Monitor pool utilization: if `idle_connections` is consistently 0, increase the pool; if it's consistently high, decrease it. For PgBouncer, `default_pool_size` should match your SQLx pool size. For most Rust web services, 10-30 SQLx connections with 10-20 PgBouncer connections is optimal. Load test to find the sweet spot.

```rust
// Right-sized pool for a typical Rust web service
let pool = sqlx::PgPoolOptions::new()
    .max_connections(20)  // Match to: 2 * 8 CPU cores + buffer
    .min_connections(5)   // Keep warm connections ready
    .acquire_timeout(Duration::from_secs(5))
    .connect(database_url)
    .await?;

// Monitor pool stats
let stats = pool.size();          // Total connections
let idle = pool.idle();           // Idle connections
let active = stats - idle;        // Active connections

// Expose metrics for monitoring
loop {
    tracing::info!(
        total = pool.size(),
        idle = pool.idle(),
        active = pool.size() - pool.idle(),
        "Connection pool stats"
    );
    tokio::time::sleep(Duration::from_secs(30)).await;
}
```

---

### Q3. What are the risks of having too many or too few database connections?

**Interview Answer**

Too many connections cause: memory exhaustion (each PostgreSQL process uses ~10MB), increased lock contention, context switching overhead, and degraded performance. PostgreSQL's `max_connections` (default: 100) is the hard limit — exceeding it rejects new connections. Too few connections cause: request queuing (applications wait for available connections), reduced throughput under load, and potential timeouts. The sweet spot depends on workload: OLTP (many short queries) benefits from more connections, while OLAP (few long queries) benefits from fewer connections. Monitor with `pg_stat_activity` and track `pg_stat_database.connections`. Use PgBouncer to allow many application connections while limiting actual database connections.

```sql
-- Monitor connection count
SELECT count(*), state
FROM pg_stat_activity
WHERE datname = current_database()
GROUP BY state;

-- Find connection-heavy applications
SELECT usename, application_name, client_addr, count(*)
FROM pg_stat_activity
WHERE datname = current_database()
GROUP BY usename, application_name, client_addr
ORDER BY count DESC;

-- Check connection limit
SHOW max_connections;  -- 100 default
SELECT rolname, rolconnlimit
FROM pg_roles
WHERE rolname = current_user;

-- Kill idle connections
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE state = 'idle'
  AND now() - state_change > interval '10 minutes'
  AND pid != pg_backend_pid();
```

---

### Q4. What is connection pool exhaustion and how do you handle it?

**Interview Answer**

Connection pool exhaustion occurs when all connections in the pool are in use and new requests must wait for one to be returned. This causes request latency spikes and potential timeouts. In SQLx, when the pool is exhausted, `acquire()` waits up to `acquire_timeout` before returning an error. To handle this: set appropriate timeouts (don't wait forever), implement backpressure (reject requests early), monitor pool utilization metrics, and scale up pool size or add PgBouncer. Common causes of exhaustion: long-running queries holding connections, connection leaks (not returning connections), and insufficient pool size for the workload. Use SQLx's `acquire_timeout` to fail fast rather than queue indefinitely.

```rust
use sqlx::PgPool;
use std::time::Duration;

// Configure pool with appropriate timeouts
let pool = PgPool::builder()
    .max_connections(20)
    .acquire_timeout(Duration::from_secs(5))  // Fail after 5s
    .connect(database_url)
    .await?;

// Handle pool exhaustion gracefully
async fn query_with_timeout(pool: &PgPool) -> Result<Vec<User>> {
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        sqlx::query_as!(User, "SELECT * FROM users WHERE active = true")
            .fetch_all(pool)
    ).await;

    match result {
        Ok(Ok(users)) => Ok(users),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => {
            tracing::error!("Query timed out — possible pool exhaustion");
            Err(anyhow!("Query timeout"))
        }
    }
}

// Monitor pool health
async fn pool_health_check(pool: &PgPool) -> bool {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => true,
        Err(e) => {
            tracing::error!(?e, "Pool health check failed");
            false
        }
    }
}
```

---

### Q5. What is connection pooling in transaction mode vs session mode in PgBouncer?

**Interview Answer**

PgBouncer supports three pooling modes. Session mode: a client gets a dedicated server connection for the entire client session, returned when the client disconnects — simplest but least efficient. Transaction mode: a server connection is assigned at transaction start and returned at COMMIT/ROLLBACK — most efficient for most applications, but breaks session-level features like PREPARE statements and SET parameters. Statement mode: each statement gets a fresh connection (autocommit only) — too restrictive for most use cases. Transaction mode is recommended for Rust web services using SQLx because web requests typically map to single transactions. Be aware that transaction mode breaks prepared statements and LISTEN/NOTIFY — use SQLx's statement caching carefully.

```ini
# pgbouncer.ini — transaction mode (recommended)
[pgbouncer]
pool_mode = transaction

# Session mode (if you need prepared statements or SET)
pool_mode = session

# Statement mode (autocommit only)
pool_mode = statement
```

```rust
// Transaction mode works well with SQLx
async fn create_user(pool: &PgPool, name: &str) -> Result<User> {
    // This transaction gets a PgBouncer connection, returns it at commit
    let mut tx = pool.begin().await?;
    let user = sqlx::query_as!(User,
        "INSERT INTO users (name) VALUES ($1) RETURNING *", name
    )
    .fetch_one(&mut *tx).await?;
    tx.commit().await?;  // Connection returned to PgBouncer pool
    Ok(user)
}

// Problem: prepared statements break in transaction mode
// SQLx caches prepared statements — in transaction mode, the next
// transaction may get a different server connection without the cache.
// SQLx handles this gracefully by re-preparing as needed.
```

---

### Q6. How do you monitor connection pool health in a Rust application?

**Interview Answer**

Monitor pool size, idle connections, active connections, wait times, and connection errors. SQLx's PgPool exposes `size()` (total connections) and `idle()` (idle connections) methods. Track these over time with metrics libraries like `prometheus` or `metrics`. Alert when idle connections approach 0 (pool exhaustion), when active connections spike (possible slow queries), or when connections are created/destroyed rapidly (instability). Also monitor PostgreSQL server-side with `pg_stat_activity` to detect long-running queries holding connections. Combine application metrics with database metrics for a complete picture. Use structured logging for connection lifecycle events.

```rust
use metrics::{counter, gauge};
use sqlx::PgPool;

async fn monitor_pool(pool: &PgPool) {
    loop {
        let total = pool.size();
        let idle = pool.idle();
        let active = total - idle;

        gauge!("db_pool_total").set(total as f64);
        gauge!("db_pool_idle").set(idle as f64);
        gauge!("db_pool_active").set(active as f64);

        // Alert on exhaustion risk
        if idle == 0 {
            tracing::warn!(total, "Connection pool fully utilized!");
        }

        // Track connection creation/destruction rate
        counter!("db_pool_connections_created").increment(0);  // Baseline
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

// SQLx connection pool metrics
// SQLx logs connection events when RUST_LOG=sqlx=trace
// Use tracing for structured connection lifecycle logs
```

---

### Q7. What is connection pool warmup and why does it matter?

**Interview Answer**

Connection pool warmup is the process of pre-establishing database connections at application startup rather than creating them on-demand during the first requests. Cold starts cause latency spikes because connection establishment takes ~50-200ms (TCP handshake + PostgreSQL authentication). For Rust web services behind load balancers, cold starts during deployment cause 503 errors. SQLx doesn't have built-in warmup, but you can implement it by calling `pool.connect()` or running `SELECT 1` in a loop during startup. Some Rust frameworks (like Axum) support initialization hooks where you can warm up the pool. The benefit is consistent latency from the first request.

```rust
use sqlx::PgPool;

async fn warm_up_pool(pool: &PgPool, connections: usize) -> Result<()> {
    let mut handles = Vec::new();
    for _ in 0..connections {
        let pool = pool.clone();
        handles.push(tokio::spawn(async move {
            // This forces a new connection to be created
            sqlx::query("SELECT 1").execute(&pool).await
        }));
    }

    for handle in handles {
        handle.await??;
    }

    tracing::info!(count = connections, "Connection pool warmed up");
    Ok(())
}

// Usage during startup
async fn main() -> Result<()> {
    let pool = sqlx::PgPool::connect("postgres://...").await?;

    // Warm up: establish 10 connections before accepting traffic
    warm_up_pool(&pool, 10).await?;

    // Start Axum server
    let app = Router::new().route("/users", get(list_users)).with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

---

### Q8. How do connection leaks occur and how do you prevent them?

**Interview Answer**

Connection leaks happen when a connection is acquired from the pool but never returned — either the code path exits without dropping the connection, or a panic prevents cleanup. In Rust, SQLx's connection types implement Drop, which automatically returns connections to the pool when they go out of scope. Leaks typically occur with: (1) `tokio::spawn` that holds a connection and panics, (2) long-lived `PoolConnection` in async tasks that never complete, (3) not dropping connections in error paths. Prevention: always scope connections tightly, use `?` to propagate errors (which drops the connection), avoid holding connections across `.await` points unnecessarily, and monitor pool size over time for gradual growth.

```rust
// LEAK: connection held across long-lived task
async fn leak_example(pool: &PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    // This connection is never returned if the task panics
    tokio::spawn(async move {
        long_running_task(&mut conn).await;  // Connection held for entire task
        // If panic occurs here, connection is dropped (Rust Drop handles it)
    });
    // conn moved into spawned task, not dropped here
}

// SAFE: connection returned promptly
async fn safe_example(pool: &PgPool) -> Result<()> {
    let result = {
        let mut conn = pool.acquire().await?;
        sqlx::query("SELECT 1").fetch_one(&mut *conn).await?
    };  // conn dropped here, returned to pool

    // Process result without holding connection
    process_result(result);
    Ok(())
}

// Monitor for leaks: pool size should stabilize
async fn detect_leaks(pool: &PgPool) {
    let mut prev_size = pool.size();
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        let current_size = pool.size();
        if current_size > prev_size + 5 {
            tracing::error!(
                prev = prev_size,
                current = current_size,
                "Possible connection leak detected"
            );
        }
        prev_size = current_size;
    }
}
```

---

### Q9. What is the difference between max_connections and pool_size?

**Interview Answer**

`max_connections` is a PostgreSQL server parameter that limits the total number of simultaneous connections to the database server (default: 100). It counts all connections from all clients, including PgBouncer. `pool_size` (or `max_connections` in SQLx) is an application-level parameter that limits how many connections your application (or PgBouncer) maintains. The relationship: your application's pool_size should be ≤ PgBouncer's default_pool_size, which should be ≤ PostgreSQL's max_connections. In a typical setup: 100 app instances × 20 SQLx connections = 2000 application connections, but PgBouncer's pool_size = 20, so only 20 actual PostgreSQL connections are used. This multiplexing is the whole point of PgBouncer.

```sql
-- PostgreSQL server limit
SHOW max_connections;  -- 100 (shared across ALL clients)

-- PgBouncer limits
# pgbouncer.ini
max_client_conn = 2000      # Accept 2000 app connections
default_pool_size = 20      # But only 20 actual DB connections

-- SQLx application pool
let pool = PgPoolOptions::new()
    .max_connections(20)    # Each app instance: 20 connections
    .connect(database_url)
    .await?;

-- If you have 10 app instances:
-- App connections: 10 × 20 = 200 (to PgBouncer)
-- PgBouncer connections: 20 (to PostgreSQL)
-- PostgreSQL sees only 20 connections, not 200
```

---

### Q10. How do you handle connection pool configuration in a production Rust deployment?

**Interview Answer**

Production connection pooling requires a layered approach: application pool (SQLx) → connection pooler (PgBouncer) → PostgreSQL. For SQLx, set `max_connections` based on your concurrency needs (typically 10-30 per instance), `acquire_timeout` to 3-5 seconds (fail fast), `idle_timeout` to 5 minutes (release unused connections), and `max_lifetime` to 30 minutes (prevent stale connections). For PgBouncer, use transaction pooling mode, `default_pool_size` of 20-50 per database, and `reserve_pool_size` of 5-10 for traffic spikes. Use environment variables for all settings to enable per-environment tuning. Monitor connection metrics and adjust based on load testing results. Always have health checks that verify database connectivity.

```rust
// Production-ready pool configuration using environment variables
async fn create_pool() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let max_connections: u32 = std::env::var("DB_POOL_MAX")
        .unwrap_or_else(|_| "20".to_string())
        .parse()?;
    let min_connections: u32 = std::env::var("DB_POOL_MIN")
        .unwrap_or_else(|_| "5".to_string())
        .parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(
            std::env::var("DB_ACQUIRE_TIMEOUT_SECS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?
        ))
        .idle_timeout(Duration::from_secs(
            std::env::var("DB_IDLE_TIMEOUT_SECS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()?
        ))
        .max_lifetime(Duration::from_secs(
            std::env::var("DB_MAX_LIFETIME_SECS")
                .unwrap_or_else(|_| "1800".to_string())
                .parse()?
        ))
        .connect(&database_url)
        .await?;

    tracing::info!(max_connections, min_connections, "Database pool initialized");
    Ok(pool)
}
```
