# pessimistic locking

## Interview Question

Explain pessimistic locking.

## Interview Answer

"Pessimistic locking locks rows before modification using FOR UPDATE to prevent concurrent changes."

---

## Follow-up Questions & Answers

### Q1. How do you use `SELECT ... FOR UPDATE` with sqlx in Rust?

**Interview Answer**

Use `sqlx::query!("SELECT * FROM accounts WHERE id = $1 FOR UPDATE", id)` inside a transaction. The lock is held until the transaction commits or rolls back. In Axum, wrap this in `pool.begin().await?` and use `tx.commit().await?` after the update to release the lock.

---

### Q2. What are the risks of pessimistic locking?

**Interview Answer**

Long-held locks block other transactions, reducing throughput and potentially causing deadlocks. If a handler crashes while holding a lock, the transaction rolls back but may have delayed other operations. Always keep locked transactions short and implement timeout mechanisms to prevent indefinite blocking.

---

### Q3. How do you prevent deadlocks with pessimistic locking?

**Interview Answer**

Always lock rows in the same order across all transactions to avoid circular wait conditions. Use `SELECT ... FOR UPDATE NOWAIT` to immediately fail if a lock can't be acquired instead of waiting. Implement retry logic with backoff in Axum handlers to handle deadlock errors gracefully.

---

### Q4. What is the difference between `FOR UPDATE` and `FOR SHARE`?

**Interview Answer**

`FOR UPDATE` acquires an exclusive lock that blocks other transactions from modifying or locking the same rows. `FOR SHARE` acquires a shared lock that allows concurrent reads but blocks writes. Use `FOR SHARE` when you need read consistency without blocking other readers, and `FOR UPDATE` when you plan to modify the rows.

---

### Q5. When should you choose pessimistic over optimistic locking in an Axum backend?

**Interview Answer**

Use pessimistic locking when write contention is high, like in inventory or payment systems where conflicts are frequent. Optimistic locking is better for read-heavy scenarios with rare conflicts. Pessimistic locking guarantees no retries but adds latency from lock acquisition; measure both approaches under your actual workload.

---

### Q6. How does `FOR UPDATE SKIP LOCKED` work and when is it useful?

**Interview Answer**

`SKIP LOCKED` skips rows already locked by other transactions instead of waiting, making it ideal for job queue patterns. In Axum, implement a task queue by selecting pending jobs with `FOR UPDATE SKIP LOCKED LIMIT 1` and processing them concurrently. This avoids contention between multiple worker instances competing for the same tasks.

---

### Q7. Can pessimistic locking cause connection pool exhaustion?

**Interview Answer**

Yes. Long-running transactions holding locks keep connections occupied, reducing available connections in the pool. In Axum with `PgPool`, this can starve other handlers waiting for connections. Monitor pool metrics and set aggressive transaction timeouts to prevent this scenario.

---

### Q8. How do you test pessimistic locking behavior in integration tests?

**Interview Answer**

Spawn concurrent Tokio tasks that execute `SELECT ... FOR UPDATE` on the same row within separate transactions. Verify that one transaction acquires the lock while the other blocks or fails with `NOWAIT`. Use `sqlx::test` with a real PostgreSQL database to ensure accurate lock behavior testing.
