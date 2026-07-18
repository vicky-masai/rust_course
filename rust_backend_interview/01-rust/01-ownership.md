# Ownership

## Interview Question

Explain Ownership in Rust.

## Interview Answer

> "Ownership is Rust's core memory management model. Every value in Rust has exactly one owner at any given time. When the owner goes out of scope, Rust automatically releases the associated resources by calling the Drop trait. This enables deterministic memory management without requiring a garbage collector.

> Ownership prevents common memory safety issues such as use-after-free, dangling pointers, double-free errors, and many memory leaks. Because these rules are enforced at compile time, Rust achieves memory safety without runtime overhead.

> In backend development, ownership allows me to build high-performance services where resources such as database connections, buffers, files, sockets, and asynchronous tasks are managed safely and predictably."

---

## Follow-up Questions & Answers

## Q1. What is Ownership?

**Interview Answer**

Ownership is Rust's compile-time memory management model where every value has exactly one owner responsible for cleaning up the resource.

---

## Q2. Why did Rust introduce Ownership?

**Interview Answer**

Rust introduced ownership to provide memory safety without requiring a garbage collector. It prevents common memory bugs at compile time.

---

## Q3. What happens when ownership is moved?

**Interview Answer**

The original variable becomes invalid, and the new variable becomes the sole owner of the value.

---

## Q4. What happens when a value goes out of scope?

**Interview Answer**

Rust automatically calls the `Drop` implementation and releases the associated resources.

---

## Q5. Why isn't `String` a Copy type?

**Interview Answer**

Because `String` owns heap memory. Automatically copying it would create multiple owners of the same allocation, leading to double-free errors.

---

## Q6. Which types implement `Copy`?

**Interview Answer**

Primitive stack-based types such as:

- i32
- u64
- bool
- char
- f64

---

## Q7. What is the difference between Move and Copy?

**Interview Answer**

Move transfers ownership without duplicating the heap allocation. Copy duplicates small stack values while both variables remain valid.

---

## Q8. How does Ownership improve backend performance?

**Interview Answer**

Ownership eliminates garbage collection pauses and enables deterministic cleanup, making Rust well suited for high-throughput backend services.

---

## Q9. How have you used Ownership in production?

**Interview Answer**

"I've used ownership throughout Axum handlers, SQLx database operations, Tokio background tasks, and service layers. I move values only when ownership should change and borrow values when multiple components need read-only access."

---

## Q10. How is Rust's Ownership different from Java's Garbage Collection?

**Interview Answer**

Java relies on a runtime garbage collector to reclaim memory, while Rust determines ownership and cleanup entirely at compile time. This provides predictable performance and avoids GC pauses.

---
