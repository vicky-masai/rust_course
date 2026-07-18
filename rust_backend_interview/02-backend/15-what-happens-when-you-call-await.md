# What happens when you call `.await`?

## Interview Question

What happens when you call `.await`?

## Interview Answer

"`await` yields execution to the async runtime. The Future stores its state, allowing the runtime to execute other tasks. When the awaited operation is ready, the Future resumes from where it paused."

---

## Follow-up Questions & Answers

### Q1. What is a Future in Rust and how is it different from a thread?

**Interview Answer**

A Future in Rust is a lazy state machine that represents an asynchronous computation, and it does nothing until polled. A thread is an OS-level execution context with its own stack that runs independently. Futures are much lighter weight since they share a single thread through Tokio's runtime, allowing millions of concurrent tasks where threads would exhaust memory.

---

### Q2. What does "yielding execution" mean at the runtime level?

**Interview Answer**

When you call `.await`, the current task returns `Poll::Pending` to the Tokio runtime, which then polls other ready tasks on the same thread. This is cooperative multitasking where tasks voluntarily give up control. The runtime uses an epoll or kqueue event loop under the hood to efficiently wait for I/O readiness without busy-waiting.

---

### Q3. What is the difference between `.await` and `tokio::spawn`?

**Interview Answer**

`.await` pauses the current task and runs other tasks on the same thread until the awaited operation completes. `tokio::spawn` creates an entirely new task that can run on any thread in the Tokio thread pool, independent of the spawning task. I use `.await` for sequential dependent operations and `spawn` for independent work that should execute concurrently.

---

### Q4. What happens if you block inside an async function?

**Interview Answer**

Blocking inside an async function, like calling `std::thread::sleep` or doing CPU-heavy work, blocks the entire Tokio worker thread and prevents other tasks from making progress. I use `tokio::task::spawn_blocking` to move CPU-bound work to a dedicated thread pool. For I/O, I always use async-aware implementations like `tokio::fs` or `sqlx` instead of synchronous equivalents.

---

### Q5. How does Rust's async/await differ from JavaScript's or Python's?

**Interview Answer**

Rust futures are zero-cost state machines compiled at build time, while JavaScript Promises and Python coroutines have runtime overhead from their event loops and garbage collectors. Rust requires an explicit runtime like Tokio, giving you control over the executor, while JavaScript has a built-in event loop. Rust's type system also ensures futures are `Send` for thread safety, which neither JS nor Python guarantee.

---

### Q6. What is the problem with `async` traits and how is it solved?

**Interview Answer**

Async functions in traits were not directly supported until Rust 1.75 because the compiler couldn't size the returned `dyn Future` without `dyn` or `async-trait`. I used the `async-trait` crate which boxes the future as `Pin<Box<dyn Future>>`. With Rust 1.75+, I can use `-> impl Future` or native async trait support to avoid the boxing overhead.

---

### Q7. How do you debug async Rust code when things go wrong?

**Interview Answer**

I use `tracing` with spans to instrument async functions and see the poll lifecycle of each future. The `tokio-console` crate provides real-time visibility into task states, wait times, and which tasks are blocking. For stuck tasks, I add timeout wrappers with `tokio::time::timeout` to prevent infinite hangs and log when they trigger.

---

### Q8. What is `Pin` and why is it necessary for async code?

**Interview Answer**

`Pin` prevents a future from being moved in memory after it's been polled, because self-referential state machines would have dangling pointers if moved. When a future captures references to its own fields across `.await` points, moving it would invalidate those references. `Pin<Box<T>>` is the common pattern for heap-allocated pinned futures, and it's required for trait objects and `async-trait`.

---
