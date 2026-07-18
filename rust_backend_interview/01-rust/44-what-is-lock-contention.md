# What is lock contention?

## Interview Question

What is lock contention?

## Interview Answer

"Lock contention occurs when many threads compete for the same lock, reducing throughput and increasing latency."

---

## Follow-up Questions & Answers

### Q1. How do you measure lock contention in Rust?

**Interview Answer**

Use `std::time::Instant` around critical sections and track hold times with `parking_lot`'s `Mutex::name()` and profiling tools like `perf` or `tracing`. The `metrics` crate can record lock acquisition latency histograms. High contention shows up as threads spending most of their time waiting rather than executing.

---

### Q2. What is lock-free programming and how does Rust support it?

**Interview Answer**

Lock-free algorithms use atomic operations instead of mutexes to ensure progress. Rust provides `std::sync::atomic` types with configurable memory ordering (`Relaxed`, `Acquire`, `Release`, `SeqCst`). The `crossbeam` crate offers lock-free queues and stacks. Lock-free code is harder to get right but avoids priority inversion and convoy effects.

---

### Q3. What is the difference between `Mutex` and `RwLock` for reducing contention?

**Interview Answer**

`Mutex` allows one holder at a time, while `RwLock` allows multiple concurrent readers. If your workload is read-heavy (like a configuration cache), `RwLock` reduces contention by allowing parallel reads. However, `RwLock` has higher per-acquisition overhead, so it's slower than `Mutex` under write-heavy workloads.

---

### Q4. What is a spin lock and when is it appropriate?

**Interview Answer**

A spin lock busy-waits instead of yielding the CPU, which is faster when the lock is held for very short durations (nanoseconds). In Rust, you can implement one with `AtomicBool::compare_exchange`. It's appropriate for low-contention scenarios where thread sleep/wake overhead exceeds the spin cost, but it wastes CPU under high contention.

---

### Q5. How does `parking_lot` improve over `std::sync::Mutex`?

**Interview Answer**

`parking_lot` uses a more efficient parking mechanism with smaller mutexes (1 byte vs 40 bytes), avoids poisoning, and provides `try_lock` with better performance. It uses the OS parking API directly rather than `pthread_mutex`, reducing syscall overhead. Benchmarks show 2-3x improvement in uncontended cases and better behavior under contention.

---

### Q6. What is a mutex convoy and how do you avoid it?

**Interview Answer**

A mutex convoy occurs when a thread holding a lock is preempted, causing all other threads to queue up and wait. This is common with `std::sync::Mutex` on oversubscribed systems. Avoid it by minimizing lock hold time, using `parking_lot` for better scheduling, or restructuring to use lock-free data structures where possible.

---

### Q7. How do you reduce contention in a database connection pool?

**Interview Answer**

Use sharding to partition the pool so different threads use different sub-pools. Implement connection lifetimes to prevent long-held connections. Use `tokio::sync::Semaphore` instead of a `Mutex` for pool management, since semaphores are optimized for async wakeup. Monitor pool metrics to detect saturation early.

---

### Q8. What is read-copy-update (RCU) and can you use it in Rust?

**Interview Answer**

RCU allows lock-free reads by publishing new versions of data and deferring reclamation of old versions until no readers reference them. In Rust, `arc-swap` crate provides RCU-like semantics for `Arc<T>`: readers get a snapshot, writers publish a new `Arc`, and the old data is reclaimed when all readers drop. This is ideal for read-heavy configuration data.

---

### Q9. How does thread priority affect lock contention?

**Interview Answer**

Higher-priority threads can preempt lower-priority threads holding locks, causing priority inversion. Rust doesn't expose thread priority directly, but you can use `std::thread::Builder` with platform-specific extensions. Real-time systems use priority inheritance protocols in the mutex implementation to mitigate this.

---

### Q10. When should you choose a `Mutex` over lock-free alternatives?

**Interview Answer**

Use `Mutex` when the critical section is complex, the data structure has invariants that are hard to maintain atomically, or simplicity matters more than peak performance. Lock-free structures are harder to debug and test. For most backend services, a well-tuned `Mutex` with short hold times outperforms a poorly implemented lock-free alternative.

---
