# Why doesn't Rust have a garbage collector?

## Interview Question

Why doesn't Rust have a garbage collector?

## Interview Answer

"Rust uses deterministic ownership and RAII. Memory is released immediately when the owner goes out of scope, eliminating GC pauses while maintaining memory safety."

---

## Follow-up Questions & Answers

### Q1. What are the performance benefits of not having a garbage collector?

**Interview Answer**

No GC pauses means deterministic latency, which is critical for real-time and high-throughput backends. Memory is freed immediately when owners go out of scope, reducing memory usage spikes. Axum applications benefit from predictable request latency without GC-induced jitter.

---

### Q2. How does RAII work in Rust compared to C++?

**Interview Answer**

RAII in Rust ties resource lifetime to variable scope using the `Drop` trait, similar to C++ destructors. However, Rust's ownership system guarantees no double-frees or use-after-free, unlike C++. This makes RAII safer and more predictable in Rust for managing files, locks, and network connections.

---

### Q3. Can Rust's approach cause memory fragmentation?

**Interview Answer**

Rust uses system allocators like `jemalloc` or the platform default, which handle fragmentation differently. Without a GC to compact memory, fragmentation can occur in long-running applications with varied allocation sizes. Use `jemalloc` with `jemalloc-ctl` to monitor and manage fragmentation in Axum backends.

---

### Q4. How does Rust handle circular references without a GC?

**Interview Answer**

Rust prevents circular references at compile time through ownership rules. `Rc<RefCell<T>>` allows shared ownership but can create cycles; use `Weak<T>` to break them. `Arc<Mutex<T>>` can also form cycles in concurrent code, so design data structures to avoid bidirectional ownership.

---

### Q5. What languages have GC and why might Rust be preferable?

**Interview Answer**

Go, Java, Python, and JavaScript use garbage collectors that introduce latency pauses. Rust is preferable for systems requiring low latency, minimal memory overhead, and predictable performance. Axum backends benefit from Rust's deterministic cleanup for handling thousands of concurrent requests.

---

### Q6. How does the absence of GC affect Rust's memory model?

**Interview Answer**

Rust's ownership and borrowing rules provide compile-time memory safety without runtime overhead. The compiler tracks lifetimes and ensures references don't outlive their data. This eliminates entire classes of bugs like dangling pointers and data races that GC languages still face at runtime.

---

### Q7. Are there scenarios where a GC would be beneficial in Rust?

**Interview Answer**

Highly dynamic data structures with complex reference graphs are easier to manage with a GC. Some embedded systems or language runtimes built in Rust use custom GC for specific use cases. However, for most Axum backends, Rust's ownership model provides better performance and predictability.

---

### Q8. How does memory reclamation work for `Arc` without GC?

**Interview Answer**

`Arc` uses atomic reference counting to track how many owners exist. When the last `Arc` is dropped, the reference count reaches zero and memory is freed immediately. This is deterministic and doesn't require scanning the heap like a tracing GC, making it suitable for high-performance concurrent Axum handlers.
