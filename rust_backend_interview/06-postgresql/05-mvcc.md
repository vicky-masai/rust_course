# MVCC

## Interview Question

Explain how MVCC works in PostgreSQL in detail.

## Interview Answer

MVCC (Multi-Version Concurrency Control) in PostgreSQL works by keeping multiple physical versions of every row, indexed by transaction ID. When a transaction reads a row, it uses a snapshot that determines which row versions are visible. Writers never block readers and readers never block writers because they operate on different row versions. The xmin and xmax system columns in each row track which transaction created and invalidated the version. The visibility rules depend on the isolation level: READ COMMITTED takes a new snapshot per statement, while REPEATABLE READ takes a snapshot once at transaction start. VACUUM cleans up obsolete versions to prevent table bloat and XID wraparound.

---

## Follow-up Questions & Answers

### Q1. What is the difference between READ COMMITTED and REPEATABLE READ in MVCC?

**Interview Answer**

READ COMMITTED is PostgreSQL's default isolation level. Each statement within a transaction gets a fresh snapshot, so it can see rows committed by other transactions between statements. This means non-repeatable reads are possible — querying the same row twice within a transaction may return different values. REPEATABLE READ takes a single snapshot at the start of the transaction and uses it for all subsequent statements. This guarantees that repeated reads of the same rows return the same values throughout the transaction. However, REPEATABLE READ can still allow phantom reads (new rows inserted by others appearing in range scans). Only SERIALIZABLE prevents phantoms through predicate locking.

```sql
-- READ COMMITTED: fresh snapshot per statement
BEGIN;
SET transaction_isolation = 'read committed';
SELECT balance FROM accounts WHERE id = 1;  -- balance = 1000
-- Another transaction commits UPDATE ... SET balance = 500 WHERE id = 1
SELECT balance FROM accounts WHERE id = 1;  -- balance = 500 (sees new commit!)
COMMIT;

-- REPEATABLE READ: single snapshot at start
BEGIN;
SET transaction_isolation = 'repeatable read';
SELECT balance FROM accounts WHERE id = 1;  -- balance = 1000
-- Another transaction commits UPDATE ... SET balance = 500 WHERE id = 1
SELECT balance FROM accounts WHERE id = 1;  -- balance = 1000 (same snapshot!)
COMMIT;
```

---

### Q2. What is a phantom read and does PostgreSQL prevent it?

**Interview Answer**

A phantom read occurs when a transaction queries a range of rows twice and finds different rows the second time — new rows inserted by another committed transaction appear between the two queries. PostgreSQL's REPEATABLE READ isolation level does NOT prevent phantom reads entirely. It uses predicate locking only for specific patterns (key-range locks for Serializable Snapshot Isolation), but plain REPEATABLE READ relies on row-level visibility checks that can miss newly inserted rows. SERIALIZABLE isolation in PostgreSQL uses Predicate Locks and SSI (Serializable Snapshot Isolation) to detect and prevent phantoms by tracking read-write dependencies. For most applications, REPEATABLE READ with retry logic is sufficient.

```sql
-- Phantom read example
-- Transaction A
BEGIN;
SET transaction_isolation = 'repeatable read';
SELECT COUNT(*) FROM orders WHERE created_at > '2025-07-01';  -- 10 rows
-- Transaction B inserts a new order and commits
COMMIT;  -- After B commits

-- Transaction A again (with REPEATABLE READ)
BEGIN;
SET transaction_isolation = 'repeatable read';
SELECT COUNT(*) FROM orders WHERE created_at > '2025-07-01';  -- 10 rows (same snapshot)
INSERT INTO orders (customer_id, total) VALUES (1, 50.00);
SELECT COUNT(*) FROM orders WHERE created_at > '2025-07-01';  -- 11 rows! Phantom appeared
COMMIT;
```

---

### Q3. What is SSI (Serializable Snapshot Isolation) and how does it work?

**Interview Answer**

