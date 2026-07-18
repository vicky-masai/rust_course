# What are Database Deadlocks?

## Interview Question

What is a database deadlock in PostgreSQL and how do you prevent it?

## Interview Answer

A deadlock occurs when two or more transactions are each waiting for the other to release a lock, creating a circular dependency that neither can resolve. PostgreSQL automatically detects deadlocks using a wait-for graph and resolves them by aborting one of the transactions (the "victim"), returning error code 40P01. The victim transaction is rolled back, releasing its locks so the other transaction can proceed. Deadlocks are a normal part of concurrent programming and must be handled in application code via retry logic. Common causes include acquiring locks in inconsistent order, holding locks too long, and using SELECT FOR UPDATE without proper indexing.

---

## Follow-up Questions & Answers

### Q1. How does PostgreSQL detect deadlocks?

**Interview Answer**

PostgreSQL detects deadlocks by building a wait-for graph — a directed graph where each node is a transaction and each edge represents "Transaction A is waiting for a lock held by Transaction B." PostgreSQL checks for cycles in this graph after every lock wait (typically every 1 second by default, controlled by `deadlock_timeout`). When a cycle is found, PostgreSQL selects a victim transaction to abort — the one that has done the least work (fewest row locks). The detection is automatic and does not require any configuration. The `log_lock_waits` parameter logs all lock waits, and `log_min_messages` set to DEBUG2 provides detailed deadlock information. After detection, the victim gets ERROR: deadlock detected and must be retried.

```sql
-- Enable deadlock logging
ALTER SYSTEM SET log_lock_waits = on;
ALTER SYSTEM SET deadlock_timeout = '1s';  -- Check for deadlock every 1s
SELECT pg_reload_conf();

-- Check for deadlocks in logs
SELECT * FROM pg_stat_activity WHERE wait_event_type = 'Lock';

-- See current locks
SELECT pid, locktype, relation::regclass, mode, granted, blocking_pid
FROM pg_locks
WHERE NOT granted;

-- Manual deadlock simulation
-- Session 1:
BEGIN; UPDATE accounts SET balance = 1 WHERE id = 1;
-- Session 2:
BEGIN; UPDATE accounts SET balance = 1 WHERE id = 2;
-- Session 1:
UPDATE accounts SET balance = 1 WHERE id = 2;  -- Blocks
-- Session 2:
UPDATE accounts SET balance = 1 WHERE id = 1;  -- Deadlock! Session 2 aborted
```

---

### Q2. What are the common causes of deadlocks in PostgreSQL?

**Interview Answer**

The most common causes are: (1) Inconsistent lock ordering — Transaction A locks row 1 then row 2, while Transaction B locks row 2 then row 1; (2) Missing indexes on foreign keys — UPDATE/DELETE on a child table locks the parent table's rows, and without an index on the foreign key, PostgreSQL acquires a full table lock on the parent; (3) Long-running transactions holding locks — increases the window for deadlocks; (4) SELECT FOR UPDATE without a WHERE clause — locks the entire table; (5) Mixing explicit locks (LOCK TABLE) with row-level locks; (6) Upgrading from SHARE to ROW EXCLUSIVE lock within a transaction. The root cause is almost always lock ordering — ensure all transactions acquire locks in the same order.

```sql
-- Cause 1: Inconsistent lock ordering
-- Transaction A: locks users, then orders
BEGIN;
UPDATE users SET name = 'X' WHERE id = 1;
UPDATE orders SET total = 0 WHERE user_id = 1;  -- May deadlock with B

-- Transaction B: locks orders, then users
BEGIN;
UPDATE orders SET total = 0 WHERE user_id = 1;
UPDATE users SET name = 'Y' WHERE id = 1;  -- Deadlock!

-- Fix: always lock users first, then orders

-- Cause 2: Missing FK index
-- Without index on orders.user_id:
-- Deleting from users locks ALL orders rows (full table scan)
CREATE INDEX idx_orders_user_id ON orders (user_id);  -- Fixes the deadlock
```

---

### Q3. How do you handle deadlock errors in Rust with SQLx?

**Interview Answer**

