# How do you optimize PostgreSQL queries?

## Interview Question

How do you optimize slow PostgreSQL queries?

## Interview Answer

Optimizing PostgreSQL queries starts with using `EXPLAIN ANALYZE` to understand the query plan — look for sequential scans on large tables, excessive row estimates, sort operations, and nested loops where hash joins would be better. Create appropriate indexes (B-tree for range/equality, GIN for JSONB/full-text), avoid `SELECT *` to reduce I/O, push filters as close to the source as possible, and ensure `work_mem` is sufficient for sorts and joins. Use connection pooling (PgBouncer) to reduce connection overhead, batch inserts with COPY or multi-row INSERT, and consider materialized views for expensive aggregations. Monitor slow query logs with `log_min_duration_statement`.

---

## Follow-up Questions & Answers

### Q1. How do you read and interpret EXPLAIN ANALYZE output?

**Interview Answer**

EXPLAIN ANALYZE executes the query and shows the actual execution plan with real timing and row counts. Read the plan bottom-up — the leaf nodes are the actual data access methods (Seq Scan, Index Scan, Bitmap Scan), and each parent node shows how rows flow upward. Key metrics to watch: "actual time" (first value is time to first row, second is total time), "rows" vs "rows removed by Filter" (indicates selectivity), and "sort method" (external merge = disk spill). If estimated rows diverge significantly from actual rows, run ANALYZE to update statistics. Hash Join and Merge Join are generally preferred over Nested Loop for large datasets.

```sql
EXPLAIN ANALYZE
SELECT u.name, COUNT(o.id) AS order_count
FROM users u
INNER JOIN orders o ON u.id = o.user_id
WHERE u.created_at > '2025-01-01'
GROUP BY u.name
ORDER BY order_count DESC
LIMIT 10;

-- Key things to look for:
-- Seq Scan on users (bad if table is large, add index on created_at)
-- Nested Loop vs Hash Join (hash is better for large joins)
-- Sort Method: external merge (bad, increase work_mem)
-- Rows Estimated vs Actual (if very different, run ANALYZE)
```

---

### Q2. What is the difference between Seq Scan, Index Scan, Bitmap Scan, and Index Only Scan?

**Interview Answer**

Sequential Scan reads every row in the table — it's the fallback when no index can help and is fast for small tables or when returning most rows. Index Scan traverses the B-tree to find matching row pointers, then fetches each row from the heap — good for selective queries returning few rows. Bitmap Scan is a two-phase approach: first builds a bitmap of matching heap pages using the index, then fetches those pages in physical order — efficient for medium selectivity. Index Only Scan reads everything from the index without touching the heap at all — the fastest option, but only works when all needed columns are in the index and the visibility map confirms visibility.

```sql
-- Sequential Scan (no useful index)
EXPLAIN ANALYZE SELECT * FROM orders WHERE total > 100;
-- Output: Seq Scan on orders  (cost=0.00..1234.00 rows=50000)

-- Index Scan (selective, uses B-tree)
CREATE INDEX idx_orders_total ON orders (total);
EXPLAIN ANALYZE SELECT * FROM orders WHERE total > 999.99;
-- Output: Index Scan using idx_orders_total on orders

-- Bitmap Scan (medium selectivity)
EXPLAIN ANALYZE SELECT * FROM orders WHERE total > 50;
-- Output: Bitmap Heap Scan on orders  ->  Bitmap Index Scan on idx_orders_total

-- Index Only Scan (all columns in index)
CREATE INDEX idx_orders_total_covering ON orders (total) INCLUDE (id, customer_id);
EXPLAIN ANALYZE SELECT id, total FROM orders WHERE total > 999.99;
-- Output: Index Only Scan using idx_orders_total_covering
```

---

### Q3. How does connection pooling with PgBouncer improve PostgreSQL performance?

**Interview Answer**

