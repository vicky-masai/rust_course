# EXPLAIN ANALYZE in PostgreSQL

## Interview Question

How do you use EXPLAIN ANALYZE to optimize PostgreSQL queries?

## Interview Answer

EXPLAIN ANALYZE executes a query and shows its actual execution plan with real timing and row counts. It reveals which data access methods PostgreSQL chose (sequential scan, index scan, bitmap scan), how rows flow through the plan (joins, sorts, aggregations), and where bottlenecks occur. The output is a tree structure read bottom-up — leaf nodes are table accesses, parent nodes are operations on those results. Key metrics: `actual time` (first value is time-to-first-row, second is total), `rows` vs `rows removed by Filter`, and `sort method` (external merge = disk spill). Use EXPLAIN ANALYZE as the first step in query optimization — it shows you exactly where time is spent.

---

## Follow-up Questions & Answers

### Q1. What is the difference between EXPLAIN and EXPLAIN ANALYZE?

**Interview Answer**

EXPLAIN shows the query plan that PostgreSQL would use without actually executing the query — it estimates row counts and costs based on table statistics. EXPLAIN ANALYZE actually executes the query and shows real timing and row counts, allowing you to compare estimated vs actual values. EXPLAIN is useful for quick plan inspection without side effects, while EXPLAIN ANALYZE is essential for real performance analysis. A key difference: EXPLAIN ANALYZE modifies the database (INSERT/UPDATE/DELETE actually run), so wrap destructive queries in a transaction and ROLLBACK. The `actual time` values in ANALYZE output represent milliseconds and are measured with high-precision timers.

```sql
-- EXPLAIN: estimated plan only (no execution)
EXPLAIN SELECT * FROM users WHERE email = 'alice@example.com';
-- Output: Index Scan using idx_users_email on users
--         Filter: (email = 'alice@example.com')
--         Rows Removed by Filter: 0
--         Planning Time: 0.1 ms
--         Execution Time: 0.05 ms

-- EXPLAIN ANALYZE: actual execution with real timing
EXPLAIN ANALYZE SELECT * FROM users WHERE email = 'alice@example.com';
-- Output: Index Scan using idx_users_email on users
--         Rows Removed by Filter: 0
--         Planning Time: 0.1 ms
--         Execution Time: 0.05 ms

-- EXPLAIN ANALYZE for destructive queries (use ROLLBACK)
BEGIN;
EXPLAIN ANALYZE DELETE FROM temp_data WHERE created_at < '2025-01-01';
ROLLBACK;

-- Add BUFFERS for I/O analysis
EXPLAIN (ANALYZE, BUFFERS, FORMAT TEXT) SELECT * FROM orders WHERE total > 100;
-- Shows: shared hit (from cache), shared read (from disk), temp written (to disk)
```

---

### Q2. What does "Seq Scan" vs "Index Scan" mean in the EXPLAIN output?

**Interview Answer**

A Sequential Scan (Seq Scan) reads every row in the table — it's the default when no index can help or when the optimizer estimates that reading the entire table is cheaper than using an index. It's fast for small tables or when returning most rows (>10-20% of the table). An Index Scan traverses a B-tree index to find matching row pointers, then fetches each row from the heap — efficient for selective queries returning few rows. A Bitmap Scan is a hybrid: it builds a bitmap of matching pages using the index, then fetches those pages in physical order — good for medium selectivity. An Index Only Scan reads entirely from the index without touching the table — the fastest option.