When PostgreSQL detects a deadlock, it returns error code 40P01 ("deadlock detected"). In Rust with SQLx, you catch this error, rollback the transaction, and retry with exponential backoff. The retry should create a completely new transaction since the old one was rolled back. Use jitter (random delay) to prevent thundering herd where multiple clients retry simultaneously. SQLx provides `as_database_error()` to inspect the PostgreSQL error code. A typical implementation has a max retry count (3-5), exponential backoff (10ms, 20ms, 40ms, ...), and logs each retry attempt for debugging.

```rust
use sqlx::{PgPool, Postgres, Transaction};
use std::time::Duration;

async fn execute_with_deadlock_retry<F, Fut, T>(
    pool: &PgPool,
    max_retries: u32,
    mut operation: F,
) -> Result<T, sqlx::Error>
where
    F: FnMut(Transaction<'_, Postgres>) -> Fut,
    Fut: std::future::Future<Output = Result<T, sqlx::Error>>,
{
    for attempt in 0..max_retries {
        let mut tx = pool.begin().await?;

        match operation(tx).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.code().map_or(false, |c| c.as_ref() == "40P01") {
                        let delay = Duration::from_millis(
                            10 * 2u64.pow(attempt) + rand::random::<u64>() % 10
                        );
                        tracing::warn!(
                            attempt, ?delay,
                            "Deadlock detected, retrying transaction"
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                }
                return Err(e);
            }
        }
    }
    Err(sqlx::Error::PoolTimedOut)
}

// Usage
let result = execute_with_deadlock_retry(&pool, 5, |mut tx| async move {
    sqlx::query!("UPDATE accounts SET balance = balance - $1 WHERE id = $2", amount, from)
        .execute(&mut *tx).await?;
    sqlx::query!("UPDATE accounts SET balance = balance + $1 WHERE id = $2", amount, to)
        .execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}).await?;
```

---

### Q4. What is a deadlock_timeout and how should you configure it?

**Interview Answer**

The `deadlock_timeout` parameter controls how long PostgreSQL waits before checking for deadlocks. The default is 1 second. When a transaction blocks on a lock, PostgreSQL waits for `deadlock_timeout` before running the deadlock detection algorithm. If a deadlock is found, the victim is immediately aborted. If no deadlock is found, the transaction continues waiting. A shorter timeout detects deadlocks faster but increases CPU overhead from more frequent checks. A longer timeout delays deadlock resolution but reduces overhead. For most applications, the default of 1 second is appropriate. For high-concurrency systems with many concurrent transactions, consider reducing it to 500ms.

```sql
-- Check current setting
SHOW deadlock_timeout;  -- Default: 1s

-- Reduce for high-concurrency systems
ALTER SYSTEM SET deadlock_timeout = '500ms';
SELECT pg_reload_conf();

-- Enable detailed lock wait logging
ALTER SYSTEM SET log_lock_waits = on;
ALTER SYSTEM SET log_min_duration_statement = 0;  -- Log all queries during lock waits

-- Monitor deadlocks
SELECT datname, deadlocks
FROM pg_stat_database
WHERE datname = current_database();
```

---

### Q5. How do missing foreign key indexes cause deadlocks?

**Interview Answer**

When you delete or update a row in a parent table, PostgreSQL must check the child table for matching rows to enforce the foreign key constraint. Without an index on the foreign key column in the child table, PostgreSQL performs a sequential scan on the child table, acquiring SHARE ROW EXCLUSIVE locks on the entire child table. This broad locking dramatically increases the chance of deadlocks because any concurrent INSERT/UPDATE on the child table will conflict with these locks. The fix is simple: always create an index on foreign key columns. This is one of the most commonly overlooked causes of deadlocks in PostgreSQL.

```sql
-- Problem: no index on orders.user_id
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),  -- No index!
    total DECIMAL
);

-- Deleting from users locks ALL orders rows
BEGIN;
DELETE FROM users WHERE id = 1;  -- Acquires SHARE ROW EXCLUSIVE on orders
-- This blocks any INSERT/UPDATE on orders, potential deadlock

-- Fix: add index on foreign key
CREATE INDEX idx_orders_user_id ON orders (user_id);
-- Now DELETE from users only locks the specific matching row

-- PostgreSQL will warn about missing FK indexes:
-- WARNING: SPI_execute failed: relation "orders" does not have index on column "user_id"
```

