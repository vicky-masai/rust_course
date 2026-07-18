# replication

## Interview Question

Explain replication.

## Interview Answer

Primary handles writes; replicas serve reads and improve availability.

---

## Follow-up Questions & Answers

### Q1. What is the difference between synchronous and asynchronous replication?

**Interview Answer**

Synchronous replication waits for replicas to acknowledge writes before confirming to the client, ensuring zero data loss but adding latency. Asynchronous replication propagates changes without waiting, providing better performance but risking data loss on primary failure. Choose based on your consistency requirements and tolerance for data loss.

---

### Q2. How does PostgreSQL streaming replication work?

**Interview Answer**

The primary streams WAL (Write-Ahead Log) segments to standby servers which replay them to stay synchronized. Configure with `primary_conninfo` in the standby's `postgresql.conf`. Use `pg_basebackup` to create initial standby copies and `recovery.conf` for standby setup.

---

### Q3. How do you read from replicas in an Axum application?

**Interview Answer**

Create separate connection pools for primary and replica databases using `sqlx::PgPool`. Route read queries to replica pools and writes to the primary pool. Implement this in Axum using middleware or repository pattern that selects the appropriate pool based on query type.

---

### Q4. What is replication lag and how do you monitor it?

**Interview Answer**

Replication lag is the delay between primary write and replica availability. Monitor using `pg_stat_replication` on the primary and `pg_last_wal_receive_lsn()` on replicas. Set alerts when lag exceeds acceptable thresholds, typically a few seconds for most applications.

---

### Q5. How does replication affect transaction isolation in Axum?

**Interview Answer**

Reads from replicas may see stale data due to replication lag, providing eventual consistency. Use primary reads for operations requiring strong consistency. In Axum, implement a consistency hint in request context to route critical reads to the primary while using replicas for dashboards and analytics.

---

### Q6. What is cascading replication and when should you use it?

**Interview Answer**

Cascading replication chains standbys from other standbys, reducing primary load for distributing data. Use it when you have many read replicas and want to minimize primary bandwidth. The tradeoff is increased replication lag for replicas further from the primary.

---

### Q7. How do you handle failover when the primary goes down?

**Interview Answer**

Use tools like `pgbouncer` or Patroni for automatic failover by promoting a standby to primary. Update connection strings in your Axum application to point to the new primary. Implement health checks and connection retry logic to handle failover gracefully without downtime.

---

### Q8. How do you set up read-write splitting in sqlx with Axum?

**Interview Answer**

Create two `PgPool` instances in `main.rs` and inject both into Axum handlers via `Extension`. Use a repository layer that routes SELECT queries to the replica pool and INSERT/UPDATE/DELETE to the primary. Handle connection errors with retry logic and fallback to primary if replicas are unavailable.
