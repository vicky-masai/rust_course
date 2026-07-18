# What are epoch-based garbage collection techniques?

## Interview Question

What are epoch-based garbage collection techniques?

## Interview Answer

"They defer memory reclamation until all threads have moved past the epoch in which an object became unreachable. This is widely used in lock-free structures."

---

## Follow-up Questions & Answers

### Q1. How does epoch-based reclamation work step by step?

**Interview Answer**

Each thread maintains a local epoch counter. When accessing shared data, the thread enters a "critical section" by publishing its current epoch. When no thread is in an epoch older than the one when an object was retired, that object is safe to free. The global epoch advances when all threads have moved past it.

---

### Q2. What is `crossbeam-epoch` and how does it relate to this?

**Interview Answer**

`crossbeam-epoch` is Rust's most popular epoch-based reclamation library. It provides `pin()`, `unpin()`, `Shared`, and `Atomic` types for safe epoch-managed memory. `epoch::pin()` enters a critical section, and `Shared::into_owned()` defers reclamation. It handles all the unsafe pointer management and thread-local epoch tracking.

---

### Q3. What is the difference between epoch-based and hazard pointer reclamation?

**Interview Answer**

Epoch-based reclamation batches reclamation by epoch—objects are freed when all threads pass the epoch. Hazard pointers free objects as soon as no thread references them. Epoch-based is faster (fewer scans) but can hold more memory. Hazard pointers provide tighter memory bounds at the cost of more scanning overhead.

---

### Q4. What is the ABA problem in epoch-based reclamation?

**Interview Answer**

If an object is freed and reallocated within the same epoch, a thread might see the same pointer but with different contents. Epoch-based reclamation prevents this by deferring deallocation until all threads from the old epoch have exited. This ensures the pointer cannot be reused while any thread might still reference it.

---

### Q5. How do you enter and exit a critical section?

**Interview Answer**

Use `let pin = crossbeam_epoch::pin();` to enter and `drop(pin)` to exit. The `pin` guard tracks the current epoch. While pinned, the thread's epoch is visible to the reclamation system. Critical sections should be as short as possible because they prevent reclamation of objects retired before the thread's epoch.

---

### Q6. What is the overhead of epoch-based reclamation?

**Interview Answer**

Entering a critical section requires an atomic load of the global epoch (~5ns). Exiting is free (just dropping the guard). The overhead is much lower than hazard pointers but higher than no reclamation. The tradeoff is holding memory longer—objects from old epochs can't be freed until all threads from that epoch exit.

---

### Q7. How do you retire objects in epoch-based reclamation?

**Interview Answer**

Use `crossbeam_epoch::Atomic::swap(ptr, Ordering::Release)` to remove the object and `pin.defer_destroy(old_ptr)` to schedule its reclamation. The `defer_destroy` method adds the pointer to a thread-local retire list. When the epoch advances, all objects retired before the old epoch are deallocated.

---

### Q8. Can epoch-based reclamation cause memory leaks?

**Interview Answer**

Yes, if threads never exit critical sections, old epochs are never vacated and objects are never freed. This is called "starvation" of reclamation. Mitigations include ensuring threads periodically exit critical sections, using bounded critical sections, and monitoring epoch advancement. In practice, this is rare because threads naturally enter and exit critical sections.

---

### Q9. How does epoch-based reclamation interact with `Arc`?

**Interview Answer**

`crossbeam-epoch` can replace `Arc` for lock-free structures by managing memory directly. Instead of incrementing/decrementing reference counts, you publish pointers and defer destruction. This eliminates atomic reference counting overhead but requires more careful `unsafe` code. `arc-swap` combines `Arc` with epoch-based reclamation for safe pointer swaps.

---

### Q10. When should you use epoch-based reclamation in a Rust backend?

**Interview Answer**

Use it for lock-free caches, concurrent data structures, and hot-path shared state where `Arc` reference counting becomes a bottleneck. `arc-swap` provides a safe API for this pattern. It's ideal for read-heavy workloads where the data rarely changes but is accessed millions of times per second, like configuration or routing tables.

---
