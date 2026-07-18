# `Mutex<T>` and `RwLock<T>` in Rust

## Interview Question

Explain `Mutex<T>` and `RwLock<T>` in Rust.

## Interview Answer

> "Mutex and RwLock are synchronization primitives used to safely access shared mutable data across multiple threads.
>
> A Mutex allows only one thread to access the protected data at a time. Every other thread must wait until the lock is released.
>
> An RwLock is optimized for read-heavy workloads. It allows multiple readers to access the data simultaneously, but when a writer acquires the lock, all readers are blocked until the write operation completes.
>
> In backend applications, I use Mutex when writes are frequent and RwLock when reads are much more common than writes, such as configuration data or in-memory caches."

---

## Follow-up Questions & Answers

### Q1. What is a Mutex?

**Interview Answer:**

> "A Mutex is a synchronization primitive that allows only one thread to access shared data at a time."

---

### Q2. What is an RwLock?

**Interview Answer:**

> "An RwLock allows multiple readers simultaneously but only one writer with exclusive access."

---

### Q3. What's the difference between Mutex and RwLock?

**Interview Answer:**

> "Mutex serializes all access, while RwLock allows concurrent reads and exclusive writes, making it more efficient for read-heavy workloads."

---

### Q4. When should you use Mutex?

**Interview Answer:**

> "When writes are frequent or when exclusive access is required regardless of whether the operation is a read or write."

---

### Q5. When should you use RwLock?

**Interview Answer:**

> "When the application performs many reads and relatively few writes, such as configuration or caching."

---

### Q6. Why is `Arc<Mutex<T>>` common in Rust?

**Interview Answer:**

> "`Arc` enables shared ownership across threads, while `Mutex` provides synchronized mutable access to the shared data."

---

### Q7. What happens if a thread panics while holding a Mutex?

**Interview Answer:**

> "The mutex becomes poisoned. Future `lock()` calls return a `PoisonError`, allowing the application to decide how to recover."

---

### Q8. Can multiple threads hold a write lock on an RwLock?

**Interview Answer:**

> "No. Only one writer can hold the lock at a time, and readers are blocked until the writer releases it."

---

### Q9. Is RwLock always faster than Mutex?

**Interview Answer:**

> "No. RwLock performs better for read-heavy workloads. If writes are frequent, the overhead of managing reader and writer locks may make Mutex a better choice."

---

### Q10. Where do you use Mutex and RwLock in backend applications?

**Interview Answer:**

> "I use Mutex for shared mutable counters, queues, or state that changes frequently. I use RwLock for shared configuration, in-memory caches, feature flags, and other read-heavy data structures."
