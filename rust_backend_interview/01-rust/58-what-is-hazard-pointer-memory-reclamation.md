# What is hazard pointer memory reclamation?

## Interview Question

What is hazard pointer memory reclamation?

## Interview Answer

"Hazard pointers allow safe memory reclamation in lock-free data structures by preventing nodes from being freed while another thread may still access them."

---

## Follow-up Questions & Answers

### Q1. How do hazard pointers work at a high level?

**Interview Answer**

Each thread publishes a "hazard pointer" indicating which nodes it might be accessing. Before freeing a node, a thread scans all hazard pointers to check if any thread is still referencing it. If not, the node is safe to free. If so, it's deferred to a retire list for later reclamation. This guarantees no thread accesses freed memory.

---

### Q2. What is the advantage of hazard pointers over epoch-based reclamation?

**Interview Answer**

Hazard pointers provide bounded memory usage—retired nodes are freed as soon as no thread references them. Epoch-based reclamation can delay reclamation until all threads pass the next epoch, potentially holding large amounts of memory. Hazard pointers are better for workloads with high churn, while epoch-based is simpler and faster for low-contention cases.

---

### Q3. How are hazard pointers implemented in Rust?

**Interview Answer**

The `haphazard` crate provides a safe Rust API for hazard pointers. It uses `AtomicPtr` for the hazard pointer slots and `Box::into_raw`/`Box::from_raw` for manual memory management. Each thread has a local hazard pointer slot, and scanning involves reading all slots to check for conflicts. The implementation requires careful `unsafe` code to manage raw pointers.

---

### Q4. What is the scan phase in hazard pointer reclamation?

**Interview Answer**

The scan phase reads all threads' hazard pointers and compares them against the retire list. For each retired node, if its address matches any hazard pointer, it cannot be freed yet. Nodes that pass the scan are safely deallocated. The scan is O(N*M) where N is threads and M is retired nodes, so efficient retirement batching is important.

---

### Q5. Can hazard pointers be used with `Arc`?

**Interview Answer**

Hazard pointers replace `Arc` for certain lock-free data structures. While `Arc` uses reference counting (atomic increment/decrement), hazard pointers avoid the atomic operations on the hot path by deferring cleanup. For structures like lock-free lists where `Arc` causes excessive atomic traffic, hazard pointers can significantly improve performance.

---

### Q6. What is the retire list and how is it managed?

**Interview Answer**

The retire list stores nodes that are no longer reachable but might still be referenced by other threads. Each thread maintains a local retire list. Periodically (or when the list grows too large), a scan is performed to reclaim safe nodes. Local retirement avoids contention on a global list, improving scalability.

---

### Q7. How do hazard pointers handle thread termination?

**Interview Answer**

When a thread terminates, it must scan and clear its hazard pointers so other threads can reclaim nodes it was protecting. The thread also transfers its retire list to a global list or another thread for cleanup. Failure to do this causes memory leaks. The `haphazard` crate handles this via `Drop` implementation for the thread-local state.

---

### Q8. What is the performance overhead of hazard pointers?

**Interview Answer**

Each read requires an `Acquire` fence to publish the hazard pointer, and each scan requires reading all threads' hazard pointers. The overhead is ~50-100ns per protected access, compared to ~10ns for `Arc::clone`. This is justified when the alternative is much higher `AtomicUsize` contention from reference counting on a hot path.

---

### Q9. When should you choose hazard pointers over other reclamation schemes?

**Interview Answer**

Choose hazard pointers when: (1) memory usage must be bounded, (2) the data structure has high node turnover, (3) `Arc` reference counting causes unacceptable contention, or (4) real-time constraints require predictable reclamation latency. For simpler cases, `arc-swap` (epoch-based) or `Arc` with sharding may be sufficient.

---

### Q10. What are the common pitfalls with hazard pointers?

**Interview Answer**

Common mistakes include forgetting to clear hazard pointers on thread exit, not scanning frequently enough (causing memory buildup), and incorrect `unsafe` pointer handling. Race conditions between publishing hazard pointers and reading the data structure are subtle—always use `Acquire`/`Release` ordering. Testing under high concurrency with `loom` is essential.

---
