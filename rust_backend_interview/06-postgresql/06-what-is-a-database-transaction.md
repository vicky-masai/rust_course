# What is a Database Transaction?

## Interview Question

What is a database transaction in PostgreSQL?

## Interview Answer

A database transaction is a logical unit of work that groups one or more SQL statements into an atomic operation — either all statements succeed and are committed, or all are rolled back. PostgreSQL transactions follow the ACID properties: Atomicity guarantees all-or-nothing execution, Consistency ensures the database moves from one valid state to another, Isolation prevents concurrent transactions from interfering with each other (via MVCC), and Durability guarantees committed data survives crashes via WAL (Write-Ahead Logging). By default, PostgreSQL runs in READ COMMITTED isolation mode, where each statement sees only data committed before that statement began. Transactions start with `BEGIN` and end with `COMMIT` (success) or `ROLLBACK` (failure).

---

## Follow-up Questions & Answers

### Q1. What are the ACID properties and how does PostgreSQL implement each one?

**Interview Answer**

Atomicity means all statements in a transaction either fully complete or fully fail — PostgreSQL implements this via the undo log embedded in WAL, allowing incomplete transactions to be rolled back on crash. Consistency means the database always satisfies integrity constraints — PostgreSQL enforces CHECK, UNIQUE, FOREIGN KEY, and NOT NULL constraints within each transaction. Isolation means concurrent transactions don't interfere — PostgreSQL uses MVCC to give each transaction a consistent snapshot. Durability means committed data survives crashes — PostgreSQL uses WAL (Write-Ahead Logging), where changes are written to the WAL before being applied to data pages. If the server crashes, PostgreSQL replays the WAL to recover committed data.

```sql
-- ACID example: bank transfer
BEGIN;
-- Atomicity: both statements succeed or both fail
UPDATE accounts SET balance = balance - 500 WHERE id = 1;
UPDATE accounts SET balance = balance + 500 WHERE id = 2;

-- Consistency: if either account doesn't exist, FK violation rolls back
-- Isolation: other transactions see either both updates or neither
-- Durability: once committed, data survives crash

COMMIT;
-- If any error occurs before COMMIT, ROLLBACK happens automatically
```

---

### Q2. What are the different transaction isolation levels in PostgreSQL?

**Interview Answer**

PostgreSQL supports four isolation levels: READ COMMITTED (default) — each statement gets a fresh snapshot, so non-repeatable reads are possible; REPEATABLE READ — a single snapshot at transaction start, prevents non-repeatable reads but allows phantoms; SERIALIZABLE — uses SSI (Serializable Snapshot Isolation) to detect read-write conflicts and prevent all anomalies; READ UNCOMMITTED — technically supported but treated identically to READ COMMITTED in PostgreSQL because MVCC doesn't allow dirty reads. The isolation level is set per-transaction with `SET TRANSACTION ISOLATION LEVEL`. Higher isolation provides more consistency but reduces concurrency and increases the chance of serialization errors that require retries.

```sql
-- Set isolation level for a transaction
BEGIN;
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;
SELECT * FROM accounts WHERE id = 1;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
COMMIT;

-- Set default for all future transactions in session
SET default_transaction_isolation = 'repeatable read';

-- Check current isolation level
SHOW transaction_isolation;

-- Serializable for critical operations
BEGIN;
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;
-- SSI will detect and abort conflicting transactions
SELECT * FROM inventory WHERE product_id = 42 FOR UPDATE;
UPDATE inventory SET quantity = quantity - 1 WHERE product_id = 42;
COMMIT;
```

---

### Q3. What is the lifecycle of a PostgreSQL transaction?

**Interview Answer**

A transaction lifecycle has several stages: (1) BEGIN — allocates a transaction ID (XID) and takes a snapshot depending on isolation level; (2) Statement execution — SQL statements are parsed, planned, and executed, with all changes written to WAL and held in memory; (3) COMMIT — flushes WAL to disk (fsync), making changes durable, marks the transaction as committed in clog (commit log), and releases row locks; (4) ROLLBACK — undoes all changes by marking the transaction as aborted in clog and releasing locks. Between BEGIN and COMMIT, MVCC row versions accumulate. The clog (commit log) is a shared memory structure that tracks which transactions are committed, aborted, or in-progress, and is consulted for every visibility check.

```sql
-- Full transaction lifecycle
BEGIN;                                    -- 1. Allocate XID, take snapshot
SELECT * FROM users WHERE id = 1;         -- 2. Read using snapshot
UPDATE users SET name = 'New' WHERE id = 1; -- 3. Write new row version
INSERT INTO audit_log (action) VALUES ('update'); -- 4. Write audit
COMMIT;                                   -- 5. Flush WAL, mark committed

-- Rollback lifecycle
BEGIN;
DELETE FROM users WHERE id = 1;           -- Row marked xmax = XID
-- Oh no, wrong user!
ROLLBACK;                                 -- XID marked aborted, row restored
SELECT * FROM users WHERE id = 1;         -- Row still visible
```

