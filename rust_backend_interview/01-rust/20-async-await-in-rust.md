# Async/Await in Rust

## Interview Question

Explain Async/Await in Rust.

## Interview Answer

> "Async/await in Rust is a concurrency model that allows tasks to perform non-blocking operations. Instead of blocking a thread while waiting for I/O, such as a database query or network request, an async function yields control back to the runtime so other tasks can continue executing.
>
> Under the hood, every async function returns a Future. The Future represents a value that will become available later. A runtime like Tokio continuously polls these futures, and when an I/O operation completes, the future resumes execution from where it was suspended.
>
> In backend applications, async/await enables a small number of threads to handle thousands of concurrent requests efficiently, making it ideal for web servers, APIs, and microservices."

---

## Follow-up Questions & Answers

### Q1. What is async in Rust?

**Interview Answer:**

> "Async allows functions to perform non-blocking operations by returning a Future instead of executing immediately."

---

### Q2. What is `.await`?

**Interview Answer:**

> "The `.await` operator pauses the current async task until the Future is ready, while allowing the runtime to execute other tasks."

---

### Q3. Does `.await` block the thread?

**Interview Answer:**

> "No. It suspends only the current async task. The thread remains free to execute other async tasks."

---

### Q4. What does an async function return?

**Interview Answer:**

> "An async function returns a type that implements the `Future` trait."

---

### Q5. Why do we need Tokio?

**Interview Answer:**

> "Rust provides Futures, but it doesn't execute them. Tokio is the async runtime that schedules, polls, and runs asynchronous tasks."

---

### Q6. When should you use async?

**Interview Answer:**

> "For I/O-bound operations such as database access, HTTP requests, file operations, Redis, Kafka, and network communication."

---

### Q7. When should you avoid async?

**Interview Answer:**

> "For CPU-intensive work like image processing or large computations. Those tasks should use dedicated blocking threads or thread pools."

---

### Q8. Can async improve API response time?

**Interview Answer:**

> "It doesn't necessarily reduce the latency of a single request, but it significantly increases throughput by allowing many requests to be processed concurrently."

---

### Q9. How is async different from multithreading?

**Interview Answer:**

> "Multithreading runs work on multiple operating system threads. Async allows many tasks to share a small number of threads efficiently, especially for I/O-bound workloads."

---

### Q10. How do you use async in your Rust backend projects?

**Interview Answer:**

> "I use async throughout Axum handlers for database queries with `sqlx`, Redis operations, external API calls, Kafka producers and consumers, and other I/O-bound tasks to build highly concurrent services."