---

### Q6. What is lock ordering and how does it prevent deadlocks?

**Interview Answer**

Lock ordering means all transactions acquire locks on resources in the same global order. If Transaction A always locks users before orders, and Transaction B always locks users before orders, a circular wait is impossible. This is the most reliable deadlock prevention technique. Implement it by defining a canonical order for all resources (e.g., table names alphabetically, or by primary key) and enforcing it in application code. For code that modifies multiple tables, document the required lock order and use code review to enforce consistency. The trade-off is reduced flexibility — some operations may need to be restructured to follow the ordering.

```sql
-- Bad: inconsistent lock ordering
-- Transaction A
BEGIN;
SELECT * FROM users WHERE id = 1 FOR UPDATE;    -- Lock user first
SELECT * FROM orders WHERE user_id = 1 FOR UPDATE;  -- Then order
COMMIT;

-- Transaction B
BEGIN;
SELECT * FROM orders WHERE user_id = 1 FOR UPDATE;  -- Lock order first
SELECT * FROM users WHERE id = 1 FOR UPDATE;          -- Then user (DEADLOCK!)
COMMIT;

-- Good: consistent lock ordering (always users before orders)
-- Transaction A
BEGIN;
SELECT * FROM users WHERE id = 1 FOR UPDATE;    -- User first
SELECT * FROM orders WHERE user_id = 1 FOR UPDATE;  -- Order second
COMMIT;

-- Transaction B
BEGIN;
SELECT * FROM users WHERE id = 1 FOR UPDATE;    -- User first (consistent!)
SELECT * FROM orders WHERE user_id = 1 FOR UPDATE;  -- Order second
COMMIT;
```

---

### Q7. How do you diagnose deadlocks in PostgreSQL after they occur?

**Interview Answer**

When a deadlock occurs, PostgreSQL logs detailed information including the transactions involved, the locks held by each, and the queries that caused the deadlock. Enable `log_lock_waits = on` to capture all lock waits, and check the PostgreSQL log for "deadlock detected" messages. The `pg_stat_activity` view shows current lock waits. The `pg_locks` view shows all active locks with `granted = false` for blocked requests. For historical analysis, use `pg_stat_database.deadlocks` counter. The `pgBadger` log analyzer can parse deadlock logs for patterns. Always include transaction IDs and query text in your logs to identify the application code path.

```sql
-- Enable comprehensive logging
ALTER SYSTEM SET log_lock_waits = on;
ALTER SYSTEM SET log_statement = 'all';  -- For debugging (remove in production)
SELECT pg_reload_conf();

-- Find blocking transactions
SELECT blocked.pid AS blocked_pid,
       blocked.query AS blocked_query,
       blocking.pid AS blocking_pid,
       blocking.query AS blocking_query,
       now() - blocked.query_start AS blocked_duration
FROM pg_stat_activity blocked
JOIN pg_locks blocked_locks ON blocked.pid = blocked_locks.pid
JOIN pg_locks blocking_locks ON blocked_locks.relation = blocking_locks.relation
  AND blocking_locks.granted = true
  AND blocked_locks.pid != blocking_locks.pid
JOIN pg_stat_activity blocking ON blocking_locks.pid = blocking.pid
WHERE NOT blocked_locks.granted;

-- Monitor deadlock count
SELECT datname, deadlocks, conflicts
FROM pg_stat_database
WHERE datname = current_database();

-- Kill a blocking transaction if needed
SELECT pg_terminate_backend(blocking_pid);
```

---

### Q8. What are advisory locks and can they help avoid deadlocks?

**Interview Answer**

Advisory locks are application-controlled locks that don't correspond to any database row or table. They can help avoid deadlocks by providing a consistent locking mechanism for application-level resources. Instead of acquiring row locks in potentially inconsistent order, you can acquire an advisory lock on a resource identifier (like a hash of the resource name) before modifying it. Session-level advisory locks are held until explicitly released; transaction-level advisory locks are released at COMMIT/ROLLBACK. The trade-off is that advisory locks bypass MVCC and must be managed manually — forgetting to release them causes resource leaks.

