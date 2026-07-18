# What causes deadlocks?

## Interview Question

What causes deadlocks?

## Interview Answer

"Deadlocks occur when multiple threads acquire locks in different orders and wait indefinitely. Prevent them by maintaining a consistent lock acquisition order or reducing lock scope."

---

## Follow-up Questions & Answers

### Q1. What are the four conditions that must hold for a deadlock to occur?

**Interview Answer**

Mutual exclusion means a resource is held exclusively. Hold and wait means a thread holds one resource while waiting for another. No preemption means resources cannot be forcibly taken. Circular wait means a cycle exists in the resource dependency graph. All four must be present simultaneously, so breaking any one prevents the deadlock.

---

### Q2. How do deadlocks manifest differently in async code vs threaded code?

**Interview Answer**

In threaded code, deadlocks happen when `Mutex::lock()` blocks a thread waiting for another thread's lock. In async Rust with `tokio::sync::Mutex`, the task yields instead of blocking the thread, but the logical deadlock still occurs with tasks waiting on each other. Async deadlocks are harder to detect because other tasks continue running on the thread, masking the stuck tasks.

---

### Q3. How do you prevent deadlocks in a Rust backend with multiple database connections?

**Interview Answer**

I follow a strict rule of never acquiring a second database connection while holding the first. If I need data from two sources, I execute the queries sequentially and release the connection before acquiring the next. For transactions, I use a single connection from the pool and perform all operations through that one transaction handle.

---

### Q4. What is a lock-free alternative to mutexes in Rust?

**Interview Answer**

I use `tokio::sync::RwLock` for read-heavy workloads where multiple readers can access data concurrently and only writers need exclusive access. For simple atomic operations, I use `std::sync::atomic` types. Channels like `tokio::sync::mpsc` eliminate shared state entirely by passing data between tasks, which is often the cleanest approach in async Rust.

---

### Q5. How does Rust's type system help prevent deadlocks at compile time?

**Interview Answer**

Rust's ownership system prevents holding a reference to data while acquiring a lock on the same data, since the borrow checker detects conflicting borrows. The `MutexGuard` returned by `.lock()` holds the borrow, so you cannot pass the inner data to another function that acquires a different lock without first dropping the guard. This catches potential deadlocks during development rather than in production.

---

### Q6. What tools do you use to detect deadlocks in production?

**Interview Answer**

I use `tokio-console` to monitor task states and identify tasks that have been in `Waiting` state abnormally long. For thread-level deadlocks, I set timeout wrappers around lock acquisitions and log warnings when they exceed thresholds. In PostgreSQL, I query `pg_locks` and `pg_stat_activity` to identify locked transactions and terminate long-running ones with `pg_terminate_backend`.

---

### Q7. Can you describe a real-world deadlock scenario you've encountered?

**Interview Answer**

I once had a deadlock where Task A locked the user record and then tried to lock the order record, while Task B locked the order record and then tried to lock the user record. The fix was to establish a convention that user locks are always acquired before order locks. I also reduced lock scope by fetching the data in a single query instead of two separate locked reads.

---

### Q8. What is a livelock and how is it different from a deadlock?

**Interview Answer**

A livelock occurs when tasks keep changing state in response to each other but make no actual progress, like two people stepping aside for each other repeatedly. Unlike deadlock where tasks are blocked, livelock tasks are actively running but stuck in a cycle. In Rust, this can happen with retry loops that immediately fail and retry without backoff. I add exponential backoff and jitter to retry logic to break livelock patterns.

---
