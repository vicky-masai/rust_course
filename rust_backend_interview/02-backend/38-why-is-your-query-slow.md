# Why is your query slow?

## Interview Question

Why is your query slow?

## Interview Answer

Check:
- Missing indexes
- Full table scans
- Large joins
- `EXPLAIN ANALYZE`
- Lock contention
- Inefficient SQL

---

## Follow-up Questions & Answers

### Q1. How do you read PostgreSQL's `EXPLAIN ANALYZE` output?

**Interview Answer**

Focus on actual vs estimated rows, total cost, and execution time. Look for sequential scans on large tables, nested loops with high row counts, and sort operations exceeding memory. Compare estimated and actual rows to identify stale statistics that need `ANALYZE` updates.

---

### Q2. What are the most common indexing mistakes that slow down queries?

**Interview Answer**

Over-indexing wastes write performance and maintenance overhead. Missing indexes on WHERE and JOIN columns cause sequential scans. Wrong index order in composite indexes fails to support query patterns. Use `pg_stat_user_indexes` to find unused indexes and `pg_stat_statements` for slow queries.

---

### Q3. How does N+1 query pattern affect performance in Axum?

**Interview Answer**

N+1 queries cause O(n) database round trips instead of O(1), adding latency per request. Fix by using JOINs or subqueries in sqlx to fetch related data in one query. In Axum handlers, batch related queries using `sqlx::query!` with IN clauses or CTEs.

---

### Q4. How do you identify lock contention in PostgreSQL?

**Interview Answer**

Query `pg_stat_activity` for waiting queries and `pg_locks` for lock modes. Use `pg_locks` joined with `pg_stat_activity` to find blocking and blocked queries. In Axum, log query duration and monitor for timeout errors that indicate lock contention under concurrent load.

---

### Q5. What is the impact of vacuuming on query performance?

**Interview Answer**

VACUUM reclaims dead tuple space and updates statistics for the query planner. Without regular VACUUM, tables bloat and queries slow down. Monitor `pg_stat_user_tables` for dead tuples and autovacuum activity. Tune autovacuum settings for high-write tables to prevent performance degradation.

---

### Q6. How do you optimize slow JOIN operations?

**Interview Answer**

Ensure JOIN columns have indexes and statistics are up to date. Use EXPLAIN ANALYZE to check for nested loop vs hash joins and their costs. Rewrite complex JOINs using CTEs or temporary tables, and filter early with WHERE clauses to reduce the rows going into JOINs.

---

### Q7. How do you use connection pool metrics to diagnose slow queries?

**Interview Answer**

Monitor pool wait times, active connections, and query duration via sqlx metrics. Long wait times indicate connection exhaustion from slow queries. Use Prometheus to track p95 query duration and correlate with pool utilization. Alert on queries exceeding acceptable latency thresholds.

---

### Q8. What query rewriting techniques have the biggest performance impact?

**Interview Answer**

Replace subqueries with JOINs, use EXISTS instead of IN for large datasets, and filter with WHERE before aggregation. Use materialized views for complex queries that don't need real-time data. In sqlx, write queries with CTEs for readability while maintaining performance.

---

# DSA (Frequently Asked)