SSI is PostgreSQL's implementation of true SERIALIZABLE isolation. It builds on MVCC snapshots and adds dependency tracking between transactions. When a transaction reads data, SSI records "rw-dependencies" — if Transaction A reads a row that Transaction B later modifies, a dependency edge is created. Before committing, SSI checks for dangerous patterns (cycles in the dependency graph) that would indicate non-serializable execution. If a cycle is detected, one transaction is aborted with a serialization error. SSI allows much higher concurrency than traditional two-phase locking because it only aborts transactions that actually conflict, not all concurrent writers. The trade-off is occasional false-positive aborts that must be retried.

```sql
-- SSI detects write skew anomaly
-- Transaction A
BEGIN;
SET transaction_isolation = 'serializable';
SELECT COUNT(*) FROM doctors WHERE on_call = true;  -- 2 doctors on call
UPDATE doctors SET on_call = false WHERE name = 'Dr. Smith';
COMMIT;  -- May fail with serialization_error if B also ran

-- Transaction B (concurrent)
BEGIN;
SET transaction_isolation = 'serializable';
SELECT COUNT(*) FROM doctors WHERE on_call = true;  -- 2 doctors on call
UPDATE doctors SET on_call = false WHERE name = 'Dr. Jones';
COMMIT;  -- One of A or B gets aborted to prevent both doctors being off-call

-- Catch serialization errors in Rust
match sqlx::query("...").execute(&mut *tx).await {
    Err(e) if e.as_database_error().map_or(false, |d| d.code() == Some("40001".into())) => {
        // Serialization failure, retry
    }
    Err(e) => return Err(e.into()),
    Ok(_) => {}
}
```

---

### Q4. What is a snapshot in MVCC and how is it stored?

**Interview Answer**

