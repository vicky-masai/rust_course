# PostgreSQL Isolation Levels

## Interview Question

What are the isolation levels in PostgreSQL and when would you use each one?

## Interview Answer

PostgreSQL supports four SQL-standard isolation levels: READ COMMITTED (default) — each statement gets a fresh snapshot, allowing non-repeatable reads but providing maximum concurrency; REPEATABLE READ — a single snapshot at transaction start prevents non-repeatable reads but may raise serialization errors on write conflicts; SERIALIZABLE — uses Serializable Snapshot Isolation (SSI) to detect and prevent all anomalies including phantoms, with occasional false-positive aborts; READ UNCOMMITTED — technically mapped to READ COMMITTED because PostgreSQL's MVCC doesn't allow dirty reads. In practice, READ COMMITTED handles most OLTP workloads, REPEATABLE READ is for read-heavy dashboards, and SERIALIZABLE is for critical financial or inventory operations where consistency is paramount.

---

## Follow-up Questions & Answers

### Q1. What is a dirty read and does PostgreSQL allow it?

**Interview Answer**

A dirty read occurs when a transaction reads data modified by another uncommitted transaction. If the modifying transaction rolls back, the reader has seen data that never existed. PostgreSQL does NOT allow dirty reads at any isolation level because its MVCC implementation always checks transaction commit status before showing row versions. Even READ UNCOMMITTED in PostgreSQL behaves like READ COMMITTED — this is explicitly stated in the PostgreSQL documentation. This is a significant difference from other databases like MySQL InnoDB, which does allow dirty reads under READ UNCOMMITTED. PostgreSQL's MVCC makes dirty reads impossible by design.

```sql
-- PostgreSQL prevents dirty reads at all levels
-- Transaction A
BEGIN;
SET transaction_isolation = 'read uncommitted';
UPDATE accounts SET balance = 0 WHERE id = 1;
-- Not committed yet

-- Transaction B (any isolation level)
BEGIN;
SELECT balance FROM accounts WHERE id = 1;
-- PostgreSQL blocks until A commits or rolls back
-- If A rolls back: B sees balance = 1000 (original)
-- If A commits: B sees balance = 0 (new)
-- B NEVER sees uncommitted data
```

---

### Q2. When should you use REPEATABLE READ over READ COMMITTED?

**Interview Answer**

Use REPEATABLE READ when you need consistent reads across multiple queries within a single transaction — for example, a report that queries multiple tables and needs the data to be consistent as of a single point in time. It prevents non-repeatable reads where the same query returns different results within a transaction. However, REPEATABLE READ can cause serialization errors when two transactions try to update the same rows, requiring retry logic in your application. Use READ COMMITTED (default) for most OLTP operations where each statement is independent and you don't need cross-statement consistency. The trade-off is: REPEATABLE READ gives consistency at the cost of more aborts; READ COMMITTED gives concurrency at the cost of potential inconsistencies.

```sql
-- READ COMMITTED: statement-level snapshot
BEGIN;
SELECT SUM(balance) FROM accounts WHERE type = 'savings';  -- $10,000
-- Another transaction deposits $1,000 into a savings account
SELECT SUM(balance) FROM accounts WHERE type = 'savings';  -- $11,000 (different!)

-- REPEATABLE READ: transaction-level snapshot
BEGIN;
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;
SELECT SUM(balance) FROM accounts WHERE type = 'savings';  -- $10,000
-- Another transaction deposits $1,000 into a savings account
SELECT SUM(balance) FROM accounts WHERE type = 'savings';  -- $10,000 (same!)
COMMIT;
```

---

### Q3. What causes serialization errors in REPEATABLE READ and how do you handle them?

**Interview Answer**

Serialization errors in REPEATABLE READ occur when a transaction tries to UPDATE a row that was modified by another committed transaction after the reader's snapshot was taken. PostgreSQL detects this as a conflict and raises "ERROR: could not serialize access due to concurrent update" (error code 40001). This typically happens with the pattern: read-then-write on the same row across concurrent transactions. To handle this, your application must catch the error, rollback the transaction, and retry with a fresh transaction. Exponential backoff with jitter prevents thundering herd. In Rust with SQLx, match on the database error code "40001" and implement retry logic.