PostgreSQL forks a new process for every connection, which is expensive in memory and CPU. PgBouncer sits between your application and PostgreSQL, maintaining a small pool of actual database connections and multiplexing many client connections over them. This dramatically reduces memory usage and eliminates the overhead of process creation/destruction. PgBouncer operates in three modes: transaction (best for most use cases — connection returned after each transaction), session (connection held for entire client session), and statement (autocommit only). For SQLx in Rust, you point your pool at PgBouncer's port instead of PostgreSQL directly, and PgBouncer handles connection lifecycle management.

```toml
# pgbouncer.ini
[databases]
mydb = host=127.0.0.1 port=5432 dbname=mydb

[pgbouncer]
pool_mode = transaction
max_client_conn = 1000
default_pool_size = 20
reserve_pool_size = 5
reserve_pool_timeout = 3
server_lifetime = 3600
server_idle_timeout = 600
```

```rust
// Rust SQLx connecting through PgBouncer
let pool = sqlx::PgPoolOptions::new()
    .max_connections(50)  // App-side pool (PgBouncer handles DB-side)
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect("postgres://user:pass@localhost:6432/mydb")  // PgBouncer port
    .await?;
```

---

### Q4. What is query plan caching in PostgreSQL and how does it differ from application-level caching?

**Interview Answer**

PostgreSQL uses a prepared statement cache (plan cache) for prepared statements and simple queries. When you execute a query with parameters, PostgreSQL may reuse a previously computed plan if the query structure is identical. However, PostgreSQL re-plans when data distribution changes significantly (after ANALYZE). Application-level caching (like Redis) stores actual query results, avoiding database round-trips entirely. SQLx's `query_as!` macro generates prepared statements that benefit from PostgreSQL's plan cache. For high-frequency read queries with stable results, adding Redis caching on top provides a second layer that avoids even the plan execution overhead. Use cache invalidation strategies like TTL or event-driven invalidation.

```rust
// SQLx prepared statement benefits from PostgreSQL plan cache
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
    .fetch_optional(&pool)
    .await?;

// With Redis caching layer
async fn get_user(pool: &PgPool, redis: &RedisPool, user_id: i64) -> Option<User> {
    let cache_key = format!("user:{}", user_id);

    // Check cache first
    if let Some(cached) = redis.get::<User>(&cache_key).await? {
        return Some(cached);
    }

    // Query database
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await?;

    // Populate cache with TTL
    if let Some(ref u) = user {
        redis.set_ex(&cache_key, u, 300).await?;
    }

    user
}
```

---

### Q5. What are the key PostgreSQL configuration parameters for performance tuning?

**Interview Answer**

The most impactful parameters are `shared_buffers` (set to 25% of system RAM — holds frequently accessed data in memory), `work_mem` (per-operation memory for sorts and hash joins — increase if EXPLAIN shows external merge sorts or hash batch spills), `effective_cache_size` (set to 75% of RAM — tells planner about OS cache), and `maintenance_work_mem` (set to 256MB+ for VACUUM and CREATE INDEX speed). `max_connections` should match your PgBouncer pool size, not your application's concurrent users. `random_page_cost` should be set to 1.1 for SSDs (default is 4.0, designed for HDDs). Enable `log_min_duration_statement` to log slow queries. Always use `pg_settings` to tune without restarting when possible.

```sql
-- Check current settings
SHOW shared_buffers;
SHOW work_mem;
SHOW effective_cache_size;

-- Tune without restart (most parameters)
SET work_mem = '64MB';
SET effective_cache_size = '12GB';

-- Permanent changes in postgresql.conf or ALTER SYSTEM
ALTER SYSTEM SET shared_buffers = '4GB';
ALTER SYSTEM SET work_mem = '32MB';
ALTER SYSTEM SET effective_cache_size = '12GB';
ALTER SYSTEM SET random_page_cost = 1.1;  -- SSD optimization
ALTER SYSTEM SET log_min_duration_statement = 1000;  -- Log queries > 1 second

SELECT pg_reload_conf();
```

---

### Q6. When should you denormalize a PostgreSQL schema for performance?

**Interview Answer**

