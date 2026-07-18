# What memory ordering would you use for atomics?

## Interview Question

What memory ordering would you use for atomics?

## Interview Answer

"The strongest ordering is `SeqCst`. Weaker orderings like `Acquire`, `Release`, and `Relaxed` improve performance when stronger guarantees are unnecessary."

---

## Follow-up Questions & Answers

### Q1. What is the difference between `Acquire` and `Release`?

**Interview Answer**

`Acquire` prevents subsequent loads/stores from being reordered before it. `Release` prevents preceding loads/stores from being reordered after it. Together, they form a happens-before relationship: a `Release` store by one thread is visible to an `Acquire` load by another thread. This is the foundation of lock-free communication patterns.

---

### Q2. When should you use `Relaxed` ordering?

**Interview Answer**

Use `Relaxed` when you don't need inter-thread synchronization—only atomicity. Examples include sequence counters, statistics counters, and random number generators where the exact value seen by other threads doesn't matter. `Relaxed` is the fastest ordering because it doesn't impose memory fences, allowing maximum hardware optimization.

---

### Q3. What is `SeqCst` and why is it the default?

**Interview Answer**

`SeqCst` provides a total global order of all `SeqCst` operations across all threads. It's the strongest guarantee and easiest to reason about. It's the default in Rust and many languages because it prevents subtle reorderings that weaker orderings allow. However, it can be slower on weakly-ordered architectures like ARM.

---

### Q4. How do memory orderings affect performance on x86?

**Interview Answer**

x86 has a strong memory model, so `Acquire`/`Release` and `SeqCst` have similar performance—often just a compiler fence with no hardware fence. `Relaxed` can be slightly faster due to additional compiler optimizations. On ARM, the difference is more significant because ARM allows more reordering by default.

---

### Q5. What is the `Ordering` enum in Rust's standard library?

**Interview Answer**

`std::sync::atomic::Ordering` has variants: `Relaxed`, `Acquire`, `Release`, `AcqRel`, and `SeqCst`. Each specifies the memory ordering guarantee for atomic operations. `AcqRel` combines `Acquire` and `Release` for read-modify-write operations like `fetch_add`. Choosing the right variant balances correctness and performance.

---

### Q6. How do you choose between `AcqRel` and `SeqCst`?

**Interview Answer**

Use `AcqRel` when you need both acquire and release semantics for a single operation (like `compare_exchange`), but don't need a total global order. Use `SeqCst` when the correctness of your algorithm depends on all threads seeing the same order of operations. Most lock-free algorithms work with `AcqRel`, but `SeqCst` is safer when unsure.

---

### Q7. Can `Relaxed` ordering cause data races?

**Interview Answer**

No, `Relaxed` still guarantees atomicity—no torn reads or writes. However, it doesn't guarantee ordering between different atomic variables. If thread A writes `x=1` then `y=2` with `Relaxed`, thread B might see `y=2` but still read the old `x`. Use `Release`/`Acquire` to establish ordering between variables.

---

### Q8. What is a store buffer and how does it affect ordering?

**Interview Answer**

Store buffers allow CPUs to defer writes to main memory, improving performance. On x86, stores become visible in program order. On ARM/POWER, stores can be reordered. `Release` ordering ensures the store buffer is flushed before subsequent operations. `Acquire` ensures the load isn't reordered with later operations by preventing the compiler from moving instructions across the fence.

---

### Q9. How do atomics interact with the Rust memory model?

**Interview Answer**

Rust's memory model is based on the C++ memory model for atomics. It defines happens-before relationships and data races. The `Send` and `Sync` traits enforce that only properly synchronized code can share data across threads. Unsafe code that violates these contracts causes undefined behavior, which atomics are designed to prevent.

---

### Q10. What is a real-world example of choosing memory ordering?

**Interview Answer**

For a lock-free counter: `Relaxed` is fine if you only care about approximate counts. For a flag that signals data is ready: `Release` on store, `Acquire` on load. For a spinlock: `AcqRel` on `compare_exchange` to ensure mutual exclusion. For a global sequence number: `SeqCst` if all threads must observe the same order.

---
