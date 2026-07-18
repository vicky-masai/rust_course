# How do you reduce API latency from 800ms to 50ms?

## Interview Question

How do you reduce API latency from 800ms to 50ms?

## Interview Answer

- Profile first
- Optimize SQL queries
- Add indexes
- Redis cache
- Batch database calls
- Parallelize independent operations
- Compress responses
- Avoid N+1 queries

---

## Follow-up Questions & Answers

### Q1. How do you identify the main source of latency in an Axum API?

**Interview Answer**

Use `tracing` with span timings to measure each step of the request lifecycle, including middleware, database queries, and serialization. Profile with `cargo flamegraph` for CPU-bound bottlenecks and `tokio-console` for async scheduling issues. Start by measuring the slowest endpoints with `EXPLAIN ANALYZE` on their SQL queries.

---

### Q2. How does Redis caching reduce API latency in Rust backends?

**Interview Answer**

Redis stores frequently accessed data in memory, eliminating database round trips for repeated queries. Use `redis::aio::ConnectionManager` in Axum to maintain an async connection pool. Implement cache-aside pattern with `GET`/`SET` and TTL to balance freshness with latency reduction.

---

### Q3. What is the N+1 query problem and how do you fix it in sqlx?

**Interview Answer**

N+1 occurs when you fetch N records and then make N individual queries for related data. Fix it by using JOINs or subqueries to fetch everything in one query. In sqlx, use `sqlx::query_as!` with complex SELECT statements to fetch related data in a single round trip.

---

### Q4. How do you parallelize independent operations in Axum?

**Interview Answer**

Use `tokio::join!` or `tokio::try_join!` to run independent async operations concurrently. For example, fetch user data and permissions simultaneously instead of sequentially. This reduces total latency to the maximum of individual operations rather than their sum.

---

### Q5. What compression strategy should you use for API responses?

**Interview Answer**

Enable gzip or brotli compression using `tower-http::compression::CompressionLayer` in the Axum middleware stack. Set `Content-Encoding` header and configure compression levels based on response size. Compress JSON responses over 1KB to balance CPU usage with bandwidth savings.

---

### Q6. How does query batching reduce latency compared to individual queries?

**Interview Answer**

Batching combines multiple SQL queries into a single round trip, reducing network latency overhead. Use sqlx's `query!` with UNION ALL or execute multiple queries within a single transaction. For example, batch 10 individual SELECTs into one query with IN clauses.

---

### Q7. What index strategies have the biggest impact on query performance?

**Interview Answer**

Add indexes on columns used in WHERE clauses, JOIN conditions, and ORDER BY. Use composite indexes for multi-column filters and partial indexes for selective queries. Monitor `pg_stat_user_indexes` to identify unused indexes and `pg_stat_statements` to find slow queries needing indexes.

---

### Q8. How do you measure latency improvements after optimizations?

**Interview Answer**

Use `hey` or `wrk` for load testing and track p50, p95, and p99 latency percentiles. Compare before and after metrics with identical request patterns and data volumes. Monitor in production with Prometheus histograms and alert on latency regression using Grafana dashboards.
