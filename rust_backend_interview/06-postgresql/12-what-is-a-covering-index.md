# What is a Covering Index?

## Interview Question

What is a covering index in PostgreSQL?

## Interview Answer

A covering index is an index that contains all the columns a query needs, allowing PostgreSQL to satisfy the query entirely from the index without accessing the table heap. This is called an index-only scan and is the fastest possible data access method because it avoids random I/O to the main table. In PostgreSQL, you create covering indexes using the B-tree index type with the INCLUDE clause, which adds "payload" columns to the leaf pages that are not part of the search key. The visibility map must also confirm all tuples on a page are visible to all transactions for an index-only scan to work — if not, PostgreSQL falls back to fetching from the heap.

---

## Follow-up Questions & Answers

### Q1. What is the difference between a covering index and a regular index?

**Interview Answer**

A regular B-tree index stores the indexed column(s) in its tree structure and points to the heap (table) to retrieve other columns. A covering index adds additional columns to the index leaf pages via the INCLUDE clause, so the index "covers" the query — all needed columns are in the index itself. The INCLUDE columns are not part of the B-tree search key (they don't affect sort order or search performance), but they are stored in the leaf pages alongside the key columns. This means the index is slightly larger than a regular index, but index-only scans avoid heap fetches entirely, making reads significantly faster.

```sql
-- Regular index (must fetch from heap for non-index columns)
CREATE INDEX idx_orders_customer ON orders (customer_id);
EXPLAIN ANALYZE SELECT id, customer_id, total FROM orders WHERE customer_id = 42;
-- Index Scan using idx_orders_customer on orders
--   -> must fetch id and total from heap

-- Covering index (no heap fetch needed)
CREATE INDEX idx_orders_covering ON orders (customer_id) INCLUDE (id, total);
EXPLAIN ANALYZE SELECT id, customer_id, total FROM orders WHERE customer_id = 42;
-- Index Only Scan using idx_orders_covering on orders
--   -> all data comes from the index

-- Size comparison
SELECT pg_size_pretty(pg_relation_size('idx_orders_customer'));
-- 5MB
SELECT pg_size_pretty(pg_relation_size('idx_orders_covering'));
-- 12MB (larger, but faster reads)
```

---

### Q2. How does the visibility map affect index-only scans?

**Interview Answer**

PostgreSQL uses a visibility map to track which pages in a table have all tuples visible to all transactions (all-frozen). For an index-only scan to work without consulting the heap, the page must be all-visible — otherwise, PostgreSQL must visit the heap to check tuple visibility, defeating the purpose of the index-only scan. The visibility map is maintained by VACUUM. If you see "Heap Fetches" in EXPLAIN output increasing, it means VACUUM hasn't made pages all-visible yet. Running VACUUM ANALYZE updates the visibility map and enables more index-only scans. The visibility map is stored in shared memory and is very small.

```sql
-- Check visibility map status
SELECT relname, pg_size_pretty(pg_relation_size(relid)) AS size,
       pg_size_pretty(pg_total_relation_size(relid)) AS total_size,
       round(100.0 * n_live_tup / NULLIF(n_live_tup + n_dead_tup, 0), 2) AS live_pct
FROM pg_stat_user_tables
WHERE schemaname = 'public' AND relname = 'orders';

-- Force visibility map update
VACUUM orders;

-- Monitor index-only scan efficiency
EXPLAIN (ANALYZE, BUFFERS)
SELECT id, customer_id, total FROM orders WHERE customer_id = 42;
-- Index Only Scan ... Heap Fetches: 0  (perfect!)
-- If Heap Fetches > 0, the page wasn't all-visible

-- After heavy writes, Heap Fetches increases
-- Run VACUUM to reset
VACUUM orders;
-- Now Heap Fetches should be 0 again
```

---

### Q3. When should you use INCLUDE columns versus just adding columns to the index key?

**Interview Answer**

Use INCLUDE when the extra columns don't need to be searched or sorted on — they're just payload data that you want to retrieve without a heap fetch. Adding columns to the index key makes them part of the B-tree sort order, which affects search performance and index size. INCLUDE columns are stored in leaf pages only (not internal pages), making the index smaller than if those columns were part of the key. INCLUDE is also useful when you want a composite key but some columns are only needed for retrieval. PostgreSQL 11+ supports INCLUDE with B-tree indexes. The INCLUDE clause does not affect the index's ability to be used for range scans on the key columns.

```sql
-- Key column: affects sort order and search
-- INCLUDE column: only stored in leaf pages for retrieval
CREATE INDEX idx_orders_covering ON orders (customer_id, order_date) INCLUDE (total, status);
-- customer_id and order_date: searchable, sortable
-- total and status: only for retrieval (index-only scan)

-- This query uses the covering index efficiently
SELECT total, status FROM orders WHERE customer_id = 42 AND order_date > '2025-07-01';
-- Index Only Scan: all data from index

-- Without INCLUDE (worse)
CREATE INDEX idx_orders_key_only ON orders (customer_id, order_date, total, status);
-- total and status are part of the key, making index larger
-- Internal pages store total and status (wasted space)
-- Search on customer_id still works but index is bigger

-- Size comparison
SELECT pg_size_pretty(pg_relation_size('idx_orders_covering'));
-- 8MB
SELECT pg_size_pretty(pg_relation_size('idx_orders_key_only'));
-- 15MB (almost double!)
```

---

### Q4. How do you create a covering index using SQLx migrations in Rust?

**Interview Answer**

You create covering indexes through SQLx migrations, which are raw SQL files executed by `sqlx::migrate!()`. The CREATE INDEX statement with the INCLUDE clause is standard SQL that PostgreSQL supports. Place migration files in the `migrations/` directory with a timestamp prefix. SQLx will run them in order during application startup. The covering index should target your most frequent read queries — analyze slow queries with EXPLAIN ANALYZE to identify which queries would benefit from covering indexes.

```sql
-- Migration: migrations/20250715000000_add_orders_covering_index.sql
CREATE INDEX CONCURRENTLY idx_orders_customer_covering
    ON orders (customer_id, created_at DESC)
    INCLUDE (total, status, shipping_address)
    WHERE status != 'cancelled';  -- Partial covering index
```

```rust
// Run migrations during startup
#[tokio::main]
async fn main() -> Result<()> {
    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Run all pending migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // Query benefits from the covering index
    let orders = sqlx::query!(
        "SELECT total, status, shipping_address
         FROM orders
         WHERE customer_id = $1 AND created_at > $2 AND status != 'cancelled'
         ORDER BY created_at DESC
         LIMIT 20",
        customer_id,
        since_date
    )
    .fetch_all(&pool)
    .await?;

    Ok(())
}
```

---

### Q5. What is the difference between an index-only scan and a covering index?

**Interview Answer**

An index-only scan is the access method PostgreSQL uses when all data needed by a query is available in the index. A covering index is the index structure that enables index-only scans by including all required columns. The covering index is the cause; the index-only scan is the effect. You can have an index-only scan without an INCLUDE clause if all the query's columns happen to already be in the index key. The INCLUDE clause explicitly adds payload columns to ensure index-only scans are possible. Both concepts are related but distinct: covering index = index design; index-only scan = execution plan node.

```sql
-- Index-only scan WITHOUT INCLUDE (works but index is less efficient)
CREATE INDEX idx_users_email_name ON users (email, name);
EXPLAIN ANALYZE SELECT email, name FROM users WHERE email = 'alice@example.com';
-- Index Only Scan using idx_users_email_name
-- Works because both email and name are in the key

-- Index-only scan WITH INCLUDE (better design)
CREATE INDEX idx_users_email ON users (email) INCLUDE (name);
EXPLAIN ANALYZE SELECT email, name FROM users WHERE email = 'alice@example.com';
-- Index Only Scan using idx_users_email
-- name is in INCLUDE, not in the key — smaller internal pages

-- Verify with EXPLAIN (ANALYZE, BUFFERS)
EXPLAIN (ANALYZE, BUFFERS)
SELECT email, name FROM users WHERE email = 'alice@example.com';
-- Index Only Scan: Heap Fetches: 0 (all-visible page, data from index)
-- Buffers: shared hit=3 (only index pages, no heap pages)
```

---

### Q6. How do you know when a covering index would help?

**Interview Answer**

Look for these indicators in EXPLAIN ANALYZE output: (1) Index Scan nodes — if the query uses an Index Scan but could use Index Only Scan, a covering index would help; (2) High "Heap Fetches" in Index Only Scan output — the index exists but doesn't cover all columns; (3) Queries that SELECT only 2-4 columns from a table — easy to cover with INCLUDE; (4) Queries with a selective WHERE clause and a few columns in SELECT — ideal candidate. Use `EXPLAIN (ANALYZE, BUFFERS)` to see if the plan accesses heap pages. If you see `shared read` on the table but `shared hit` on the index, the covering index would eliminate the heap access.

```sql
-- Before: Index Scan (fetches from heap)
EXPLAIN (ANALYZE, BUFFERS)
SELECT customer_id, total, status FROM orders WHERE customer_id = 42;
-- Index Scan using idx_orders_customer on orders
--   Buffers: shared hit=50 (index) shared read=120 (heap!)
--   Heap Fetches: 150

-- Create covering index
CREATE INDEX idx_orders_covering ON orders (customer_id) INCLUDE (total, status);

-- After: Index Only Scan (no heap access)
EXPLAIN (ANALYZE, BUFFERS)
SELECT customer_id, total, status FROM orders WHERE customer_id = 42;
-- Index Only Scan using idx_orders_covering on orders
--   Buffers: shared hit=50 (index only, no heap!)
--   Heap Fetches: 0
```

---

### Q7. Can you create a covering index for queries with ORDER BY?

**Interview Answer**

Yes, you can create covering indexes that support both WHERE clauses and ORDER BY. The key columns in the index should match the WHERE and ORDER BY columns in order: first the equality columns (WHERE), then the range columns, then the ORDER BY columns. The INCLUDE clause holds columns only needed for retrieval. This avoids both a sort and heap fetches. For descending order, PostgreSQL 11+ supports DESC in the index definition. For mixed ASC/DESC, use the column specification in the index. Partial covering indexes (with WHERE) can further reduce index size.

```sql
-- Query with WHERE and ORDER BY
EXPLAIN ANALYZE
SELECT customer_id, total, status, created_at
FROM orders
WHERE customer_id = 42
ORDER BY created_at DESC
LIMIT 10;
-- Index Scan + Sort (needs both index and sort)

-- Covering index for WHERE + ORDER BY
CREATE INDEX idx_orders_cust_created ON orders (customer_id, created_at DESC)
    INCLUDE (total, status);

EXPLAIN ANALYZE
SELECT customer_id, total, status, created_at
FROM orders
WHERE customer_id = 42
ORDER BY created_at DESC
LIMIT 10;
-- Index Only Scan (no sort, no heap fetch!)
-- Scans only 10 rows in order

-- Composite WHERE with ORDER BY
CREATE INDEX idx_orders_multi ON orders (customer_id, status, created_at DESC)
    INCLUDE (total);
-- Supports: WHERE customer_id = X AND status = 'active' ORDER BY created_at DESC
```

---

### Q8. What is the storage overhead of INCLUDE columns?

**Interview Answer**

INCLUDE columns add storage overhead only in the B-tree leaf pages — they are not stored in internal (non-leaf) pages, which means the B-tree search and traversal performance is not affected. The overhead depends on the size of the included columns: a single int column adds ~4 bytes per leaf entry, while a VARCHAR(255) adds ~255 bytes. For a table with 1 million rows, a covering index with a small INT INCLUDE column adds roughly 4MB to the index. This is usually worth the trade-off because index-only scans avoid the much larger overhead of heap page accesses. Monitor index size with `pg_relation_size()` and compare to the benefit in query performance.

```sql
-- Measure covering index size overhead
CREATE INDEX idx_orders_regular ON orders (customer_id);
CREATE INDEX idx_orders_covering ON orders (customer_id) INCLUDE (total, status, created_at);

SELECT indexname, pg_size_pretty(pg_relation_size(indexname::regclass)) AS size
FROM pg_indexes
WHERE tablename = 'orders' AND indexname LIKE 'idx_orders%';

-- Typical output:
-- idx_orders_regular:    5MB
-- idx_orders_covering:   12MB (includes total, status, created_at)

-- Is it worth it? Compare query performance
EXPLAIN (ANALYZE, BUFFERS)
SELECT total, status, created_at FROM orders WHERE customer_id = 42;
-- Regular: Index Scan, 150 heap fetches, 2.3ms
-- Covering: Index Only Scan, 0 heap fetches, 0.8ms
-- 3x faster for 2.4x more storage — worth it for hot queries
```

---

### Q9. How do you create a partial covering index?

**Interview Answer**

A partial covering index combines a WHERE clause with the INCLUDE clause — it indexes only a subset of rows and includes payload columns for those rows. This is extremely efficient when you frequently query for a specific condition that matches a small fraction of the table. The index is smaller (fewer rows), faster to scan, and cheaper to maintain. The query planner uses the partial index only when the query's WHERE clause matches or is a subset of the index's WHERE clause. This is a powerful pattern for multi-tenant systems, status-based queries, and soft-delete patterns.

```sql
-- Partial covering index: only active orders
CREATE INDEX idx_active_orders_covering ON orders (customer_id, created_at DESC)
    INCLUDE (total, status)
    WHERE status = 'active';

-- Query that benefits (matches partial index condition)
EXPLAIN ANALYZE
SELECT total, status, created_at
FROM orders
WHERE customer_id = 42 AND status = 'active'
ORDER BY created_at DESC
LIMIT 10;
-- Index Only Scan using idx_active_orders_covering
-- Only 50K rows indexed instead of 1M!

-- Query that does NOT use partial index (different condition)
EXPLAIN ANALYZE
SELECT total, status FROM orders WHERE customer_id = 42 AND status = 'shipped';
-- Falls back to regular Index Scan or Seq Scan

-- Soft-delete pattern
CREATE INDEX idx_active_users ON users (email) INCLUDE (name, created_at)
    WHERE deleted_at IS NULL;

SELECT name, created_at FROM users WHERE email = 'alice@example.com' AND deleted_at IS NULL;
-- Index Only Scan using idx_active_users
```

---

### Q10. How do covering indexes interact with partitioned tables?

**Interview Answer**

Covering indexes work with partitioned tables, but the INCLUDE columns must be the same across all partitions. Each partition has its own physical index, so the covering index is created per-partition (automatically when you create it on the parent table). The index-only scan benefit applies at the partition level — if a query scans only one partition, the covering index on that partition eliminates heap fetches. Partition pruning combined with covering indexes provides excellent performance for time-series and multi-tenant data. Use partial covering indexes on partitions to further optimize for specific access patterns.

```sql
-- Partitioned table with covering index
CREATE TABLE orders (
    id BIGSERIAL,
    customer_id INT,
    total DECIMAL,
    status VARCHAR(20),
    created_at TIMESTAMPTZ
) PARTITION BY RANGE (created_at);

CREATE TABLE orders_2025_07 PARTITION OF orders
    FOR VALUES FROM ('2025-07-01') TO ('2025-08-01');

-- Covering index created on all partitions automatically
CREATE INDEX idx_orders_covering ON orders (customer_id, created_at DESC)
    INCLUDE (total, status);

-- Query with partition pruning + index-only scan
EXPLAIN ANALYZE
SELECT total, status FROM orders
WHERE customer_id = 42
  AND created_at >= '2025-07-01' AND created_at < '2025-08-01';
-- Index Only Scan on orders_2025_07 only (pruned other partitions)
-- Heap Fetches: 0 (all data from index)
```
