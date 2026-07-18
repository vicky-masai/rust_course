# Why Rust?

## Interview Question

Why Rust?

## Interview Answer

"Rust provides memory safety without a garbage collector, excellent performance, fearless concurrency, and strong compile-time checks. It helps prevent memory leaks and data races while producing highly efficient backend services."

---

## Follow-up Questions & Answers

### Q1. What makes Rust's memory safety different from garbage-collected languages?

**Interview Answer**

Rust uses an ownership system enforced at compile time, so there is no runtime garbage collector pausing threads. Each value has a single owner, and references follow strict borrowing rules that prevent dangling pointers and data races. This means memory is freed deterministically when it goes out of scope, giving both safety and predictable performance.

---

### Q2. Can you explain what "fearless concurrency" means in Rust?

**Interview Answer**

The compiler prevents data races by design through the `Send` and `Sync` traits. If two threads could access mutable shared state unsafely, the code simply won't compile. This lets you use `tokio::spawn`, channels, and mutexes confidently without the race condition bugs that plague C++ or Java concurrent code.

---

### Q3. What are zero-cost abstractions and why do they matter for backend systems?

**Interview Answer**

Zero-cost abstractions mean you pay no runtime overhead for using high-level features like iterators, generics, or async/await compared to hand-written low-level code. The compiler monomorphizes generics and inlines iterator chains, so the generated machine code is as fast as a manual implementation. For backend systems this means you can write clean, expressive code without sacrificing throughput.

---

### Q4. What are the downsides or tradeoffs of using Rust?

**Interview Answer**

The compile times are longer than Go or Java, especially for large projects with many dependencies. The learning curve around lifetimes, ownership, and async is steep for new team members. Ecosystem maturity in some areas like GUI or data science is still catching up, though the backend ecosystem with Axum, sqlx, and Tokio is very solid.

---

### Q5. How does Rust compare to Go for backend development specifically?

**Interview Answer**

Go is simpler to learn and has faster compile times with a built-in concurrency model using goroutines. Rust offers better raw performance, stronger type safety, and no garbage collector pauses. For most backend teams, Go is productive quickly, but Rust shines when you need predictable low latency and memory efficiency under heavy load.

---

### Q6. What role does `cargo` play in your daily Rust development?

**Interview Answer**

Cargo handles dependency management, building, testing, benchmarking, and publishing crates. I use `cargo clippy` for linting, `cargo fmt` for formatting, and `cargo test` for running unit and integration tests. The workspace feature lets me organize multi-crate projects cleanly, and `cargo doc` generates documentation automatically from code comments.

---

### Q7. How does Rust's error handling differ from exceptions in other languages?

**Interview Answer**

Rust uses `Result<T, E>` and the `?` operator instead of try-catch exceptions. Every possible error must be explicitly handled or propagated, so there are no hidden control flow paths. Libraries like `thiserror` and `anyhow` make defining and consuming errors ergonomic while keeping the codebase robust.

---