```sql
-- Serialization error scenario
-- Transaction A
BEGIN;
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;
SELECT balance FROM accounts WHERE id = 1;  -- balance = 1000
-- Transaction B commits UPDATE ... SET balance = 500 WHERE id = 1
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
-- ERROR: could not serialize access due to concurrent update
ROLLBACK;

-- Retry with fresh snapshot
BEGIN;
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;
SELECT balance FROM accounts WHERE id = 1;  -- balance = 500 (B's committed value)
UPDATE accounts SET balance = balance - 100 WHERE id = 1;  -- Works now
COMMIT;
```

---

### Q4. What is write skew and how does SERIALIZABLE prevent it?

**Interview Answer**

Write skew is a concurrency anomaly where two transactions each read data that constrains the other's write, and both commits produce an inconsistent state. A classic example: two doctors check that at least one is on call, then each sets themselves off call — resulting in no doctors on call. REPEATABLE READ cannot prevent write skew because neither transaction modifies a row the other reads. SERIALIZABLE via SSI detects the read-write dependency between the two transactions and aborts one of them. SSI tracks "rw-locks" and looks for dangerous patterns (the "dangerous structure" in the SSI algorithm) that indicate potential serialization anomalies.

```sql
-- Write skew anomaly (REPEATABLE READ cannot prevent)
-- Transaction A
BEGIN;
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;
SELECT COUNT(*) FROM doctors WHERE on_call = true;  -- 2
UPDATE doctors SET on_call = false WHERE name = 'Dr. Smith';
COMMIT;  -- Success

-- Transaction B (concurrent)
BEGIN;
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;
SELECT COUNT(*) FROM doctors WHERE on_call = true;  -- 2 (sees A's snapshot before update)
UPDATE doctors SET on_call = false WHERE name = 'Dr. Jones';
COMMIT;  -- Success — but now 0 doctors on call!

-- SERIALIZABLE prevents this
BEGIN;
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;
SELECT COUNT(*) FROM doctors WHERE on_call = true;  -- 2
UPDATE doctors SET on_call = false WHERE name = 'Dr. Smith';
COMMIT;  -- One of A or B gets ERROR: could not serialize access
```

---

### Q5. How does PostgreSQL detect serialization anomalies using SSI?

**Interview Answer**

SSI (Serializable Snapshot Isolation) tracks dependencies between transactions using predicate locks and rw-dependencies. When a transaction reads data, SSI records which other transactions hold write locks on that data. Before committing, SSI checks for "dangerous structures" — specifically, if Transaction A reads data written by Transaction B, and Transaction B reads data written by Transaction A, creating a cycle. PostgreSQL maintains this dependency graph in shared memory (the `pg_locks` and `pg_prewarm` catalogs expose some of this). If a cycle is detected, one transaction is aborted with a serialization error. SSI is lightweight because it only tracks read-write conflicts, not read-read or write-write conflicts. The false-positive rate is typically low (<1% of serializable transactions).

```sql
-- SSI tracking dependencies
-- Transaction A
BEGIN;
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;
SELECT * FROM inventory WHERE product_id = 42;  -- SSI records read
-- Transaction B modifies inventory for product_id = 42 and commits
UPDATE inventory SET quantity = quantity - 1 WHERE product_id = 42;
COMMIT;  -- SSI detects dependency cycle, may abort

-- Monitor serialization conflicts
SHOW stats_serializable;  -- May not exist in all versions

-- Check for serialization errors in logs
-- Look for: "ERROR: could not serialize access due to read/write dependencies"
-- The log will show which transactions conflicted
```

---

### Q6. What is the difference between predicate locks and row locks in PostgreSQL?

**Interview Answer**

Row locks (SELECT FOR UPDATE, SELECT FOR SHARE) lock specific rows, preventing other transactions from modifying them until the lock is released. They are physical locks that show up in `pg_locks`. Predicate locks, used by SSI under SERIALIZABLE isolation, are logical locks that don't prevent modification — they simply track which data a transaction has read so SSI can detect conflicts. Predicate locks don't block other transactions and are much lighter than row locks. The key difference is intent: row locks enforce mutual exclusion, predicate locks enable conflict detection. Under REPEATABLE READ and READ COMMITTED, PostgreSQL uses row locks but not predicate locks. Under SERIALIZABLE, predicate locks are added automatically for range scans and index lookups.

