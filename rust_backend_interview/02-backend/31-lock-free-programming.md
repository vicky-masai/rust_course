# lock-free programming

## Interview Question

Explain lock-free programming.

## Interview Answer

"Lock-free algorithms use atomic operations such as Compare-And-Swap (CAS) instead of mutexes, reducing contention and improving scalability under high concurrency."

---

## Follow-up Questions & Answers

### Q1. How does Compare-And-Swap work in Rust?

**Interview Answer**

CAS atomically compares a memory location's current value with an expected value and replaces it with a new value if they match. In Rust, use `AtomicUsize::compare_exchange()` from `std::sync::atomic`. This forms the building block for lock-free data structures like concurrent queues and counters.

---

### Q2. What is the difference between lock-free and wait-free algorithms?

**Interview Answer**

Lock-free algorithms guarantee system-wide progress but individual threads may starve. Wait-free algorithms guarantee every thread completes in bounded steps, which is harder to achieve. Most lock-free implementations in Rust, like `crossbeam::queue`, are lock-free but not wait-free.

---

### Q3. How do you implement a lock-free counter in Rust?

**Interview Answer**

Use `AtomicUsize` with `fetch_add()` for concurrent increments without locks. For complex operations, use `compare_exchange()` in a loop to retry on contention. In Axum handlers, atomic counters work well for metrics and request counting without mutex overhead.

---

### Q4. What are the ABA problems in lock-free programming?

**Interview Answer**

ABA occurs when a value changes from A to B and back to A, making CAS succeed incorrectly. This can cause subtle bugs in lock-free data structures. In Rust, use version counters or `arc-swap` crate to detect ABA conditions in concurrent code.

---

### Q5. When should you use lock-free structures over mutexes in Axum?

**Interview Answer**

Use lock-free structures for high-contention shared state like counters, caches, or queues where mutexes would become bottlenecks. `DashMap` and `crossbeam::queue` provide lock-free alternatives for common patterns. Mutexes are simpler and preferred when contention is low or operations are complex.

---

### Q6. How does `std::sync::atomic` relate to lock-free programming?

**Interview Answer**

`std::sync::atomic` provides primitive atomic types like `AtomicBool`, `AtomicUsize`, and `AtomicPtr` with CAS operations. These are the foundation for building lock-free data structures in Rust. Use `Ordering::SeqCst` for strict ordering or weaker orderings like `Relaxed` for performance when safe.

---

### Q7. What are the memory ordering implications of lock-free operations?

**Interview Answer**

Memory ordering controls how operations on different memory locations are seen across threads. `SeqCst` provides the strongest ordering but has performance overhead. Use `Acquire`/`Release` for synchronization points and `Relaxed` for independent counters. Incorrect ordering can cause subtle concurrency bugs.

---

### Q8. Can you use lock-free programming with Tokio?

**Interview Answer**

Yes. Tokio's runtime uses lock-free structures internally for task scheduling. In Axum handlers, use `tokio::sync::watch` or `tokio::sync::broadcast` for lock-free channel communication. For shared state, `Arc<AtomicUsize>` works alongside Tokio without blocking the async runtime.