A snapshot is a data structure that records the state of all in-flight transactions at a specific point in time. It contains three key arrays: `xip` (transactions in progress — not yet committed), `xact_sum` (total transactions committed), and `xid` (the snapshot's own transaction ID). When PostgreSQL needs to determine row visibility, it compares the row's xmin and xmax against these arrays. Snapshots are stored in shared memory for the duration of a transaction. The `pg_export_snapshot()` function allows one transaction to share its snapshot with another. For long-running transactions, holding a snapshot prevents VACUUM from cleaning up old row versions, causing table bloat. This is why keeping transactions short is critical for MVCC performance.

```sql
-- See active snapshots
SELECT pid, usename, active, query, backend_xmin
FROM pg_stat_activity
WHERE backend_xmin IS NOT NULL;

-- Export a snapshot for use by another session
SELECT pg_export_snapshot();
-- Returns a snapshot ID that can be imported

-- Import a snapshot in another session
BEGIN;
SET TRANSACTION SNAPSHOT 'snapshot_id';

-- Check for long-running transactions holding snapshots
SELECT pid, usename, state, xact_start,
       now() - xact_start AS duration,
       backend_xmin
FROM pg_stat_activity
WHERE state != 'idle'
ORDER BY duration DESC;
```

---

### Q5. How does MVCC handle UPDATE and DELETE operations?

**Interview Answer**

When PostgreSQL executes an UPDATE, it does not modify the row in place. Instead, it marks the old row version by setting its xmax to the current transaction ID, then creates a new row version (tuple) with the updated data and sets its xmin to the current transaction. The old and new versions coexist in the table. For DELETE, PostgreSQL simply sets the xmax of the row to the current transaction ID — the row is not physically removed. The old versions become "dead tuples" once the deleting/updating transaction commits and no active transaction needs to see them. VACUUM then reclaims the dead tuple space. This approach means UPDATE and DELETE are essentially INSERT operations from a storage perspective.

```sql
-- UPDATE creates a new version
BEGIN;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
-- Old row: xmax = current_xact
-- New row: xmin = current_xact, balance = balance - 100
SELECT ctid, xmin, xmax, balance FROM accounts WHERE id = 1;
COMMIT;

-- DELETE marks the row as dead
BEGIN;
DELETE FROM accounts WHERE id = 1;
-- Row: xmax = current_xact
-- Row is still physically present until VACUUM
SELECT ctid, xmin, xmax FROM accounts WHERE id = 1;  -- Still visible!
COMMIT;
-- Row invisible to new transactions but physically present

-- VACUUM removes dead tuples
VACUUM accounts;
```

---

### Q6. What is table bloat in MVCC and how do you prevent it?

**Interview Answer**

Table bloat occurs when dead tuples accumulate faster than VACUUM can reclaim them. Each dead tuple consumes storage and increases the number of pages PostgreSQL must scan during sequential scans. Bloat directly degrades read performance because more data must be read from disk into memory. Common causes: long-running transactions that prevent VACUUM from cleaning dead tuples, insufficient autovacuum settings for high-write tables, and lack of manual VACUUM after bulk operations. Prevent bloat by: tuning autovacuum thresholds for heavy-write tables (`ALTER TABLE ... SET autovacuum_vacuum_scale_factor = 0.01`), running VACUUM ANALYZE after bulk loads, using `pgstattuple` to measure bloat, and periodically running VACUUM FULL (which rewrites the table but blocks all access).

```sql
-- Measure bloat using pgstattuple
CREATE EXTENSION IF NOT EXISTS pgstattuple;
SELECT * FROM pgstattuple('orders');
-- Look at: dead_tuple_percent, free_percent

-- High dead_tuple_percent = needs vacuum
SELECT relname, n_live_tup, n_dead_tup,
       pg_size_pretty(pg_relation_size(oid)) AS size,
       round(100.0 * n_dead_tup / NULLIF(n_live_tup + n_dead_tup, 0), 2) AS dead_pct
FROM pg_stat_user_tables
WHERE n_dead_tup > 10000
ORDER BY dead_pct DESC;

-- Fix bloat
VACUUM ANALYZE orders;  -- Non-blocking, reclaims dead tuples

-- For severe bloat (blocks writes)
VACUUM FULL orders;  -- Rewrites table, returns space to OS

-- Prevent bloat with aggressive autovacuum
ALTER TABLE orders SET (
    autovacuum_vacuum_scale_factor = 0.01,
    autovacuum_vacuum_threshold = 50,
    autovacuum_analyze_scale_factor = 0.005
);
```

---

### Q7. What is a serialization error and how should Rust applications handle it?

**Interview Answer**

A serialization error (PostgreSQL error code 40001) occurs when the database detects that concurrent transactions would produce an inconsistent result if serialized. Under REPEATABLE READ, this happens when a transaction tries to update a row that was modified by a committed transaction after the reader's snapshot was taken. Under SERIALIZABLE, it happens when SSI detects a dependency cycle. Rust applications must handle these errors by retrying the transaction with exponential backoff. SQLx provides the error code through its database error API. A robust pattern is a retry loop with configurable max attempts and jitter to avoid thundering herd effects. The transaction should be rolled back and a new one started from scratch.

```rust
use sqlx::{PgPool, Transaction, Postgres};
use std::time::Duration;

async fn execute_with_retry<F, Fut, T>(
    pool: &PgPool,
    max_retries: u32,
    mut f: F,
) -> Result<T, sqlx::Error>
where
    F: FnMut(Transaction<'_, Postgres>) -> Fut,
    Fut: std::future::Future<Output = Result<(Transaction<'_, Postgres>, T), sqlx::Error>>,
{
    for attempt in 0..max_retries {
        let mut tx = pool.begin().await?;

        match f(tx).await {
            Ok((tx, result)) => {
                tx.commit().await?;
                return Ok(result);
            }
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.code().map_or(false, |c| c.as_ref() == "40001") {
                        // Serialization failure — retry
                        let delay = Duration::from_millis(10 * 2u64.pow(attempt));
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
```

---

### Q8. How does MVCC differ from lock-based concurrency control?

**Interview Answer**

Lock-based concurrency control (used by databases like MySQL InnoDB's default) uses shared and exclusive locks — readers acquire shared locks and writers acquire exclusive locks, causing readers to block writers and vice versa. MVCC eliminates this conflict by giving each transaction its own snapshot — readers never wait for writers because they see a frozen view of committed data. The advantage of MVCC is dramatically higher throughput for mixed read/write workloads. The disadvantage is increased storage (multiple row versions), vacuum overhead, and complexity in handling visibility. PostgreSQL's MVCC is superior for OLTP workloads with many concurrent readers, while lock-based systems may have lower storage overhead for write-heavy workloads.

```sql
-- Lock-based: reader blocks writer (not PostgreSQL behavior)
BEGIN;
SELECT balance FROM accounts WHERE id = 1;  -- Acquires shared lock
-- Another transaction: UPDATE accounts SET balance = 0 WHERE id = 1
-- Blocks! Waiting for shared lock to be released

-- MVCC: reader never blocks writer (PostgreSQL behavior)
BEGIN;
SELECT balance FROM accounts WHERE id = 1;  -- Uses snapshot, no lock needed
-- Another transaction: UPDATE accounts SET balance = 0 WHERE id = 1
-- Proceeds immediately! Creates new row version
SELECT balance FROM accounts WHERE id = 1;  -- Still sees old version (snapshot)

-- Locking when you explicitly need it
BEGIN;
SELECT * FROM accounts WHERE id = 1 FOR UPDATE;  -- Acquires row lock
-- This blocks other FOR UPDATE attempts on the same row
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
COMMIT;
```

---

### Q9. What are frozen transaction IDs and why are they important?

**Interview Answer**

Frozen transaction IDs (FrozenTransactionId) are special sentinel values that replace old xmin values during VACUUM's freeze cycle. When a row's xmin becomes older than `autovacuum_freeze_max_age` (default: 200 million transactions), VACUUM replaces it with FrozenTransactionId, which is treated as always visible to all transactions. This is critical for preventing transaction ID wraparound — since XIDs are 32-bit unsigned integers, they wrap after ~4.2 billion values. Without freezing, wrapped XIDs could make old data appear "from the future" and be incorrectly hidden. Freezing is automatic but you should monitor `pg_database.datfrozenxid` age to ensure VACUUM is keeping up.

```sql
-- Check frozen XID age per database
SELECT datname, age(datfrozenxid) AS xid_age,
       round(age(datfrozenxid) / 210000000.0 * 100, 2) AS pct_to_wraparound
FROM pg_database
ORDER BY xid_age DESC;

-- Check per-table frozen XID age
SELECT relname, age(relfrozenxid) AS table_xid_age
FROM pg_class
WHERE relkind = 'r' AND relfrozenxid != 0
ORDER BY table_xid_age DESC
LIMIT 10;

-- Force freeze on a specific table
VACUUM FREEZE orders;

-- Monitor for freeze warnings
-- PostgreSQL logs WARNING when age exceeds autovacuum_freeze_max_age
-- Logs ERROR at 1.2 billion, enters panic mode at 2 billion
```

---

### Q10. What are common MVCC-related pitfalls in PostgreSQL applications?

**Interview Answer**

The most common pitfalls are: (1) Long-running transactions that prevent VACUUM from cleaning dead tuples, causing table bloat — always keep transactions short and release connections promptly; (2) Idle connections in a pool holding back VACUUM because their backend_xmin is still referenced — set connection timeouts and idle-in-transaction timeouts; (3) Not handling serialization errors in REPEATABLE READ/SERIALIZABLE — applications must retry; (4) Accidental table bloat after bulk operations without running VACUUM ANALYZE; (5) SELECT FOR UPDATE without a WHERE clause locking the entire table; (6) Using REPEATABLE READ when READ COMMITTED would suffice, increasing contention unnecessarily. For Rust backends, always set `idle_in_transaction_session_timeout` and handle pool acquisition timeouts.

```sql
-- Set idle-in-transaction timeout (prevents MVCC snapshot leaks)
ALTER SYSTEM SET idle_in_transaction_session_timeout = '30s';
SELECT pg_reload_conf();

-- Monitor connections holding back VACUUM
SELECT pid, usename, state, backend_xmin,
       xact_start, now() - xact_start AS xact_age,
       query
FROM pg_stat_activity
WHERE backend_xmin IS NOT NULL
  AND backend_xmin < (SELECT xact_start FROM pg_stat_activity WHERE pid = backend_xmin)
ORDER BY xact_age DESC;

-- Kill long-running idle-in-transaction connections
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE state = 'idle in transaction'
  AND now() - state_change > interval '5 minutes';
```