```sql
-- Seq Scan: reading entire table
EXPLAIN ANALYZE SELECT * FROM orders WHERE total > 0;
-- Seq Scan on orders  (cost=0.00..1234.00 rows=99999 width=64)
--   Filter: (total > 0)
--   Rows Removed by Filter: 1
--   Planning Time: 0.1 ms
--   Execution Time: 45.2 ms

-- Index Scan: using B-tree
CREATE INDEX idx_orders_total ON orders (total);
EXPLAIN ANALYZE SELECT * FROM orders WHERE total > 999.99;
-- Index Scan using idx_orders_total on orders
--   Index Cond: (total > 999.99)
--   Rows Removed by Index Recheck: 0
--   Planning Time: 0.2 ms
--   Execution Time: 0.08 ms

-- Index Only Scan: all data in index
CREATE INDEX idx_orders_total_covering ON orders (total) INCLUDE (id, customer_id);
EXPLAIN ANALYZE SELECT id, customer_id, total FROM orders WHERE total > 999.99;
-- Index Only Scan using idx_orders_total_covering on orders
--   Planning Time: 0.1 ms
--   Execution Time: 0.03 ms
```

---

### Q3. What does "actual time" mean in EXPLAIN ANALYZE output?

**Interview Answer**

In EXPLAIN ANALYZE output, `actual time` shows two values in milliseconds: the first is the time to return the first row from that node, and the second is the total time to return all rows. For example, `actual time=0.05..12.34` means the first row appeared in 0.05ms and all rows were returned in 12.34ms. The total query execution time appears at the bottom as `Execution Time`. Note that timing includes only execution time, not planning time (shown separately as `Planning Time`). For parallel queries, timing is shown for each worker and the leader. Understanding these values helps identify whether a node is slow due to initial overhead or bulk processing.

```sql
EXPLAIN (ANALYZE, FORMAT TEXT)
SELECT u.name, COUNT(o.id)
FROM users u
JOIN orders o ON u.id = o.user_id
GROUP BY u.name;

-- Output interpretation:
-- HashAggregate  actual time=45.12..45.15 rows=1000
--   ^agg complete in 45ms, 1000 groups
-- Hash Join  actual time=12.34..44.89 rows=50000
--   ^first row in 12ms, all rows in 44ms
-- Seq Scan on users  actual time=0.01..2.34 rows=1000
--   ^users table scan takes 2ms
-- Hash  actual time=8.90..8.91 rows=50000
--   ^building hash table takes 9ms
-- Seq Scan on orders  actual time=0.01..6.78 rows=50000
--   ^orders table scan takes 7ms
-- Planning Time: 0.5 ms
-- Execution Time: 45.5 ms
```

---

### Q4. How do you identify a query that needs an index from EXPLAIN output?

**Interview Answer**

Look for these red flags in EXPLAIN ANALYZE output: (1) `Seq Scan` on a large table with a low `rows Removed by Filter` ratio (returns few rows from many scanned — needs index on filter column); (2) `Sort Method: external merge` (sorting spills to disk — needs index for ORDER BY or increase work_mem); (3) `Nested Loop` with high row estimates (should be Hash Join — statistics may be stale, run ANALYZE); (4) High `actual time` on a single node relative to total execution time; (5) `rows Removed by Filter` much larger than returned rows (low selectivity — index would help). The general rule: if a Seq Scan on a large table returns less than 10-15% of rows, an index on the filter column would likely help.

```sql
-- Flag 1: Seq Scan with low selectivity (needs index)
EXPLAIN ANALYZE SELECT * FROM orders WHERE customer_id = 42;
-- Seq Scan on orders  actual time=0.01..234.56 rows=3
--   Filter: (customer_id = 42)
--   Rows Removed by Filter: 999997  -- Scans 1M rows for 3!
-- FIX: CREATE INDEX idx_orders_customer ON orders (customer_id);

-- Flag 2: Sort spilling to disk
EXPLAIN ANALYZE SELECT * FROM orders ORDER BY created_at DESC LIMIT 10;
-- Top-N Sort  actual time=100.23..100.24 rows=10
--   Sort Method: top-N heapsort  Memory: 256kB
-- Or worse: Sort Method: external merge  Disk: 1024kB
-- FIX: CREATE INDEX idx_orders_created ON orders (created_at DESC);

-- Flag 3: Stale statistics
EXPLAIN ANALYZE SELECT * FROM users WHERE active = true;
-- Rows Estimated: 50000  Actual: 100  -- Huge mismatch!
-- FIX: ANALYZE users;
```

