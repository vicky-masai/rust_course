# optimistic vs pessimistic locking

## Interview Question

Explain optimistic vs pessimistic locking.

## Interview Answer

- Optimistic: version column, retry on conflict.
- Pessimistic: `SELECT ... FOR UPDATE`, lock before update.

---

## Follow-up Questions & Answers

### Q1. When should you choose optimistic over pessimistic locking in production?

**Interview Answer**

Choose optimistic locking when conflicts are rare, like most read-heavy APIs with occasional writes. It avoids lock overhead and allows higher throughput under low contention. Pessimistic locking is better for high-contention scenarios like inventory management where conflicts are frequent and retries waste resources.

---

### Q2. How do you implement retry logic for optimistic locking failures in Axum?

**Interview Answer**

Wrap the read-modify-write cycle in a retry loop with exponential backoff using `tokio::time::sleep`. Limit retries to 3-5 attempts and return `409 Conflict` if retries are exhausted. Log the retry count for monitoring and alert on high retry rates indicating contention issues.

---

### Q3. What are the deadlock risks with pessimistic locking?

**Interview Answer**

Deadlocks occur when two transactions hold locks and wait for each other's locks. Prevent by always acquiring locks in the same order and using `SELECT ... FOR UPDATE NOWAIT` to fail immediately instead of waiting. Implement deadlock detection and retry logic in your Axum handlers.

---

### Q4. How does isolation level affect locking behavior?

**Interview Answer**

Read Committed doesn't prevent phantom reads even with locking, while Repeatable Read provides stronger guarantees. SERIALIZABLE isolation in PostgreSQL uses predicate locking which can cause more contention. Choose the appropriate isolation level based on your consistency requirements and performance tradeoffs.

---

### Q5. Can you combine optimistic and pessimistic locking?

**Interview Answer**

Yes, use pessimistic locking for the critical section and optimistic locking as a final check. For example, lock rows during complex operations but still verify version before commit. This provides defense-in-depth against concurrent modifications in payment or inventory systems.

---

### Q6. How do you test both locking strategies in integration tests?

**Interview Answer**

Use `sqlx::test` with concurrent Tokio tasks that attempt simultaneous updates on the same row. For optimistic locking, verify that one update succeeds and others receive version mismatch errors. For pessimistic locking, verify that one transaction blocks until the other completes.

---

### Q7. What are the scalability implications of each approach?

**Interview Answer**

Optimistic locking scales better because it doesn't hold locks, allowing unlimited concurrent readers. Pessimistic locking limits scalability due to lock contention and potential deadlocks under high load. Measure throughput under concurrent load with `hey` or `wrk` to determine which approach meets your scale requirements.

---

### Q8. How do you handle locking across multiple tables in a single transaction?

**Interview Answer**

Acquire locks on all tables in a consistent order to prevent deadlocks, or use `SELECT ... FOR UPDATE NOWAIT` on each table. In sqlx, use a single transaction to lock rows across tables before performing updates. Keep multi-table transactions as short as possible to minimize lock hold time.