Denormalize when read performance is critical and JOINs are the bottleneck — typically in analytics dashboards, high-traffic read-heavy APIs, and reporting systems. Common denormalization strategies include: storing pre-computed aggregates (materialized views), duplicating frequently-read columns (e.g., storing `customer_name` on the `orders` table), and creating summary tables. The trade-off is increased write complexity and storage. Always measure first with `EXPLAIN ANALYZE` — most query problems are solved by proper indexing rather than denormalization. In a Rust backend, you can use denormalized read models (CQRS pattern) with SQLx while keeping normalized write models.

```sql
-- Denormalized order summary table
CREATE TABLE order_summaries AS
SELECT o.id, o.created_at, c.name AS customer_name, c.email AS customer_email,
       SUM(li.quantity * li.price) AS total
FROM orders o
JOIN customers c ON o.customer_id = c.id
JOIN line_items li ON li.order_id = o.id
GROUP BY o.id, c.name, c.email;

-- Materialized view (better than manual denormalization)
CREATE MATERIALIZED VIEW mv_order_summaries AS
SELECT o.id, o.created_at, c.name AS customer_name,
       SUM(li.quantity * li.price) AS total
FROM orders o
JOIN customers c ON o.customer_id = c.id
JOIN line_items li ON li.order_id = o.id
GROUP BY o.id, c.name;

-- Refresh periodically
REFRESH MATERIALIZED VIEW CONCURRENTLY mv_order_summaries;
```

---

### Q7. How do you optimize batch INSERT performance in PostgreSQL with SQLx?

**Interview Answer**

Single-row INSERT statements are extremely slow in PostgreSQL because each requires a separate round-trip, plan, and WAL write. Use multi-row INSERT (`INSERT INTO t VALUES (...), (...), (...)`) or PostgreSQL's COPY protocol for bulk loading. SQLx supports multi-row inserts via `sqlx::query()` with `.bind()` in a loop, but the most efficient approach is using `sqlx::raw_sql()` with a COPY command or a multi-row VALUES clause. For batch sizes, 100-1000 rows per INSERT balances memory usage and performance. Disable indexes during bulk loading and re-enable them afterward if inserting millions of rows. Use `ON CONFLICT` for upserts rather than separate SELECT + INSERT logic.

```rust
// Efficient batch INSERT with SQLx
async fn batch_insert_users(pool: &PgPool, users: &[NewUser]) -> Result<u64> {
    let mut tx = pool.begin().await?;

    for chunk in users.chunks(500) {
        let mut query = String::from("INSERT INTO users (name, email) VALUES ");

        for (i, user) in chunk.iter().enumerate() {
            if i > 0 {
                query.push(',');
            }
            query.push_str(&format!("('{}', '{}')", user.name, user.email));
        }

        query.push_str(" ON CONFLICT (email) DO NOTHING");
        sqlx::raw_sql(&query).execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(users.len() as u64)
}

// Using SQLx with bindings (cleaner but slightly slower)
let mut query = "INSERT INTO users (name, email) VALUES ".to_string();
let mut bind_count = 0;
for user in &users {
    if bind_count > 0 { query.push(','); }
    bind_count += 1;
    query.push_str(&format!("(${}, ${})", bind_count, bind_count + 1));
    bind_count += 1;
}
```

---

### Q8. What is the N+1 query problem and how do you solve it in PostgreSQL?

**Interview Answer**

The N+1 problem occurs when you execute one query to fetch N parent rows, then N additional queries to fetch related data for each parent — resulting in N+1 total queries. This is devastating for performance because each query incurs network latency and connection overhead. Solutions include: using JOINs to fetch everything in one query, using `ANY` with array parameters (PostgreSQL-specific), batching with IN clauses, or using LATERAL JOINs for correlated subqueries. In Rust with SQLx, the pattern is to fetch all IDs first, then use `WHERE id = ANY($1)` with a PostgreSQL array to fetch all related data in one query.