---

### Q5. What does "Buffers" tell you in EXPLAIN ANALYZE output?

**Interview Answer**

The BUFFERS option shows I/O statistics for each node in the plan. `shared hit` means pages read from PostgreSQL's shared buffer cache (fast, in-memory). `shared read` means pages read from disk (slow, I/O-bound). `shared written` means dirty pages written to disk. `temp read/written` means pages written to and read from temporary files (sort/hash operations that exceeded work_mem). High `shared read` counts indicate the table doesn't fit in memory — consider increasing `shared_buffers` or adding indexes. High `temp written` indicates sorts/hashes spilling to disk — increase `work_mem` or add indexes to avoid sorting.

```sql
EXPLAIN (ANALYZE, BUFFERS, FORMAT TEXT)
SELECT * FROM orders WHERE customer_id = 42 ORDER BY created_at DESC;

-- Output:
-- Index Scan using idx_orders_customer on orders
--   Index Cond: (customer_id = 42)
--   Buffers: shared hit=125 read=8    -- 125 from cache, 8 from disk
-- Planning Time: 0.2 ms
-- Execution Time: 2.3 ms

-- Bad: lots of disk reads
-- Buffers: shared hit=50 read=12000   -- 12000 pages from disk!
-- FIX: increase shared_buffers, or ensure table fits in cache

-- Bad: sort spilling to disk
-- Sort Method: external merge  Disk: 2048kB
-- Buffers: shared hit=500 temp_written=256
-- FIX: increase work_mem or add index on ORDER BY columns
```

---

### Q6. How do you use EXPLAIN ANALYZE with SQLx in Rust?

**Interview Answer**

You can run EXPLAIN ANALYZE through SQLx to analyze queries in your Rust application. Use `sqlx::query()` with the EXPLAIN ANALYZE prefix and fetch the raw output. This is useful for production debugging — you can analyze slow queries identified by `log_min_duration_statement` without needing direct database access. For automated analysis, parse the output to detect sequential scans, sort spills, or high row estimates. SQLx's `query_scalar!` or `query!` macros can be used with EXPLAIN ANALYZE to get structured output. Consider adding an endpoint or admin tool that runs EXPLAIN ANALYZE on demand for specific queries.

```rust
use sqlx::PgPool;

async fn analyze_query(pool: &PgPool, query: &str) -> Result<String> {
    let explain_query = format!("EXPLAIN (ANALYZE, BUFFERS, FORMAT TEXT) {}", query);

    let result: (String,) = sqlx::query_as(&explain_query)
        .fetch_one(pool)
        .await?;

    Ok(result.0)
}

// Usage: analyze a slow query
async fn debug_slow_query(pool: &PgPool) -> Result<()> {
    let query = "SELECT * FROM orders WHERE customer_id = $1 ORDER BY created_at DESC";
    let plan = analyze_query(pool, query).await?;

    // Check for red flags
    if plan.contains("Seq Scan") && plan.contains("Rows Removed by Filter: ") {
        tracing::warn!("Query uses Seq Scan with filter — consider adding an index");
    }
    if plan.contains("external merge") {
        tracing::warn!("Sort spilling to disk — increase work_mem or add index");
    }
    if plan.contains("Nested Loop") {
        tracing::warn!("Nested Loop detected — may need statistics update (ANALYZE)");
    }

    tracing::info!(plan, "EXPLAIN ANALYZE output");
    Ok(())
}

// Automated slow query analyzer
async fn analyze_slow_queries(pool: &PgPool) -> Result<()> {
    let slow_queries = sqlx::query!(
        "SELECT query, mean_exec_time, calls
         FROM pg_stat_statements
         WHERE mean_exec_time > 1000
         ORDER BY mean_exec_time DESC
         LIMIT 10"
    )
    .fetch_all(pool)
    .await?;

    for q in &slow_queries {
        let plan = analyze_query(pool, &q.query).await?;
        tracing::info!(query = %q.query, mean_ms = q.mean_exec_time, plan, "Slow query analysis");
    }
    Ok(())
}
```

