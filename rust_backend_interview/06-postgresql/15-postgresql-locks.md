# PostgreSQL Locks

## Interview Question

What types of locks does PostgreSQL use and how do they work?

## Interview Answer

PostgreSQL uses several lock types to ensure data consistency. Table-level locks range from ACCESS SHARE (weakest, taken by SELECT) to ACCESS EXCLUSIVE (strongest, taken by DROP TABLE, ALTER TABLE). Row-level locks include FOR UPDATE (exclusive), FOR NO KEY UPDATE, FOR SHARE, and FOR KEY SHARE. Advisory locks are application-level locks for coordinating external resources. Locks are managed by PostgreSQL's lock manager in shared memory and are automatically released at transaction end. The lock hierarchy ensures compatibility: weaker locks can coexist, but stronger locks block weaker ones. Most application developers use row-level locks (SELECT FOR UPDATE) and should avoid table-level locks.

---

## Follow-up Questions & Answers

### Q1. What is the difference between row locks and table locks?

**Interview Answer**

Row locks (FOR UPDATE, FOR SHARE) lock specific rows identified by the WHERE clause, allowing concurrent access to other rows. Table locks (LOCK TABLE) lock the entire table, preventing all concurrent modifications. Row locks are the default for UPDATE/DELETE operations and are granular — only conflicting rows block each other. Table locks are used for schema changes (ALTER TABLE) or explicit locking. Row locks are stored per-tuple in the row header (xmax field), while table locks are stored in the lock table (shared memory). Always prefer row-level locks for application logic.

```sql
-- Row lock: only locks matching rows
BEGIN;
SELECT * FROM accounts WHERE id = 1 FOR UPDATE;
-- Other transactions can still modify accounts where id = 2
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
COMMIT;

-- Table lock: locks everything
BEGIN;
LOCK TABLE accounts IN EXCLUSIVE MODE;
-- No other transaction can read or write accounts
-- Used for bulk operations or schema changes
COMMIT;

-- Check current locks
SELECT locktype, relation::regclass, mode, granted
FROM pg_locks WHERE pid = pg_backend_pid();
```

---

### Q2. What are the different row lock modes and when do you use them?

**Interview Answer**

