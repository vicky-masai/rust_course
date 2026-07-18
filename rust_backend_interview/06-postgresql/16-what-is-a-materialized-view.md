# What is a Materialized View?

## Interview Question

What is a materialized view in PostgreSQL and when should you use one?

## Interview Answer

A materialized view is a database object that stores the result of a query physically on disk, like a pre-computed table. Unlike a regular view (which executes the query every time it's accessed), a materialized view caches the result and must be explicitly refreshed to update. Use materialized views for expensive aggregations, complex JOINs across many tables, and frequently-accessed read-heavy queries where the data doesn't need to be real-time. They bridge normalization (logical design) and performance (physical optimization). The trade-off is staleness — data is only as fresh as the last refresh. PostgreSQL supports `REFRESH MATERIALIZED VIEW CONCURRENTLY` for non-blocking refreshes when a unique index exists.

---

## Follow-up Questions & Answers

### Q1. What is the difference between a regular view and a materialized view?

**Interview Answer**

A regular view is a saved query that executes every time it's accessed — no data is stored, results are always current, and it adds no storage overhead. A materialized view stores the query result physically on disk, making reads fast but requiring explicit refreshes to stay current. Regular views are virtual (zero storage cost), materialized views are physical (consume disk space). Use regular views for simplicity and always-current data. Use materialized views for performance when the underlying query is expensive and staleness is acceptable.

```sql
-- Regular view (always current, no storage)
CREATE VIEW active_orders AS
SELECT o.id, o.total, c.name
FROM orders o JOIN customers c ON o.customer_id = c.id
WHERE o.status = 'active';

SELECT * FROM active_orders;
-- Executes the full JOIN every time

-- Materialized view (stored result, needs refresh)
CREATE MATERIALIZED VIEW mv_active_orders AS
SELECT o.id, o.total, c.name
FROM orders o JOIN customers c ON o.customer_id = c.id
WHERE o.status = 'active';

SELECT * FROM mv_active_orders;
-- Reads from disk, instant

-- Refresh to update
REFRESH MATERIALIZED VIEW mv_active_orders;
```

---

### Q2. How do you refresh a materialized view and what are the refresh strategies?

**Interview Answer**

Refresh strategies: (1) Full refresh (REFRESH MATERIALIZED VIEW) — rebuilds the entire view, blocks reads during refresh; (2) Concurrent refresh (REFRESH MATERIALIZED VIEW CONCURRENTLY) — requires a unique index, allows reads during refresh, slightly slower; (3) Periodic refresh — schedule via pg_cron, cron jobs, or application-level timers; (4) Event-driven refresh — refresh after specific data changes using triggers or application logic. For small views, full refresh is fine. For large views used in production, use concurrent refresh to avoid downtime. The refresh frequency depends on how stale your data can be — hourly for dashboards, daily for reports.

```sql
-- Full refresh (blocks reads)
REFRESH MATERIALIZED VIEW mv_dashboard_stats;

-- Concurrent refresh (allows reads, needs unique index)
CREATE UNIQUE INDEX idx_mv_dashboard_id ON mv_dashboard_stats (id);
REFRESH MATERIALIZED VIEW CONCURRENTLY mv_dashboard_stats;

-- Schedule with pg_cron (extension)
CREATE EXTENSION IF NOT EXISTS pg_cron;
SELECT cron.schedule('refresh-dashboard', '0 * * * *',
    'REFRESH MATERIALIZED VIEW CONCURRENTLY mv_dashboard_stats');

-- Refresh from Rust application
async fn scheduled_refresh(pool: &sqlx::PgPool) -> Result<()> {
    loop {
        sqlx::query("REFRESH MATERIALIZED VIEW CONCURRENTLY mv_dashboard_stats")
            .execute(pool).await?;
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
```

---

### Q3. How do you create an indexed materialized view?

**Interview Answer**

Materialized views don't automatically have indexes — you must create them manually after the materialized view. A unique index is required for CONCURRENTLY refresh. Create indexes based on your query patterns: index columns used in WHERE clauses and foreign keys used for JOINs. The index creation is a one-time cost during setup; refreshes rebuild the indexes automatically. Without indexes, queries on materialized views fall back to sequential scans, negating the performance benefit.

```sql
-- Create materialized view
CREATE MATERIALIZED VIEW mv_product_sales AS
SELECT p.id, p.name, p.category_id,
       COUNT(oi.id) AS total_orders,
       SUM(oi.quantity * oi.price) AS total_revenue
FROM products p
LEFT JOIN order_items oi ON p.id = oi.product_id
GROUP BY p.id, p.name, p.category_id;

-- Required: unique index for CONCURRENTLY refresh
CREATE UNIQUE INDEX idx_mv_product_sales_id ON mv_product_sales (id);

-- Query-pattern indexes
CREATE INDEX idx_mv_product_sales_category ON mv_product_sales (category_id);
CREATE INDEX idx_mv_product_sales_revenue ON mv_product_sales (total_revenue DESC);

-- Now queries are fast
SELECT * FROM mv_product_sales WHERE category_id = 5 ORDER BY total_revenue DESC;
```

---

### Q4. What are the limitations of materialized views?

**Interview Answer**

Limitations: (1) Data is stale until refreshed — not suitable for real-time requirements; (2) Refresh locks the view (full mode) or requires a unique index (concurrent mode); (3) No incremental refresh — PostgreSQL rebuilds the entire view; (4) No triggers or foreign keys on materialized views; (5) Cannot INSERT/UPDATE/DELETE directly; (6) Storage overhead for large views; (7) Refresh can be expensive for complex queries on large datasets; (8) No automatic refresh — must be scheduled or triggered externally. For real-time needs, use regular views. For near-real-time, use frequent concurrent refreshes or the pg_ivm extension for incremental maintenance.

```sql
-- Limitation: no direct modifications
INSERT INTO mv_product_sales VALUES (1, 'test', 1, 0, 0);
-- ERROR: cannot insert into materialized view "mv_product_sales"

-- Limitation: no foreign keys
ALTER TABLE mv_product_sales ADD FOREIGN KEY (category_id) REFERENCES categories(id);
-- ERROR: cannot create foreign key on materialized view

-- Workaround: use a regular table with materialized view logic
CREATE TABLE product_sales_cache AS
SELECT * FROM mv_product_sales;
-- Now you have a regular table with indexes, FKs, triggers
```

---

### Q5. How do you use materialized views with SQLx in Rust?

**Interview Answer**

Materialized views are queried like regular tables in SQLx — use `query_as!` with your struct. You cannot INSERT/UPDATE/DELETE into them, and REFRESH is done via `sqlx::query()`. In Rust, schedule refreshes using tokio::spawn with a timer, or use pg_cron for database-side scheduling. SQLx's compile-time verification works with materialized views if you have them in your development database.

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct ProductSales {
    id: i64,
    name: String,
    category_id: i64,
    total_orders: i64,
    total_revenue: rust_decimal::Decimal,
}

// Query materialized view (like any table)
async fn get_top_products(pool: &sqlx::PgPool, limit: i64) -> Result<Vec<ProductSales>> {
    sqlx::query_as!(ProductSales,
        "SELECT id, name, category_id, total_orders, total_revenue
         FROM mv_product_sales
         ORDER BY total_revenue DESC
         LIMIT $1", limit
    )
    .fetch_all(pool)
    .await
}

// Refresh materialized view
async fn refresh_product_sales(pool: &sqlx::PgPool) -> Result<()> {
    sqlx::query("REFRESH MATERIALIZED VIEW CONCURRENTLY mv_product_sales")
        .execute(pool)
        .await?;
    Ok(())
}

// Background refresh task
async fn start_refresh_task(pool: sqlx::PgPool) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(3600)).await;
            match refresh_product_sales(&pool).await {
                Ok(_) => tracing::info!("Materialized view refreshed"),
                Err(e) => tracing::error!("Refresh failed: {}", e),
            }
        }
    });
}
```

---

### Q6. When should you use a materialized view versus a regular table with triggers?

**Interview Answer**

Use materialized views when: data can be slightly stale, the underlying query is complex/expensive, you want zero application-level maintenance, and refresh frequency is acceptable (hourly/daily). Use a regular table with triggers when: you need real-time data, need foreign keys or constraints on the cached data, need to modify the cached data directly, or need incremental updates without full refresh. Triggers provide immediate consistency but add write overhead. Materialized views are simpler to maintain but have refresh latency. For high-frequency updates, consider the pg_ivm extension for incremental materialized views.

```sql
-- Materialized view: simple, batch refresh
CREATE MATERIALIZED VIEW mv_dashboard AS
SELECT customer_id, COUNT(*) AS orders, SUM(total) AS revenue
FROM orders GROUP BY customer_id;

