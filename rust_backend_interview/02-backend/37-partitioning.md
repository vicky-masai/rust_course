# partitioning

## Interview Question

Explain partitioning.

## Interview Answer

Split one large table into smaller partitions to improve performance and maintenance.

---

## Follow-up Questions & Answers

### Q1. What is the difference between horizontal and vertical partitioning?

**Interview Answer**

Horizontal partitioning splits rows across partitions, like splitting a large events table by date range. Vertical partitioning splits columns into separate tables, like separating user profile data from frequently accessed fields. Horizontal partitioning is more common in PostgreSQL for improving query performance on large tables.

---

### Q2. How does PostgreSQL implement table partitioning?

**Interview Answer**

PostgreSQL supports range, list, and hash partitioning using `PARTITION BY` in `CREATE TABLE`. Range partitioning divides data by date or numeric ranges, list by specific values, and hash by modulo of a column. Create partitions with `CREATE TABLE ... PARTITION OF` and attach indexes to each partition.

---

### Q3. How does partitioning improve query performance in PostgreSQL?

**Interview Answer**

Partitioning enables partition pruning, which skips scanning irrelevant partitions based on WHERE clauses. A query filtering by date range only scans matching partitions instead of the entire table. This reduces I/O and improves query performance for large tables with millions of rows.

---

### Q4. How do you choose the right partition key?

**Interview Answer**

Choose a key that evenly distributes data and aligns with common query patterns, like timestamps for time-series data. Avoid keys with skewed distribution that create hot partitions. Analyze query patterns with `pg_stat_statements` to ensure the partition key supports your most frequent WHERE clauses.

---

### Q5. How do you manage partition lifecycle in production?

**Interview Answer**

Create future partitions proactively using cron jobs or pg_partman extension. Drop old partitions with `DROP TABLE` instead of DELETE for instant cleanup. In Axum, implement partition management as background tasks using `tokio::time::interval` to create new partitions before they're needed.

---

### Q6. What are the limitations of PostgreSQL partitioning?

**Interview Answer**

Partitioned tables have constraints on unique indexes, foreign keys, and triggers. Cross-partition queries can be slower without proper partition pruning. Partition maintenance adds operational complexity. Consider partitioning only when tables exceed millions of rows and query performance degrades significantly.

---

### Q7. How do you query across partitions efficiently?

**Interview Answer**

Ensure queries include the partition key in WHERE clauses to trigger partition pruning. Use `EXPLAIN ANALYZE` to verify that only relevant partitions are scanned. Create indexes on each partition rather than a global index, as PostgreSQL supports partition-level indexes.

---

### Q8. How does partitioning interact with replication in PostgreSQL?

**Interview Answer**

Partitions replicate to standbys just like regular tables, maintaining consistency across replicas. However, partition maintenance operations like CREATE/DROP generate WAL traffic that affects replication lag. Schedule partition maintenance during low-traffic periods to minimize impact on replica lag.