PostgreSQL has four row lock modes: FOR UPDATE (strongest, blocks all other row locks, used for read-then-write), FOR NO KEY UPDATE (lighter than FOR UPDATE, doesn't block FK checks), FOR SHARE (shared lock, multiple transactions can hold it, blocks FOR UPDATE), and FOR KEY SHARE (weakest, only blocks updates to foreign key columns). The choice depends on what you're protecting: FOR UPDATE for modifying rows, FOR SHARE for read-only locking, FOR KEY SHARE for FK integrity. Lighter locks allow more concurrency.

```sql
-- FOR UPDATE: exclusive row lock
BEGIN;
SELECT * FROM accounts WHERE id = 1 FOR UPDATE;
-- Blocks other FOR UPDATE and FOR SHARE on same row
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
COMMIT;

-- FOR SHARE: shared row lock
BEGIN;
SELECT * FROM accounts WHERE id = 1 FOR SHARE;
-- Other FOR SHARE can coexist, but FOR UPDATE blocks
-- Used for consistent reads
COMMIT;

-- FOR KEY SHARE: lightest lock (FK checking)
BEGIN;
SELECT * FROM parent_table WHERE id = 1 FOR KEY SHARE;
-- Only blocks UPDATE on the locked row's primary key
-- Used automatically for FK references
COMMIT;

-- NOWAIT: fail immediately instead of blocking
SELECT * FROM accounts WHERE id = 1 FOR UPDATE NOWAIT;
-- ERROR if row is locked by another transaction
```

---

### Q3. What are advisory locks and how do they work?

**Interview Answer**

Advisory locks are application-level locks managed by PostgreSQL that don't correspond to any table or row. They use bigint or two-int32 keys and are useful for distributed locking patterns. Two types: session-level (held until explicit release or session end) and transaction-level (released at COMMIT/ROLLBACK). They bypass MVCC and don't interact with regular table/row locks. Use cases: job queue processing (ensuring one worker per job), preventing concurrent execution of maintenance tasks, and coordinating application-level resources.

```sql
-- Session-level advisory lock
SELECT pg_advisory_lock(hashtext('my_resource'));
-- Lock held until pg_advisory_unlock or session ends

-- Try without blocking
SELECT pg_try_advisory_lock(12345);
-- Returns true if acquired, false if already held

-- Transaction-level advisory lock
BEGIN;
SELECT pg_advisory_xact_lock(67890);
-- Auto-released at COMMIT or ROLLBACK
COMMIT;

-- Release session lock
SELECT pg_advisory_unlock(hashtext('my_resource'));

-- Check for lock conflicts
SELECT * FROM pg_locks WHERE locktype = 'advisory';
```

---

### Q4. What is SELECT FOR UPDATE and when should you use it?

**Interview Answer**

SELECT FOR UPDATE acquires an exclusive row lock immediately at read time, rather than waiting until an UPDATE. Use it for read-then-modify patterns where you need to prevent other transactions from modifying the row between your read and write. It ensures atomicity of the read-modify-write cycle. Without it, another transaction could modify the row between your SELECT and UPDATE (lost update anomaly). Alternatives: PostgreSQL's implicit row locks during UPDATE (locks at write time), or optimistic locking with a version column.

```sql
-- Without FOR UPDATE (vulnerable to lost update)
BEGIN;
SELECT balance FROM accounts WHERE id = 1;  -- 1000
-- Transaction B updates here: SET balance = 500
UPDATE accounts SET balance = 500 - 100 WHERE id = 1;  -- Overwrites B's change!
COMMIT;

-- With FOR UPDATE (safe)
BEGIN;
SELECT balance FROM accounts WHERE id = 1 FOR UPDATE;  -- 1000, locked
-- Transaction B blocks here until A commits
UPDATE accounts SET balance = balance - 100 WHERE id = 1;  -- 900
COMMIT;  -- B now proceeds with updated value

-- SKIP LOCKED for queue patterns
BEGIN;
SELECT * FROM job_queue
WHERE status = 'pending'
ORDER BY id LIMIT 1
FOR UPDATE SKIP LOCKED;
-- Skips rows locked by other workers
UPDATE job_queue SET status = 'processing' WHERE id = $1;
COMMIT;
```

---

### Q5. How do you detect and resolve lock contention?

**Interview Answer**

Monitor lock contention using `pg_locks` joined with `pg_stat_activity` to find blocking and blocked transactions. Key queries: find all blocking sessions, identify long-running locks, and measure wait times. Configure `lock_timeout` to prevent indefinite blocking. Use `log_lock_waits` to log all lock waits. Common causes: long-running transactions, missing indexes on foreign keys, and SELECT FOR UPDATE on heavily-contended rows. Resolution: kill blocking sessions, reduce transaction duration, add indexes, or use SKIP LOCKED for queue patterns.

```sql
-- Find blocking transactions
SELECT
    blocked.pid AS blocked_pid,
    blocked.query AS blocked_query,
    blocking.pid AS blocking_pid,
    blocking.query AS blocking_query,
    now() - blocked.query_start AS wait_duration
FROM pg_stat_activity blocked
JOIN pg_locks bl ON blocked.pid = bl.pid AND NOT bl.granted
JOIN pg_locks gl ON bl.relation = gl.relation AND gl.granted AND bl.pid != gl.pid
JOIN pg_stat_activity blocking ON gl.pid = blocking.pid;

-- Set lock timeout
SET lock_timeout = '5s';
-- Fails with error after 5 seconds instead of blocking forever

-- Enable lock wait logging
ALTER SYSTEM SET log_lock_waits = on;

-- Monitor lock statistics
SELECT datname, deadlocks, conflicts
FROM pg_stat_database WHERE datname = current_database();
```

---

### Q6. How does SQLx handle locks in Rust?

**Interview Answer**

SQLx executes lock-acquiring queries (SELECT FOR UPDATE, LOCK TABLE) through the standard query API. You write the SQL with lock clauses and SQLx handles the connection. For transaction-level locking, begin a transaction, execute SELECT FOR UPDATE, modify the row, and commit. SQLx's connection pool ensures locks are held on a single connection and released when the transaction ends. For advisory locks, use `sqlx::query!("SELECT pg_advisory_lock($1)", key)`.

```rust
use sqlx::{PgPool, Postgres, Transaction};

// SELECT FOR UPDATE in a transaction
async fn debit_account(
    pool: &PgPool,
    account_id: i64,
    amount: rust_decimal::Decimal,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    let row = sqlx::query!(
        "SELECT balance FROM accounts WHERE id = $1 FOR UPDATE",
        account_id
    )
    .fetch_one(&mut *tx)
    .await?;

    if row.balance < amount {
        tx.rollback().await?;
        return Err(anyhow!("Insufficient funds"));
    }

    sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
        amount, account_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

// Advisory lock for job processing
async fn process_job(pool: &PgPool, job_id: i64) -> Result<()> {
    let acquired = sqlx::query_scalar!(
        "SELECT pg_try_advisory_lock($1)", job_id
    )
    .fetch_one(pool)
    .await?;

    if !acquired {
        return Err(anyhow!("Job already being processed"));
    }

    // Process job...
    sqlx::query!("SELECT pg_advisory_unlock($1)", job_id)
        .execute(pool).await?;
    Ok(())
}
```

---

### Q7. What is predicate locking and how does it relate to SERIALIZABLE isolation?

**Interview Answer**

Predicate locks are logical locks used by SSI (Serializable Snapshot Isolation) to detect conflicts. Unlike row locks, they don't prevent modification — they track which ranges of data a transaction has read. When another transaction modifies data in that range, SSI detects the read-write dependency and may abort one transaction. Predicate locks are invisible in `pg_locks` and don't block other transactions. They're the mechanism that prevents write skew and other serialization anomalies without the overhead of exclusive locks.

```sql
-- Under SERIALIZABLE, PostgreSQL adds predicate locks automatically
BEGIN;
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;
SELECT * FROM doctors WHERE on_call = true;
-- Spredicate lock acquired on the range (on_call = true)
COMMIT;

-- Predicate locks don't appear in pg_locks
-- They're internal to SSI and don't block other transactions
-- They only cause serialization errors when conflicts are detected
```

---

### Q8. What are table-level lock modes in PostgreSQL?

**Interview Answer**

PostgreSQL has eight table-level lock modes in increasing strength: ACCESS SHARE (SELECT), ROW SHARE (SELECT FOR UPDATE/SHARE), ROW EXCLUSIVE (INSERT/UPDATE/DELETE), SHARE UPDATE EXCLUSIVE (VACUUM, ANALYZE, some ALTER TABLE), SHARE (CREATE INDEX), SHARE ROW EXCLUSIVE (some ALTER TABLE), EXCLUSIVE (some ALTER TABLE), and ACCESS EXCLUSIVE (DROP TABLE, TRUNCATE, some ALTER TABLE). Each mode blocks certain combinations of other modes. Most application code only encounters ACCESS SHARE and ROW EXCLUSIVE. Avoid explicit LOCK TABLE in application code — use row-level locks instead.

```sql
-- Check table-level locks
SELECT relation::regclass, mode, granted
FROM pg_locks
WHERE locktype = 'relation'
  AND relation = 'accounts'::regclass;

-- Explicit table lock (rarely needed)
BEGIN;
LOCK TABLE accounts IN SHARE MODE;
-- Other transactions can read but not write
-- Use for consistent reads without row-level locking
COMMIT;

-- See lock compatibility matrix
-- ACCESS SHARE blocks only ACCESS EXCLUSIVE
-- ROW EXCLUSIVE blocks SHARE and stronger
-- ACCESS EXCLUSIVE blocks everything
```

---

### Q9. How do foreign key constraints create locks?

**Interview Answer**

When you INSERT or UPDATE a row in a child table, PostgreSQL acquires a SHARE ROW EXCLUSIVE lock on the parent table to check the foreign key. Without an index on the foreign key column, this lock covers the entire parent table (table-level lock), blocking other writes. With an index, it locks only the specific referenced row. This is why missing foreign key indexes cause deadlocks and performance issues. The lock is acquired at statement level and released at transaction end. Always index foreign key columns in PostgreSQL.

```sql
-- Without FK index: locks entire parent table
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id)  -- No index!
);
INSERT INTO orders (user_id) VALUES (1);
-- Acquires SHARE ROW EXCLUSIVE on ALL of users table

-- With FK index: locks only the specific user row
CREATE INDEX idx_orders_user ON orders (user_id);
INSERT INTO orders (user_id) VALUES (1);
-- Acquires SHARE ROW EXCLUSIVE only on users row where id = 1

-- PostgreSQL warns about missing FK indexes:
-- WARNING: SPI_execute failed: relation "orders" does not have
-- index on column "user_id"
```

---

### Q10. What is the difference between pessimistic and optimistic locking?

**Interview Answer**

Pessimistic locking acquires a lock before modifying data (SELECT FOR UPDATE), preventing other transactions from accessing the row. It's safe but reduces concurrency because transactions block each other. Optimistic locking uses a version column — read the version, then UPDATE with a WHERE version = old_version. If the row was modified, the UPDATE affects 0 rows and you retry. Optimistic locking allows higher concurrency but requires retry logic. Use pessimistic for high-contention rows (account balances, inventory), optimistic for low-contention (profile updates, preferences).

```sql
-- Pessimistic locking (SELECT FOR UPDATE)
BEGIN;
SELECT * FROM accounts WHERE id = 1 FOR UPDATE;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
COMMIT;

-- Optimistic locking (version column)
ALTER TABLE accounts ADD COLUMN version INT DEFAULT 1;

-- Read
SELECT id, balance, version FROM accounts WHERE id = 1;
-- balance=1000, version=3

-- Update with version check
UPDATE accounts SET balance = 900, version = version + 1
WHERE id = 1 AND version = 3;
-- If 0 rows affected, someone else modified it — retry
```