```sql
-- Row lock: blocks other transactions
BEGIN;
SELECT * FROM inventory WHERE product_id = 42 FOR UPDATE;
-- Other transactions trying to UPDATE this row will block
UPDATE inventory SET quantity = quantity - 1 WHERE product_id = 42;
COMMIT;

-- Predicate lock (SERIALIZABLE): tracks read, doesn't block
BEGIN;
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;
SELECT * FROM inventory WHERE product_id = 42;
-- SSI records: "this transaction read inventory for product_id = 42"
-- If another transaction modifies this row and commits, SSI detects conflict
COMMIT;

-- Check locks
SELECT locktype, relation::regclass, mode, granted
FROM pg_locks WHERE pid = pg_backend_pid();
```

---

### Q7. How do you choose the right isolation level for a Rust web application?

**Interview Answer**

Start with READ COMMITTED (PostgreSQL default) for most operations — it provides the best concurrency and simplest error handling. Use REPEATABLE READ for read-heavy operations that need point-in-time consistency, like financial reports or analytics queries that aggregate across multiple tables. Use SERIALIZABLE only for critical operations where correctness is more important than throughput, like inventory management, ticket booking, or financial transfers. Always implement retry logic with exponential backoff for REPEATABLE READ and SERIALIZABLE, since serialization errors are expected. In Rust, use sqlx's error handling to detect error code "40001" and retry. Set `idle_in_transaction_session_timeout` to prevent long transactions from holding back VACUUM.

```rust
use sqlx::{PgPool, Postgres, Transaction};

// Default: READ COMMITTED (implicit)
async fn create_user(pool: &PgPool, name: &str) -> Result<User> {
    sqlx::query_as!(User, "INSERT INTO users (name) VALUES ($1) RETURNING *", name)
        .fetch_one(pool).await
}

// REPEATABLE READ for consistent reads
async fn generate_report(pool: &PgPool) -> Result<Report> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *tx).await?;

    let revenue = sqlx::query_scalar!(r#"SELECT SUM(total) FROM orders"#)
        .fetch_one(&mut *tx).await?;
    let customers = sqlx::query_scalar!(r#"SELECT COUNT(DISTINCT customer_id) FROM orders"#)
        .fetch_one(&mut *tx).await?;

    tx.commit().await?;
    Ok(Report { revenue, customers })
}

// SERIALIZABLE with retry for critical operations
async fn book_ticket(pool: &PgPool, show_id: i64, user_id: i64) -> Result<()> {
    for attempt in 0..5 {
        let mut tx = pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut *tx).await?;

        let available = sqlx::query_scalar!(
            "SELECT quantity FROM shows WHERE id = $1", show_id
        ).fetch_one(&mut *tx).await?;

        if available <= 0 {
            tx.rollback().await?;
            return Err(anyhow!("No tickets available"));
        }

        sqlx::query!("UPDATE shows SET quantity = quantity - 1 WHERE id = $1", show_id)
            .execute(&mut *tx).await?;

        match tx.commit().await {
            Ok(_) => return Ok(()),
            Err(e) if is_serialization_error(&e) => {
                tokio::time::sleep(Duration::from_millis(10 * 2u64.pow(attempt))).await;
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Err(anyhow!("Failed after retries"))
}
```

---

### Q8. What is READ COMMITTED mode and why is it PostgreSQL's default?

**Interview Answer**

READ COMMITTED is PostgreSQL's default because it provides the best balance of concurrency and simplicity for typical OLTP workloads. Each statement sees only data committed before that statement began — this means a single transaction can see different data across multiple statements (non-repeatable reads). This behavior matches what most applications expect: each query is independent and sees the most recent committed data. There are no serialization errors to handle, no phantom reads to worry about, and write conflicts are resolved by blocking (not aborting). For applications where each HTTP request maps to one or a few SQL statements, READ COMMITTED is ideal because it maximizes throughput and minimizes error handling complexity.

```sql
-- READ COMMITTED behavior
BEGIN;
-- Statement 1: sees data committed before this statement
SELECT balance FROM accounts WHERE id = 1;  -- $1000

-- Another transaction commits a change here

-- Statement 2: sees NEW snapshot (includes the other transaction's commit)
SELECT balance FROM accounts WHERE id = 1;  -- $500 (different!)

-- But the UPDATE uses the same snapshot as the SELECT
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
-- Uses current committed value, not the value from SELECT

-- This is the default
SHOW transaction_isolation;  -- read committed
```