-- Regular table with trigger: real-time, more complex
CREATE TABLE dashboard_cache (
    customer_id INT PRIMARY KEY,
    orders INT DEFAULT 0,
    revenue DECIMAL DEFAULT 0
);

CREATE OR REPLACE FUNCTION update_dashboard() RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO dashboard_cache (customer_id, orders, revenue)
    VALUES (NEW.customer_id, 1, NEW.total)
    ON CONFLICT (customer_id) DO UPDATE SET
        orders = dashboard_cache.orders + 1,
        revenue = dashboard_cache.revenue + EXCLUDED.revenue;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_dashboard AFTER INSERT ON orders
    FOR EACH ROW EXECUTE FUNCTION update_dashboard();
```

---

### Q7. How do you handle materialized view staleness in your application?

**Interview Answer**

Strategies for handling staleness: (1) Accept it — display "data as of [timestamp]" to users; (2) Hybrid approach — query materialized view for most data, fall back to live query for recent changes; (3) Version the view — include a refresh timestamp and check it client-side; (4) Cache invalidation — refresh the view when relevant data changes (event-driven); (5) Tiered freshness — use materialized views for historical data, live queries for today's data. In Rust, implement a wrapper that checks staleness and optionally queries fresh data.

```rust
async fn get_dashboard(pool: &PgPool, max_age_secs: i64) -> Result<Dashboard> {
    // Check when the materialized view was last refreshed
    let last_refresh = sqlx::query_scalar!(
        "SELECT EXTRACT(EPOCH FROM (now() - pg_catalog.pg_stat_get_last_autoanalyze_time(
            (SELECT oid FROM pg_class WHERE relname = 'mv_dashboard')
        )))::int"
    )
    .fetch_one(pool)
    .await?;

    // If stale, refresh first
    if last_refresh.unwrap_or(0) > max_age_secs {
        sqlx::query("REFRESH MATERIALIZED VIEW CONCURRENTLY mv_dashboard")
            .execute(pool).await?;
    }

    // Query the materialized view
    let data = sqlx::query_as!(Dashboard, "SELECT * FROM mv_dashboard")
        .fetch_all(pool).await?;

    Ok(Dashboard { data, refreshed_at: chrono::Utc::now() })
}
```

---

### Q8. Can you create a partial materialized view?

**Interview Answer**

PostgreSQL doesn't support `WHERE` clauses directly in CREATE MATERIALIZED VIEW, but you can achieve partial materialization by including the filter in the AS query and creating a partial index on the view. This limits what's stored in the view, reducing size and refresh time. For truly partial views, use a regular table with a partial index and refresh logic. The pg_ivm extension (Incremental View Maintenance) can maintain partial views more efficiently.

```sql
-- Materialized view with built-in filter
CREATE MATERIALIZED VIEW mv_recent_orders AS
SELECT id, customer_id, total, created_at
FROM orders
WHERE created_at > NOW() - INTERVAL '90 days'
GROUP BY id, customer_id, total, created_at;

