# optimistic locking

## Interview Question

Explain optimistic locking.

## Interview Answer

"Optimistic locking detects conflicts using a version column. Updates succeed only if the version matches."

---

## Follow-up Questions & Answers

### Q1. How is optimistic locking implemented in sqlx?

**Interview Answer**

Add a `version` column to the table and include it in the UPDATE's WHERE clause: `UPDATE users SET name = $1, version = version + 1 WHERE id = $2 AND version = $3`. Check the row count returned; if zero, another transaction modified the row. This pattern works well in Axum handlers where conflicts are rare and you want to avoid holding locks.

---

### Q2. What happens when an optimistic lock conflict occurs?

**Interview Answer**

The UPDATE affects zero rows, indicating a conflict. The application should retry the entire read-modify-write cycle or return a 409 Conflict error to the client. In Axum, implement retry logic with exponential backoff using `tokio::time::sleep` or let the client decide based on the HTTP status code.

---

### Q3. Is optimistic locking suitable for high-contention scenarios?

**Interview Answer**

No. Under high contention, frequent conflicts lead to excessive retries and wasted CPU cycles. In such cases, pessimistic locking with `SELECT ... FOR UPDATE` is more efficient because it blocks conflicting transactions upfront. Optimistic locking is best for read-heavy workloads where writes rarely conflict.

---

### Q4. How do you handle version column type in PostgreSQL with sqlx?

**Interview Answer**

Use an integer type like `INTEGER` or `BIGINT` with a default of 1, or a timestamp-based version using `TIMESTAMPTZ`. PostgreSQL's `xmin` system column can also serve as a version without adding a custom column. With sqlx, map the version field to `i32` or `i64` in your Rust struct.

---

### Q5. Can optimistic locking prevent lost updates in Axum handlers?

**Interview Answer**

Yes. By checking the version in the WHERE clause, the UPDATE fails if the row was modified between read and write. This prevents lost updates without database-level locks. Combine this with proper error handling in the Axum handler to return appropriate status codes when conflicts occur.

---

### Q6. How does optimistic locking interact with database transactions?

**Interview Answer**

Within a transaction, optimistic locking still works because the version check happens at UPDATE time. However, the conflict is only detected when the transaction commits or the UPDATE executes. If you need immediate conflict detection, combine optimistic locking with explicit locking or use serialization isolation level.

---

### Q7. What are the performance implications of optimistic vs pessimistic locking?

**Interview Answer**

Optimistic locking has lower overhead because it doesn't hold locks, allowing higher throughput under low contention. Pessimistic locking introduces lock wait times and potential deadlocks but avoids retry storms. Measure both approaches under your workload using benchmarks and choose based on actual conflict rates.

---

### Q8. How do you test optimistic locking behavior in a Rust application?

**Interview Answer**

Write integration tests that simulate concurrent updates by spawning multiple Tokio tasks that read the same row, then attempt to update with different versions. Assert that exactly one succeeds and others receive conflict errors. Use `sqlx::test` to set up test databases with the version column schema.