---

### Q4. What happens if a PostgreSQL transaction fails or the server crashes?

**Interview Answer**

If a transaction fails (e.g., constraint violation, deadlock), PostgreSQL automatically rolls it back — the XID is marked as aborted in clog, all row modifications become invisible, and locks are released. If the server crashes mid-transaction, on restart PostgreSQL replays the WAL to recover committed transactions and undo incomplete ones. This crash recovery process is automatic and ensures durability. The `max_wal_size` parameter controls how much WAL is retained for recovery. Uncommitted changes are never visible to other transactions, even during recovery. The only data loss scenario is if a commit's WAL record wasn't flushed to disk before the crash — but PostgreSQL's `synchronous_commit` setting (default: on) ensures this doesn't happen.

```sql
-- Transaction failure (automatic rollback)
BEGIN;
INSERT INTO users (email) VALUES ('duplicate@example.com');
-- ERROR: duplicate key value violates unique constraint
-- Transaction is automatically aborted

-- Must explicitly start a new transaction
BEGIN;
INSERT INTO users (email) VALUES ('new@example.com');
COMMIT;

-- Check WAL settings for durability
SHOW synchronous_commit;    -- on by default
SHOW wal_level;             -- replica or logical
SHOW max_wal_size;          -- 1GB default

-- Crash-safe: synchronous commit ensures WAL is on disk before ACK
SET synchronous_commit = on;    -- Default, safest
SET synchronous_commit = off;   -- Faster but risky (data loss on crash)
```

---

### Q5. What is SAVEPOINT and when would you use it?

**Interview Answer**

SAVEPOINT creates a named marker within a transaction that you can roll back to without undoing the entire transaction. This is useful when a transaction performs multiple operations and some may fail while others should succeed — for example, batch inserts where some rows have constraint violations. Using SAVEPOINT, you can roll back to the savepoint, skip the failed operation, and continue the transaction. After ROLLBACK TO SAVEPOINT, all locks acquired after the savepoint are released. SAVEPOINTs are also used by ORMs and connection pools to implement retry logic without losing the entire transaction context.

```sql
-- SAVEPOINT for batch operations
BEGIN;
INSERT INTO products (name, price) VALUES ('Widget', 9.99);
SAVEPOINT batch_start;

INSERT INTO products (name, price) VALUES ('Gadget', 19.99);
-- This fails: duplicate key
INSERT INTO products (name, price) VALUES ('Widget', 9.99);

-- Roll back only the failed part
ROLLBACK TO SAVEPOINT batch_start;

-- Continue with other products
INSERT INTO products (name, price) VALUES ('Doohickey', 29.99);
COMMIT;
-- Widget and Doohickey are saved, Gadget is lost
```

---

### Q6. How does SQLx manage transactions in Rust?

**Interview Answer**

SQLx provides `pool.begin()` to start a transaction, returning a `Transaction` object that borrows the connection from the pool. You execute queries against `&mut *tx` (dereferencing the transaction). When done, you call `tx.commit()` or `tx.rollback()` — if neither is called before the Transaction is dropped, it automatically rolls back (RAII pattern). SQLx's Transaction type is generic over the database driver (Postgres, MySQL, etc.). The connection is returned to the pool when the transaction ends. For nested transactions, SQLx uses SAVEPOINTs internally. Always handle errors explicitly and ensure rollback happens on failure.

```rust
use sqlx::{PgPool, Transaction, Postgres};

async fn create_order(pool: &PgPool, order: &NewOrder) -> Result<Order, sqlx::Error> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    let order = sqlx::query_as!(Order,
        "INSERT INTO orders (customer_id, total) VALUES ($1, $2) RETURNING *",
        order.customer_id, order.total
    )
    .fetch_one(&mut *tx)
    .await?;

    for item in &order.items {
        sqlx::query!(
            "INSERT INTO line_items (order_id, product_id, quantity) VALUES ($1, $2, $3)",
            order.id, item.product_id, item.quantity
        )
        .execute(&mut *tx)
        .await?;
    }

    // If we reach here, commit. On any error, tx drops and rolls back.
    tx.commit().await?;
    Ok(order)
}

// Explicit rollback on error
async fn safe_transfer(pool: &PgPool, from: i64, to: i64, amount: rust_decimal::Decimal) -> Result<()> {
    let mut tx = pool.begin().await?;
    if let Err(e) = transfer_inner(&mut tx, from, to, amount).await {
        tx.rollback().await?;
        return Err(e);
    }
    tx.commit().await?;
    Ok(())
}
```

---

### Q7. What is the difference between explicit and implicit transactions in PostgreSQL?

**Interview Answer**

An explicit transaction is started with `BEGIN` and ended with `COMMIT` or `ROLLBACK`. An implicit transaction is when a single statement runs outside an explicit transaction block — PostgreSQL wraps each statement in an implicit transaction that auto-commits on success and auto-rolls-back on failure. In PostgreSQL's protocol, the autocommit mode is the default if no BEGIN is issued. This means every single INSERT, UPDATE, or DELETE runs in its own implicit transaction. The performance implication is significant: batching multiple statements in an explicit transaction avoids repeated WAL flushes (fsync) and snapshot allocations, making it 10-100x faster for bulk operations.

