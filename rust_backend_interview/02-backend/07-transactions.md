# Transactions

## Interview Question

Transactions.

## Interview Answer

"Transactions ensure ACID properties so either all operations succeed or all are rolled back."

---

## Follow-up Questions & Answers

### Q1. What are the four ACID properties and why do they matter?

**Interview Answer**

Atomicity means all operations in a transaction complete or none do. Consistency ensures the database moves from one valid state to another. Isolation guarantees concurrent transactions don't interfere with each other. Durability means committed data survives crashes. Together they guarantee data integrity even under failures and concurrent access, which is critical for financial or inventory operations.

---

### Q2. How do you implement transactions with sqlx in Rust?

**Interview Answer**

I call `pool.begin()` to start a transaction, which returns a `Transaction` object. I execute queries on this transaction using `.execute()` and `.query_as()`. If everything succeeds I call `tx.commit()`, and if any error occurs I return early and the transaction is automatically rolled back when the `Transaction` object is dropped. The `?` operator makes this pattern clean and ergonomic.

---

### Q3. What is a deadlock in the context of database transactions and how do you prevent it?

**Interview Answer**

A deadlock happens when two transactions hold locks that the other needs, so both wait indefinitely. PostgreSQL detects deadlocks and rolls back one transaction automatically, but this hurts performance. I prevent deadlocks by always acquiring locks in the same order, keeping transactions short, and avoiding user interaction inside a transaction.

---

### Q4. What are isolation levels and which one do you typically use?

**Interview Answer**

PostgreSQL supports Read Uncommitted, Read Committed, Repeatable Read, and Serializable. I default to Read Committed for most operations because it balances safety and performance. I use Serializable for financial transfers or operations where any concurrent modification could cause inconsistency, even though it may reduce throughput due to serialization conflicts.

---

### Q5. When would you avoid using transactions?

**Interview Answer**

I avoid transactions for read-only analytics queries or bulk inserts where eventual consistency is acceptable. Transactions add overhead from lock management and WAL writes, so using them unnecessarily reduces throughput. For operations like logging or caching updates where partial failure is tolerable, skipping transactions keeps the system fast.

---

### Q6. How do you handle long-running transactions in a backend API?

**Interview Answer**

Long-running transactions hold database locks and consume resources, so I break them into smaller transactions where possible. If a long transaction is unavoidable, I use optimistic concurrency control with version columns instead of pessimistic locks. The API returns a conflict error if the data changed, and the client retries the operation.

---

### Q7. What is the difference between optimistic and pessimistic concurrency control?

**Interview Answer**

Pessimistic control uses database locks to prevent concurrent modification, which can cause contention and deadlocks. Optimistic control lets transactions proceed freely and checks for conflicts at commit time using a version column or timestamp. I prefer optimistic control for web APIs because it scales better without holding locks during HTTP request processing.

---

### Q8. How does Rust's type system help with transaction safety?

**Interview Answer**

The `Transaction` type in sqlx borrows the connection pool, so you cannot use the same pool connection for another query while a transaction is active. This prevents accidentally mixing transactional and non-transactional queries at compile time. The lifetime system ensures the transaction cannot outlive the connection, catching resource management bugs before runtime.

---