---

### Q7. What is a Bitmap Heap Scan and when does PostgreSQL choose it?

**Interview Answer**

A Bitmap Heap Scan is a two-phase scan strategy PostgreSQL uses when an index identifies a moderate number of matching rows. Phase 1 (Bitmap Index Scan): PostgreSQL traverses the index and builds a bitmap in memory of all matching heap page numbers. Phase 2 (Bitmap Heap Scan): PostgreSQL fetches the actual rows from the heap pages in physical order, which is more efficient than random I/O. PostgreSQL chooses a Bitmap Scan when the selectivity is between Seq Scan and Index Scan territory — typically when 1-15% of rows match. The bitmap is stored in work_mem; if it exceeds that, it spills to disk ("Lossy" bitmap, requiring rechecking).

```sql
-- Bitmap Scan example
CREATE INDEX idx_orders_total ON orders (total);
EXPLAIN ANALYZE SELECT * FROM orders WHERE total > 50;
-- Bitmap Heap Scan on orders  actual time=10.23..45.67 rows=15000
--   Recheck Cond: (total > 50)
--   Heap Blocks: exact=1500
--   Buffers: shared hit=1500
--   ->  Bitmap Index Scan on idx_orders_total
--         Index Cond: (total > 50)
--         Rows Removed by Index Recheck: 0

-- When PostgreSQL chooses Bitmap vs Index vs Seq Scan
-- Seq Scan: < 5% of rows (or very small table)
-- Bitmap Scan: 5-15% of rows
-- Index Scan: > 15% of rows (or single row)
-- These thresholds depend on random_page_cost setting
```

---

### Q8. What does "Planning Time" vs "Execution Time" mean?

**Interview Answer**

Planning Time is the time PostgreSQL spends parsing SQL, checking syntax, running the query optimizer to find the best plan, and generating the execution plan. Execution Time is the actual time spent executing the plan — reading data, filtering, joining, sorting, and returning results. Planning Time is usually small (0.1-5ms) for simple queries but can be significant for complex queries with many joins or subqueries. If Planning Time is high relative to Execution Time, the optimizer is struggling — consider simplifying the query, using CTEs (which may be optimized), or updating statistics. For prepared statements, Planning Time is paid once and reused across executions.

```sql
-- Simple query: planning is negligible
EXPLAIN ANALYZE SELECT * FROM users WHERE id = 1;
-- Planning Time: 0.1 ms
-- Execution Time: 0.05 ms
-- Total: 0.15 ms

-- Complex query: planning may be significant
EXPLAIN ANALYZE
SELECT u.name, o.total, p.name
FROM users u
JOIN orders o ON u.id = o.user_id
JOIN line_items li ON o.id = li.order_id
JOIN products p ON li.product_id = p.id
WHERE u.created_at > '2025-01-01'
  AND o.total > 100
ORDER BY o.total DESC
LIMIT 10;
-- Planning Time: 2.3 ms  (optimizer considering many join orders)
-- Execution Time: 45.6 ms
-- Total: 47.9 ms

-- Prepared statement: planning time is amortized
PREPARE find_users AS SELECT * FROM users WHERE email = $1;
EXECUTE find_users('alice@example.com');  -- Planning: 0.5ms, Execution: 0.1ms
EXECUTE find_users('bob@example.com');    -- Planning: 0.0ms (cached), Execution: 0.1ms
```

---

### Q9. How do you read a nested loop, hash join, and merge join from EXPLAIN output?

**Interview Answer**

