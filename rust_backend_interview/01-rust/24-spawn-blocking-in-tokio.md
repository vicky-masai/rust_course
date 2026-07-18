# `spawn_blocking()` in Tokio

## Interview Question

Explain `spawn_blocking()` in Tokio.

## Interview Answer

> "Tokio's `spawn_blocking()` is designed for CPU-intensive or blocking operations that should not run on Tokio's async worker threads. Unlike `tokio::spawn()`, which schedules asynchronous tasks, `spawn_blocking()` executes the work on a dedicated blocking thread pool.
>
> This prevents long-running computations or blocking system calls from occupying Tokio's worker threads, ensuring that the async runtime remains responsive.
>
> In backend applications, I use `spawn_blocking()` for operations such as PDF generation, Excel processing, image resizing, compression, encryption, and other CPU-heavy workloads."

---

## Follow-up Questions & Answers

### Q1. What is `spawn_blocking()`?

**Interview Answer:**

> "`spawn_blocking()` executes blocking or CPU-intensive work on Tokio's dedicated blocking thread pool instead of its async worker threads."

---

### Q2. Why do we need `spawn_blocking()`?

**Interview Answer:**

> "It prevents CPU-heavy or blocking tasks from occupying Tokio's worker threads, keeping the async runtime responsive."

---

### Q3. What's the difference between `tokio::spawn()` and `spawn_blocking()`?

**Interview Answer:**

> "`tokio::spawn()` is for asynchronous I/O-bound work, while `spawn_blocking()` is for CPU-intensive or blocking operations."

---

### Q4. What kinds of tasks should use `spawn_blocking()`?

**Interview Answer:**

> "PDF generation, Excel parsing, image processing, compression, encryption, report generation, and other CPU-bound workloads."

---

### Q5. Can database queries use `spawn_blocking()`?

**Interview Answer:**

> "Generally no. Async database libraries like `sqlx` already provide non-blocking I/O and should be awaited directly."

---

### Q6. What happens if CPU-intensive work runs inside `tokio::spawn()`?

**Interview Answer:**

> "It occupies a Tokio worker thread for a long time, reducing throughput and delaying other asynchronous tasks."

---

### Q7. Does `spawn_blocking()` create a new thread every time?

**Interview Answer:**

> "No. Tokio uses a managed blocking thread pool and schedules blocking tasks onto it."

---

### Q8. Can `spawn_blocking()` return a value?

**Interview Answer:**

> "Yes. It returns a `JoinHandle`, which can be awaited to obtain the computation's result."

---

### Q9. Is `spawn_blocking()` asynchronous?

**Interview Answer:**

> "The closure itself runs synchronously on a blocking thread, but awaiting its `JoinHandle` integrates cleanly with async code."

---

### Q10. How do you use `spawn_blocking()` in your backend projects?

**Interview Answer:**

> "I use it for CPU-intensive operations such as processing large Excel files, generating reports or PDFs, resizing images, compressing files, and performing expensive computations while keeping the Axum/Tokio runtime responsive."
