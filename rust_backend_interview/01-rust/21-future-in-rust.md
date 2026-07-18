# Future in Rust

## Interview Question

Explain Future in Rust.

## Interview Answer

> "A Future represents a value that may not be available immediately but will be produced at some point in the future. Every async function in Rust returns a type implementing the Future trait.
>
> Calling an async function does not execute it immediately. Instead, it creates a Future object. The async runtime, such as Tokio, repeatedly polls the Future. If the operation cannot continue, it returns `Poll::Pending`, allowing the runtime to execute other tasks. Once the required resource, such as a database response, becomes available, the runtime polls the Future again until it returns `Poll::Ready` with the final result.
>
> This polling mechanism enables Rust to perform non-blocking asynchronous operations without requiring one operating system thread per request."

---

## Follow-up Questions & Answers

### Q1. What is a Future in Rust?

**Interview Answer:**

> "A Future represents an asynchronous computation that will produce a value later. Every async function returns a type implementing the Future trait."

---

### Q2. Does calling an async function execute it immediately?

**Interview Answer:**

> "No. Calling an async function creates a Future. Execution begins only when the Future is polled by a runtime such as Tokio."

---

### Q3. What does the `poll()` method do?

**Interview Answer:**

> "The `poll()` method checks whether the asynchronous operation has completed. It returns `Poll::Ready` if finished or `Poll::Pending` if it needs more time."

---

### Q4. What is `Poll::Pending`?

**Interview Answer:**

> "`Poll::Pending` indicates that the Future cannot make progress yet and should be polled again later."

---

### Q5. What is `Poll::Ready`?

**Interview Answer:**

> "`Poll::Ready` means the asynchronous computation has completed successfully and its result is available."

---

### Q6. Why are Futures considered zero-cost abstractions?

**Interview Answer:**

> "Because the compiler transforms async code into optimized state machines at compile time, avoiding unnecessary runtime overhead."

---

### Q7. What is the relationship between async and Future?

**Interview Answer:**

> "An async function is syntax sugar that the compiler converts into a type implementing the Future trait."

---

### Q8. Who executes a Future?

**Interview Answer:**

> "An async runtime such as Tokio or async-std repeatedly polls Futures until they complete."

---

### Q9. Can a Future run without a runtime?

**Interview Answer:**

> "Generally no. A Future needs an executor or runtime to poll it and drive it to completion."

---

### Q10. Why is understanding Futures important for backend development?

**Interview Answer:**

> "Because every async database query, HTTP request, Redis operation, Kafka consumer, and Axum handler is built on Futures. Understanding them helps write efficient, scalable, and non-blocking backend services."
