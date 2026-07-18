# What is MVCC?

## Interview Question

What is MVCC in PostgreSQL?

## Interview Answer

MVCC stands for Multi-Version Concurrency Control. It is PostgreSQL's mechanism for allowing multiple transactions to access the database simultaneously without blocking each other. Instead of using locks for every read operation, PostgreSQL maintains multiple versions of each row. When a row is updated, PostgreSQL doesn't overwrite the old data — it creates a new version and marks the old version as obsolete. Readers see a consistent snapshot of the data as it existed at the start of their transaction, while writers create new row versions concurrently. This eliminates read-write conflicts and is the foundation of PostgreSQL's excellent concurrency performance.

---

## Follow-up Questions & Answers

### Q1. How does PostgreSQL implement MVCC internally?

**Interview Answer**

PostgreSQL implements MVCC by storing transaction IDs ( xmin and xmax ) in every row header. The `xmin` field records which transaction created (or last updated) the row. The `xmax` field records which transaction deleted or updated the row (0 if the row is still live). When a transaction reads a row, PostgreSQL checks whether `xmin` is visible (committed before the reader's snapshot) and `xmax` is not yet visible (not committed or committed after the snapshot). This visibility check happens at the tuple level, not the table level, allowing extremely fine-grained concurrency. Old row versions (dead tuples) are eventually reclaimed by VACUUM.

```sql
-- You can see MVCC fields using ctid and system columns
SELECT ctid, xmin, xmax, id, name
FROM users
WHERE id = 1;

-- ctid shows (page, offset) of the physical row version
-- xmin = transaction that created this version
-- xmax = transaction that deleted/updated this version (0 if alive)

-- After an UPDATE, you can see the old version is still there
UPDATE users SET name = 'Alice' WHERE id = 1;
SELECT ctid, xmin, xmax, name FROM users WHERE id = 1;
-- New version with new ctid, old version marked with xmax set
```

---

### Q2. What is transaction visibility in MVCC?

**Interview Answer**

Transaction visibility determines which rows a given transaction can see. Each transaction receives a snapshot at its start that records all currently committed transactions. A row is visible to a transaction if: (1) its `xmin` is in the snapshot (committed before the snapshot), (2) its `xmin` is the current transaction itself, and (3) its `xmax` is either 0 (never deleted), not in the snapshot (deleted by a future transaction), or the current transaction. This means each transaction sees a frozen, consistent view of the database as of its start time, regardless of concurrent modifications. The isolation level (READ COMMITTED vs REPEATABLE READ) determines whether the snapshot is taken per-statement or per-transaction.

```sql
-- Transaction A (snapshot taken at t1)
BEGIN;
SELECT * FROM accounts WHERE id = 1;  -- balance = 1000

-- Transaction B updates (t2, visible to new transactions)
BEGIN;
UPDATE accounts SET balance = 500 WHERE id = 1;
COMMIT;

-- Transaction A still sees old value (snapshot is frozen)
SELECT * FROM accounts WHERE id = 1;  -- balance = 1000 (still!)
COMMIT;
```

---

### Q3. What is the difference between xmin and xmax in PostgreSQL?

**Interview Answer**

The `xmin` system column records the transaction ID that inserted or last updated a row. It is set when the row version is created and never changes. The `xmax` records the transaction ID that deleted or updated the row, creating a new version. If `xmax` is 0, the row version is still live (not deleted or updated). When a row is updated, the old version gets its `xmax` set to the updating transaction, and a new row version is created with that transaction as its `xmin`. When a row is deleted, only `xmax` is set. VACUUM cleans up old versions where both xmin and xmax are no longer needed by any active transaction.

```sql
-- Insert a row (only xmin is set)
INSERT INTO products (name, price) VALUES ('Widget', 9.99);
SELECT xmin, xmax, * FROM products;
-- xmin=100, xmax=0

-- Update the row (old version xmax set, new version xmin set)
UPDATE products SET price = 12.99 WHERE name = 'Widget';
SELECT ctid, xmin, xmax, price FROM products;
-- (0,1) xmin=100, xmax=101  (old version, marked obsolete)
-- (0,2) xmin=101, xmax=0    (new version, current)

-- Delete the row
DELETE FROM products WHERE name = 'Widget';
-- xmin=101, xmax=102  (marked as deleted)
```

---

### Q4. What are transaction IDs and why do they matter for MVCC?

**Interview Answer**

Every transaction in PostgreSQL gets a unique, monotonically increasing 32-bit transaction ID (XID). XIDs are used to determine the visibility of row versions — the core mechanism of MVCC. PostgreSQL uses comparison of XIDs to determine ordering: an XID `a` is before XID `b` if `(b - a) < 2^31` (accounting for wraparound). The `freeze_max_age` parameter controls when old XIDs are frozen (replaced with a special FrozenTransactionId) to prevent XID wraparound, which would be catastrophic if unhandled. VACUUM handles XID freezing automatically. You can monitor XID consumption via `txid_current()` and `pg_last_xact_replay_timestamp()`.

```sql
-- Current transaction ID
SELECT txid_current();

-- See all active transactions and their XIDs
SELECT pid, usename, application_name, backend_xmin,
       xact_start, query
FROM pg_stat_activity
WHERE state = 'active';

-- Check XID wraparound risk
SELECT age(datfrozenxid) AS xid_age, datname
FROM pg_database
ORDER BY xid_age DESC;

-- Monitor transaction ID consumption rate
SELECT datname, age(datfrozenxid) AS xid_age,
       pg_size_pretty(pg_database_size(datname)) AS size
FROM pg_database;
```

---

### Q5. How does VACUUM interact with MVCC?

**Interview Answer**

VACUUM is the process that reclaims space occupied by dead tuples — old row versions that are no longer visible to any active transaction. Because MVCC keeps old versions around for concurrent readers, dead tuples accumulate on the table. VACUUM marks the space used by dead tuples as available for reuse (without returning it to the OS) and updates the visibility map, which is crucial for Index Only Scans. VACUUM also freezes old transaction IDs to prevent XID wraparound. Autovacuum runs automatically based on thresholds, but heavily updated tables may need manual vacuuming. Without VACUUM, tables grow indefinitely (table bloat), sequential scans slow down, and eventually PostgreSQL forces autovacuum to prevent XID wraparound.

```sql
-- Check if VACUUM is keeping up
SELECT relname, n_live_tup, n_dead_tup,
       last_vacuum, last_autovacuum,
       last_analyze, last_autoanalyze
FROM pg_stat_user_tables
WHERE n_dead_tup > 10000
ORDER BY n_dead_tup DESC;

-- Manual vacuum with detailed output
VACUUM (VERBOSE, ANALYZE) users;

-- Check visibility map (1.0 = all-visible, good for index-only scans)
SELECT relname, pg_size_pretty(pg_relation_size(relid)) AS size,
       round(100.0 * n_live_tup / NULLIF(n_live_tup + n_dead_tup, 0), 2) AS live_pct
FROM pg_stat_user_tables
WHERE schemaname = 'public';

-- Tune autovacuum for heavily updated tables
ALTER TABLE orders SET (
    autovacuum_vacuum_scale_factor = 0.01,
    autovacuum_analyze_scale_factor = 0.005
);
```

---

### Q6. What happens when a transaction reads data modified by an uncommitted transaction?

**Interview Answer**

Under READ COMMITTED isolation (the default), each statement within a transaction gets a fresh snapshot. If a statement encounters a row modified by an uncommitted transaction, it waits (blocks) until that transaction either commits or rolls back. If the modifying transaction commits, the reader sees the new data. If it rolls back, the reader sees the old data. This is called "read blocked on write." Under REPEATABLE READ, the transaction would get a serialization error (ERROR: could not serialize access due to concurrent update) rather than blocking, because it cannot see uncommitted changes from its snapshot. This is a key difference between isolation levels and affects application error handling.

```sql
-- Transaction A (uncommitted update)
BEGIN;
UPDATE accounts SET balance = 0 WHERE id = 1;
-- Still open, not committed

-- Transaction B (blocked in READ COMMITTED)
BEGIN;
SELECT balance FROM accounts WHERE id = 1;
-- Blocks here, waiting for Transaction A to commit or rollback

-- If Transaction A commits:
-- Transaction B sees balance = 0

-- If Transaction A rolls back:
-- Transaction B sees balance = 1000 (original)
```

---

### Q7. How does SQLx handle MVCC-related concepts in Rust?

**Interview Answer**

SQLx doesn't expose MVCC internals directly — it's an abstraction layer over PostgreSQL's wire protocol. However, understanding MVCC matters for Rust applications because: (1) long-running transactions in SQLx hold back VACUUM and cause table bloat — use short transactions and release connections promptly; (2) serialization errors under REPEATABLE READ must be caught and retried in Rust code; (3) connection pool timeouts prevent indefinite blocking from uncommitted writes. SQLx's `acquire_timeout` and pool configuration help mitigate MVCC-related performance issues. When using `sqlx::Transaction`, always `.commit()` or `.rollback()` explicitly to avoid holding MVCC snapshots.

```rust
use sqlx::PgPool;
use sqlx::Transaction;

// Handle serialization errors from REPEATABLE READ
async fn transfer(pool: &PgPool, from: i64, to: i64, amount: rust_decimal::Decimal) -> Result<()> {
    for attempt in 0..3 {
        let mut tx: Transaction<'_, sqlx::Postgres> = pool.begin().await?;

        sqlx::query!("UPDATE accounts SET balance = balance - $1 WHERE id = $2", amount, from)
            .execute(&mut *tx).await?;
        sqlx::query!("UPDATE accounts SET balance = balance + $1 WHERE id = $2", amount, to)
            .execute(&mut *tx).await?;

        match tx.commit().await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if e.as_database_error()
                    .map_or(false, |d| d.code().map_or(false, |c| c == "40001"))
                {
                    // Serialization failure, retry
                    tokio::time::sleep(Duration::from_millis(10 * 2u64.pow(attempt))).await;
                    continue;
                }
                return Err(e.into());
            }
        }
    }
    Err("Transfer failed after retries".into())
}

// Set isolation level on transaction
let mut tx = pool.begin().await?;
sqlx::query!("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
    .execute(&mut *tx).await?;
```

---

### Q8. What is transaction ID wraparound and how does it affect MVCC?

**Interview Answer**

PostgreSQL uses 32-bit unsigned integers for transaction IDs, meaning there are approximately 4.2 billion possible XIDs. After that many transactions, XIDs wrap around to 0. Since MVCC uses XID comparisons to determine visibility, a wrapped XID could make old rows appear "from the future" and be incorrectly visible or invisible. PostgreSQL prevents this by "freezing" old XIDs — replacing them with a special FrozenTransactionId value that is always visible. VACUUM performs this freezing when `age(datfrozenxid)` exceeds `autovacuum_freeze_max_age` (default: 200 million). If VACUUM cannot keep up and XIDs approach wraparound, PostgreSQL enters "shutdown recovery mode" and stops accepting writes. Monitor `pg_database.datfrozenxid` age to prevent this.

```sql
-- Check XID age for all databases
SELECT datname, age(datfrozenxid) AS xid_age,
       age(datfrozenxid) / 210000000.0 AS wraparound_risk_pct
FROM pg_database
ORDER BY xid_age DESC;

-- Critical warning if age approaches 1.5 billion (out of 2.1 billion safe range)
-- PostgreSQL warns at autovacuum_freeze_max_age (200M default)
-- Shuts down at 2 billion

-- Prevent wraparound by forcing vacuum
VACUUM FREEZE orders;

-- Monitor for databases at risk
SELECT relname, age(relfrozenxid) AS table_xid_age
FROM pg_class
WHERE relkind = 'r'
ORDER BY table_xid_age DESC
LIMIT 10;
```