Nested Loop: for each row from the outer input, scan the inner input for matches. Fast when the outer input is small and the inner input is indexed. Shows as "Nested Loop" with inner/outer child nodes. Hash Build/Hash Probe: build a hash table from the smaller input, then probe it with each row from the larger input. Best for equi-joins of medium-to-large tables. Shows as "Hash" (build phase) then "Hash Join". Merge Join: sort both inputs and merge them. Best for pre-sorted data or when both inputs are indexed. Shows as "Merge Join" with sort nodes if needed. The optimizer chooses based on table sizes, available indexes, and statistics.

```sql
-- Nested Loop (small outer, indexed inner)
EXPLAIN ANALYZE
SELECT * FROM users u
JOIN orders o ON u.id = o.user_id
WHERE u.id = 42;
-- Nested Loop  actual time=0.05..0.08 rows=5
--   ->  Index Scan using users_pkey on users u
--   ->  Index Scan using idx_orders_user on orders o
-- Best when: outer = 1 row, inner = few rows, inner is indexed

-- Hash Join (medium-to-large equi-join)
EXPLAIN ANALYZE
SELECT * FROM users u
JOIN orders o ON u.id = o.user_id;
-- Hash Join  actual time=10.23..45.67 rows=50000
--   ->  Seq Scan on users u
--   ->  Hash
--         ->  Seq Scan on orders o
-- Best when: both tables are medium/large, equi-join

-- Merge Join (pre-sorted inputs)
EXPLAIN ANALYZE
SELECT * FROM users u
JOIN orders o ON u.id = o.user_id
ORDER BY u.id;
-- Merge Join  actual time=5.12..30.45 rows=50000
--   ->  Index Scan using users_pkey on users u
--   ->  Index Scan using idx_orders_user on orders o
-- Best when: both inputs are sorted on join key (index or ORDER BY)
```

---

### Q10. What are common EXPLAIN ANALYZE anti-patterns and how do you fix them?

**Interview Answer**

Common anti-patterns: (1) `Seq Scan` on large table with few matching rows — add index on filter column; (2) `Sort Method: external merge Disk:` — increase work_mem or add index on ORDER BY columns; (3) `Nested Loop` with high row estimates — run ANALYZE to update statistics, or force Hash Join with `SET enable_nestloop = off`; (4) `Rows Removed by Filter` much larger than returned rows — needs index; (5) High `Planning Time` — simplify query or use prepared statements; (6) `SubPlan` in output — rewrite as JOIN or CTE; (7) `Hash Batch` — hash table exceeds work_mem, increase it or reduce data volume.

```sql
-- Anti-pattern 1: Seq Scan with filter
EXPLAIN ANALYZE SELECT * FROM logs WHERE message LIKE '%error%';
-- Seq Scan, 1M rows scanned, 100 returned
-- FIX: GIN index for full-text search, or trigram index

-- Anti-pattern 2: Sort spilling to disk
EXPLAIN ANALYZE SELECT * FROM events ORDER BY created_at LIMIT 100;
-- Sort Method: external merge  Disk: 4096kB
-- FIX: CREATE INDEX idx_events_created ON events (created_at);

-- Anti-pattern 3: Nested Loop with bad estimates
EXPLAIN ANALYZE SELECT * FROM a JOIN b ON a.id = b.a_id;
-- Nested Loop  actual time=0.01..5000.00 rows=1000000
-- Estimated rows: 100, Actual: 1000000 (10000x off!)
-- FIX: ANALYZE a; ANALYZE b; -- Update statistics

-- Anti-pattern 4: SubPlan (correlated subquery)
EXPLAIN ANALYZE SELECT * FROM users u WHERE EXISTS (SELECT 1 FROM orders o WHERE o.user_id = u.id);
-- SubPlan  cost=0.00..500000
-- FIX: Rewrite as: SELECT DISTINCT u.* FROM users u INNER JOIN orders o ON u.id = o.user_id;

-- Diagnostic query to find missing indexes
SELECT schemaname, relname, seq_scan, idx_scan,
       seq_tup_read, idx_tup_fetch
FROM pg_stat_user_tables
WHERE seq_scan > idx_scan AND n_live_tup > 10000
ORDER BY seq_tup_read DESC;
```