-- Partial index on the materialized view
CREATE INDEX idx_mv_recent_orders_customer ON mv_recent_orders (customer_id);

-- Query only recent data (uses partial index)
SELECT * FROM mv_recent_orders
WHERE customer_id = 42 AND created_at > NOW() - INTERVAL '30 days';

-- Refresh only includes recent data (smaller, faster)
REFRESH MATERIALIZED VIEW CONCURRENTLY mv_recent_orders;
```

---

### Q9. What is the pg_ivm extension and how does it relate to materialized views?

**Interview Answer**

pg_ivm (Incremental View Maintenance) is a PostgreSQL extension that automatically maintains materialized views incrementally — instead of rebuilding the entire view on refresh, it applies only the changes (deltas). This is dramatically faster for large materialized views with infrequent changes. pg_ivm creates "incremental materialized views" using `CREATE INCREMENTAL MATERIALIZED VIEW`. It handles INSERT, UPDATE, and DELETE by computing deltas and applying them. Limitations: not all queries are supported (complex aggregates, certain JOINs), and it's still experimental. For most production use cases, standard materialized views with CONCURRENTLY refresh are more reliable.

```sql
-- Install pg_ivm extension
CREATE EXTENSION pg_ivm;

-- Create incremental materialized view
CREATE INCREMENTAL MATERIALIZED VIEW mv_orders_summary AS
SELECT customer_id, COUNT(*) AS order_count, SUM(total) AS total_spent
FROM orders
GROUP BY customer_id;

-- When orders are inserted/updated/deleted, the view updates automatically
INSERT INTO orders (customer_id, total) VALUES (1, 100);
-- mv_orders_summary is automatically updated (no REFRESH needed!)

-- Check maintenance status
SELECT * FROM pg_ivm_maintenance;
```

---

### Q10. How do materialized views compare to Redis caching for read performance?

**Interview Answer**

Materialized views are database-native, SQL-queryable, persistent (survive restarts), and require no additional infrastructure. They're slower to refresh (seconds to minutes) but provide consistent SQL semantics. Redis caching is faster (microseconds), supports arbitrary data structures, and can be updated in real-time, but requires additional infrastructure, has no SQL interface, and data can be lost on restart. Use materialized views for complex analytical queries and dashboards. Use Redis for session data, real-time counters, and frequently-accessed simple lookups. Many production systems use both: materialized views for complex reads, Redis for hot data.

```sql
-- Materialized view for complex analytics
CREATE MATERIALIZED VIEW mv_sales_dashboard AS
SELECT
    DATE_TRUNC('day', o.created_at) AS day,
    p.category,
    SUM(oi.quantity) AS units_sold,
    SUM(oi.quantity * oi.price) AS revenue
FROM orders o
JOIN order_items oi ON o.id = oi.order_id
JOIN products p ON oi.product_id = p.id
WHERE o.status = 'completed'
GROUP BY 1, 2;

-- Redis for real-time counters (application-level)
-- GET product:42:views -> "1234"
-- INCR product:42:views
-- EXPIRE product:42:views 3600

-- Hybrid: materialized view for daily, Redis for hourly
-- Materialized view refreshed daily for historical accuracy
-- Redis updated in real-time for current day's data
```
