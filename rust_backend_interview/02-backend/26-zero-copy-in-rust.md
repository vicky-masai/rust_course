# zero-copy in Rust

## Interview Question

Explain zero-copy in Rust.

## Interview Answer

"Rust minimizes unnecessary memory copies by borrowing data instead of cloning it."

---

## Follow-up Questions & Answers

### Q1. How does zero-copy work with Axum request bodies?

**Interview Answer**

Axum's `Body` type uses `bytes::Bytes` which supports zero-copy slicing and sharing via reference counting. When forwarding request bodies between services, you can slice the body without copying data. This is efficient for proxying, logging, or transforming request payloads in middleware.

---

### Q2. What is the difference between zero-copy and move semantics in Rust?

**Interview Answer**

Move semantics transfer ownership of data without copying, which is the default for heap-allocated types. Zero-copy goes further by borrowing data in place, avoiding any memory transfer. Both reduce overhead, but zero-copy requires careful lifetime management to ensure borrowed data outlives its references.

---

### Q3. How does `bytes::Bytes` enable zero-copy in network applications?

**Interview Answer**

`Bytes` wraps an reference-counted buffer that supports cheap slicing and cloning without data duplication. In Axum, response bodies and request payloads use `Bytes` so large payloads can be shared across tasks. This avoids allocations when forwarding data between handlers or middleware layers.

---

### Q4. Can you achieve zero-copy deserialization in Rust?

**Interview Answer**

Yes, using `serde` with `Cow<'a, str>` or `&'a [u8]` types to borrow directly from the input buffer. Libraries like `simd-json` and `serde_json` support zero-copy parsing with borrowed strings. This eliminates allocations for large JSON payloads in Axum handlers processing high-throughput APIs.

---

### Q5. What are the risks of zero-copy in Rust?

**Interview Answer**

Zero-copy requires careful lifetime management; borrowing data that goes out of scope causes compilation errors. Complex lifetime annotations can reduce code readability. In concurrent Axum handlers, borrowed data must outlive all tasks that reference it, which sometimes necessitates `Arc` or `Bytes` instead of raw borrows.

---

### Q6. How does zero-copy benefit IPC and shared memory scenarios?

**Interview Answer**

Zero-copy allows processes to share memory regions without serialization overhead, like using `memmap2` for memory-mapped files. In Rust, shared `Bytes` buffers can be passed between threads without copying. This is valuable for high-performance backends that need inter-process communication or large data transfers.

---

### Q7. How does zero-copy interact with Tokio's async runtime?

**Interview Answer**

Tokio tasks can share zero-copy buffers via `Arc<Bytes>` without blocking the runtime. Async I/O operations like `tokio::io::copy` can use zero-copy syscalls like `sendfile` for file transfers. In Axum, combining zero-copy with async I/O minimizes both CPU and memory overhead for large payloads.

---

### Q8. When should you avoid zero-copy optimizations?

**Interview Answer**

Avoid zero-copy when the code complexity outweighs performance gains, like in simple CRUD handlers with small payloads. It adds lifetime management overhead that can slow development. Profile first to confirm that data copying is actually a bottleneck before introducing zero-copy patterns.
