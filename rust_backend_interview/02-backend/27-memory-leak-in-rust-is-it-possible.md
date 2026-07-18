# Memory leak in Rust—is it possible?

## Interview Question

Memory leak in Rust—is it possible?

## Interview Answer

"Yes. For example, reference cycles created with `Rc<RefCell<T>>` can leak memory because reference counts never reach zero."

---

## Follow-up Questions & Answers

### Q1. How does Rust's ownership system prevent most memory leaks?

**Interview Answer**

Rust's ownership rules ensure memory is freed deterministically when the owner goes out of scope, preventing accidental leaks from forgotten deallocation. The borrow checker enforces single ownership, eliminating use-after-free and double-free bugs. However, leaks can still occur through reference cycles, leaked allocations, or forgotten cleanup logic.

---

### Q2. What is a reference cycle and how does it cause leaks in Rust?

**Interview Answer**

A reference cycle occurs when two objects hold `Rc` or `Arc` references to each other, preventing their reference counts from reaching zero. For example, a parent pointing to a child and the child pointing back to the parent. `Weak` references break cycles by not incrementing the reference count.

---

### Q3. How do you detect memory leaks in a Rust application?

**Interview Answer**

Use `valgrind` or `dhat-rs` to profile memory allocations and identify unreleased memory. The `std::mem::forget` function intentionally leaks memory, which is useful for testing leak detection tools. In production, monitor RSS memory usage with Prometheus metrics and set alerts for steady memory growth.

---

### Q4. Can `Arc<Mutex<T>>` cause memory leaks?

**Interview Answer**

Yes, if `Arc` references form cycles, memory leaks occur because `Arc` uses reference counting like `Rc`. Additionally, if a mutex is poisoned or a thread panics while holding a lock, the data inside the `Mutex` may not be properly cleaned up. Use `Weak` references to break cycles in concurrent data structures.

---

### Q5. What is the difference between memory leaks and memory unsafety in Rust?

**Interview Answer**

Memory leaks waste memory but don't cause undefined behavior, crashes, or security vulnerabilities. Memory unsafety includes use-after-free, buffer overflows, and data races which are undefined behavior. Rust guarantees memory safety through its ownership system but explicitly allows memory leaks as a safe operation.

---

### Q6. How does `Box::leak` work and when is it intentionally used?

**Interview Answer**

`Box::leak` converts a heap allocation into a static reference, intentionally leaking memory for the program's lifetime. It's useful for creating static strings from runtime values or initializing global state. This pattern is safe and deliberate, not a bug, but should be used sparingly.

---

### Q7. How do you prevent memory leaks in async Rust applications?

**Interview Answer**

Avoid holding `Arc` references across await points when possible to prevent reference count accumulation. Use `tokio::select!` with timeouts to cancel long-running tasks that might hold resources. Regularly profile async tasks with `tokio-console` to identify tasks that never complete and leak memory.

---

### Q8. Can the garbage collector in other languages prevent all leaks?

**Interview Answer**

No. Garbage collectors prevent reference-counting cycles through tracing algorithms but can still leak through global references, event listeners, or cached objects. Rust's explicit memory management gives developers control over exactly when memory is freed, which can be more predictable than GC pauses for performance-critical backends.