```rust
// BAD: N+1 queries
let orders = sqlx::query_as!(Order, "SELECT * FROM orders WHERE customer_id = $1", cid)
    .fetch_all(&pool).await?;

for order in &orders {
    // This executes a query PER ORDER — N+1!
    let items = sqlx::query_as!(Item, "SELECT * FROM line_items WHERE order_id = $1", order.id)
        .fetch_all(&pool).await?;
}

// GOOD: Single query with JOIN
let orders_with_items = sqlx::query!(
    "SELECT o.id, o.total, li.product_name, li.quantity
     FROM orders o
     LEFT JOIN line_items li ON o.id = li.order_id
     WHERE o.customer_id = $1",
    cid
).fetch_all(&pool).await?;

// GOOD: Batch fetch with ANY (PostgreSQL array)
let order_ids: Vec<i64> = orders.iter().map(|o| o.id).collect();
let items = sqlx::query_as!(
    LineItem,
    "SELECT * FROM line_items WHERE order_id = ANY($1)",
    &order_ids
).fetch_all(&pool).await?;
```

---

### Q9. How does VACUUM affect query performance and when should you run it?

**Interview Answer**

PostgreSQL uses MVCC, which means UPDATE and DELETE operations leave dead tuples (old row versions) behind. These dead tuples consume storage and slow down sequential scans because the scanner must check each tuple's visibility. VACUUM reclaims dead tuple space and updates the visibility map, which enables Index Only Scans. Autovacuum runs automatically but may not keep up with heavy write workloads. You should manually run VACUUM ANALYZE after bulk operations (mass UPDATE, DELETE, or COPY) to update table statistics. A bloated table (check with `pgstattuple`) has significantly worse performance because more pages must be read per query.

```sql
-- Check table bloat
SELECT relname, n_live_tup, n_dead_tup,
       round(100.0 * n_dead_tup / NULLIF(n_live_tup + n_dead_tup, 0), 2) AS dead_pct,
       last_vacuum, last_autovacuum, last_analyze
FROM pg_stat_user_tables
WHERE schemaname = 'public'
ORDER BY n_dead_tup DESC;

-- Manual vacuum after bulk operations
VACUUM ANALYZE orders;

-- Full vacuum (reclaims space, but blocks other operations)
VACUUM FULL orders;  -- Use sparingly, requires exclusive lock

-- Check autovacuum settings
SHOW autovacuum_vacuum_threshold;
SHOW autovacuum_vacuum_scale_factor;

-- Tune for heavily updated table
ALTER TABLE orders SET (autovacuum_vacuum_scale_factor = 0.01);
```

---

### Q10. What are prepared statements in PostgreSQL and how does SQLx use them?

**Interview Answer**

Prepared statements parse, plan, and optimize a query once, then allow it to be executed multiple times with different parameters without re-parsing. PostgreSQL's extended query protocol sends Parse, Bind, and Execute messages separately. SQLx uses this protocol by default — every `sqlx::query()` call creates a prepared statement that PostgreSQL caches. For repeated queries with different parameters, this avoids the overhead of re-planning. SQLx's `sqlx::query!` macro additionally verifies query correctness at compile time by connecting to the database during build. You can also manually prepare statements using `sqlx::query.prepare()` if you want explicit control over plan reuse. The `statement_cache_capacity` in PgPoolOptions controls how many prepared statements SQLx caches.

```rust
// SQLx automatically uses prepared statements
// This query is planned once, executed many times
async fn find_users(pool: &PgPool, emails: &[String]) -> Result<Vec<User>> {
    let mut users = Vec::new();

    // Each call reuses the prepared statement plan
    for email in emails {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;
        if let Some(u) = user {
            users.push(u);
        }
    }

    Ok(users)
}

// Better: use ANY with array (single prepared statement)
async fn find_users_batch(pool: &PgPool, emails: &[String]) -> Result<Vec<User>> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = ANY($1)", emails)
        .fetch_all(pool)
        .await
}

// Control prepared statement cache size
let pool = sqlx::PgPoolOptions::new()
    .max_connections(20)
    .statement_cache_capacity(200)  // Cache up to 200 prepared statements
    .connect("postgres://...")
    .await?;
```