```sql
-- Implicit transaction (autocommit)
INSERT INTO users (name) VALUES ('Alice');  -- Auto-committed
INSERT INTO users (name) VALUES ('Bob');    -- Separate transaction

-- Explicit transaction (much faster for batch)
BEGIN;
INSERT INTO users (name) VALUES ('Alice');
INSERT INTO users (name) VALUES ('Bob');
INSERT INTO users (name) VALUES ('Charlie');
COMMIT;
-- Single WAL flush, single snapshot, much faster

-- Check autocommit setting
SHOW autocommit;  -- on by default in psql

-- For SQLx/Rust, always use explicit transactions for multi-statement operations
```

---

### Q8. What are advisory locks and how do they differ from regular locks?

**Interview Answer**

Advisory locks are application-level locks managed by PostgreSQL that don't correspond to any table or row data. They are used to coordinate access to external resources or implement distributed locking patterns. PostgreSQL provides two types: session-level advisory locks (held until explicitly released or session ends) and transaction-level advisory locks (released at COMMIT/ROLLBACK). Advisory locks use bigint keys, allowing you to lock on arbitrary identifiers (like user IDs or resource names). They don't interact with MVCC and are invisible to normal lock monitoring. In Rust backends, advisory locks are useful for distributed task scheduling — ensuring only one worker processes a given job.

```sql
-- Session-level advisory lock (held until released)
SELECT pg_advisory_lock(12345);  -- Lock resource 12345
-- ... do work ...
SELECT pg_advisory_unlock(12345);  -- Release

-- Try to acquire without blocking
SELECT pg_try_advisory_lock(12345);  -- Returns true/false

-- Transaction-level advisory lock (auto-released on COMMIT/ROLLBACK)
BEGIN;
SELECT pg_advisory_xact_lock(67890);
-- Lock held for duration of transaction
COMMIT;  -- Lock automatically released

-- Lock a job for processing
SELECT pg_advisory_lock(hashtext('job_queue'));
-- Process job...
SELECT pg_advisory_unlock(hashtext('job_queue'));
```

---

### Q9. What is the two-phase commit protocol and does PostgreSQL support it?

**Interview Answer**

Two-phase commit (2PC) is a protocol for distributed transactions across multiple database systems. Phase one (PREPARE) — the coordinator asks all participants to prepare and vote; each participant writes its changes to WAL but doesn't commit. Phase two (COMMIT/ABORT) — if all participants vote yes, the coordinator sends COMMIT; otherwise it sends ABORT. PostgreSQL supports 2PC natively via `PREPARE TRANSACTION` and `COMMIT PREPARED`. This is essential when a single logical transaction spans PostgreSQL and another database (like a message queue). SQLx supports 2PC through explicit PREPARE TRANSACTION commands within a transaction.

```sql
-- Two-phase commit in PostgreSQL
BEGIN;
INSERT INTO orders (total) VALUES (100.00);
PREPARE TRANSACTION 'txn_order_123';
-- Transaction is now prepared, changes written but not visible

-- Later, from coordinator or same session:
COMMIT PREPARED 'txn_order_123';
-- Transaction is now committed

-- Or abort:
ROLLBACK PREPARED 'txn_order_123';

-- List prepared transactions
SELECT * FROM pg_prepared_xacts;
```

---

### Q10. What is write-ahead logging (WAL) and why is it essential for transactions?

**Interview Answer**

WAL is PostgreSQL's mechanism for ensuring durability and enabling crash recovery. Before any change is applied to a data page, the change record is first written to the WAL log on disk. This means that even if the server crashes mid-transaction, the WAL contains enough information to reconstruct (redo) committed changes and undo uncommitted ones. WAL also enables replication — standby servers replay the WAL stream to stay synchronized. The `wal_level` parameter controls how much information is recorded (minimal, replica, logical). WAL files are 16MB segments stored in `pg_wal/` and are recycled after being archived or replayed. The `synchronous_commit` setting controls whether WAL is flushed to disk before acknowledging each commit.

```sql
-- WAL configuration
SHOW wal_level;            -- replica (default), minimal, logical
SHOW max_wal_size;         -- 1GB default
SHOW min_wal_size;         -- 80MB default
SHOW synchronous_commit;   -- on (default)

-- Check WAL activity
SELECT * FROM pg_stat_bgwriter;
SELECT pg_current_wal_lsn();  -- Current WAL position

-- WAL usage for replication
SELECT slot_name, restart_lsn, confirmed_flush_lsn
FROM pg_replication_slots;

-- For high-throughput systems, batch WAL writes
SET synchronous_commit = off;  -- Risk: data loss on crash
-- Better: use async replication with synchronous_commit = remote_apply
```