```sql
-- Advisory lock for consistent resource locking
-- Lock resource "account:1" before modifying
SELECT pg_advisory_xact_lock(hashtext('account:1'));
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
-- Lock automatically released at COMMIT

-- Lock multiple resources in consistent order (by hash)
SELECT pg_advisory_xact_lock(hashtext('account:1'));
SELECT pg_advisory_xact_lock(hashtext('account:2'));
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
UPDATE accounts SET balance = balance + 100 WHERE id = 2;
-- Both locks released at COMMIT

-- Try lock without blocking
SELECT pg_try_advisory_lock(hashtext('account:1'));  -- Returns true/false
```

---

### Q9. How does SELECT FOR UPDATE interact with deadlocks?

**Interview Answer**

SELECT FOR UPDATE acquires an exclusive row lock immediately, rather than waiting until an actual UPDATE/DELETE. This is useful for "read then modify" patterns but can cause deadlocks if two transactions lock the same rows in different orders. The key difference from implicit row locks is timing: SELECT FOR UPDATE locks at read time, while UPDATE locks at write time. This means a SELECT FOR UPDATE followed by UPDATE doesn't require a second lock acquisition (the row is already locked). However, if you SELECT FOR UPDATE multiple rows, ensure you always lock them in a consistent order. Use SKIP LOCKED for queue-like patterns to avoid deadlocks entirely.

```sql
-- SELECT FOR UPDATE can cause deadlocks
-- Transaction A
BEGIN;
SELECT * FROM accounts WHERE id IN (1, 2) FOR UPDATE;
-- PostgreSQL locks rows in index order (id order): 1, then 2

-- Transaction B
BEGIN;
SELECT * FROM accounts WHERE id IN (2, 1) FOR UPDATE;
-- PostgreSQL also locks in index order: 2, then 1 (same order, no deadlock!)

-- But with different index patterns, order may differ
-- Use ORDER BY to enforce consistent locking order
SELECT * FROM accounts WHERE id IN (2, 1) ORDER BY id FOR UPDATE;

-- SKIP LOCKED for queue patterns (avoids deadlocks entirely)
BEGIN;
SELECT * FROM job_queue
WHERE status = 'pending'
ORDER BY id
LIMIT 1
FOR UPDATE SKIP LOCKED;  -- Skip rows locked by other transactions
-- Process job...
UPDATE job_queue SET status = 'done' WHERE id = $1;
COMMIT;
```

---

### Q10. What is the difference between a deadlock and a lock wait?

**Interview Answer**

A lock wait occurs when one transaction is waiting for a lock held by another transaction, but there is no circular dependency — the blocking transaction will eventually commit or rollback, releasing the lock. A deadlock is a special case of lock wait where a circular dependency exists — Transaction A waits for B, and B waits for A, so neither can proceed. PostgreSQL automatically detects and resolves deadlocks by aborting one transaction. Lock waits are resolved naturally when the blocking transaction finishes. To distinguish between them in monitoring: deadlocks are logged as errors with error code 40P01, while lock waits show up in `pg_stat_activity` with `wait_event_type = 'Lock'`. Configure `lock_timeout` to fail lock waits that exceed a threshold, preventing indefinite blocking.

```sql
-- Lock wait (not a deadlock)
-- Session 1 holds a lock on row 1
BEGIN;
UPDATE accounts SET balance = 1 WHERE id = 1;
-- Don't commit yet...

-- Session 2 waits for the lock (not a deadlock)
BEGIN;
UPDATE accounts SET balance = 2 WHERE id = 1;
-- Blocks here, waiting for Session 1 to commit/rollback

-- Set lock timeout to prevent indefinite waits
SET lock_timeout = '5s';  -- Fail after 5 seconds instead of waiting forever
-- ERROR: lock timeout after 5000 ms

-- Monitor lock waits
SELECT pid, state, wait_event_type, wait_event, query
FROM pg_stat_activity
WHERE wait_event_type = 'Lock';

-- Kill a blocking session if needed
SELECT pg_terminate_backend(<blocking_pid>);
```
