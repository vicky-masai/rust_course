# the Waker

## Interview Question

Explain the Waker.

## Interview Answer

"A `Waker` is a handle used by asynchronous resources to notify the executor that a suspended future is ready to make progress."

---

## Follow-up Questions & Answers

### Q1. How does a `Waker` differ from a `Future`?

**Interview Answer**

A `Future` represents a computation that may not be ready yet, while a `Waker` is the notification mechanism that tells the executor to re-poll the future. When a future returns `Poll::Pending`, it registers a `Waker` with the I/O system (epoll, kqueue). When the I/O event occurs, the `Waker` wakes the executor to re-poll the future.

---

### Q2. What happens when you call `waker.wake()`?

**Interview Answer**

`wake()` signals the executor that the associated task should be polled again. The executor re-enqueues the task's future and polls it on the next iteration of the event loop. `wake_by_ref()` does the same without consuming the waker, which is useful when you need to wake multiple times. Both are cheap, typically involving a queue push.

---

### Q3. How is a `Waker` created?

**Interview Answer**

`Waker` is created from a `RawWaker` which contains function pointers for `wake`, `wake_by_ref`, and `clone`/`drop`. Tokio creates wakers that push the task ID onto a run queue. Custom executors implement `RawWakerVTable` with their own wake semantics. The `futures::task` module provides utilities for creating and manipulating wakers.

---

### Q4. What is `Context` in async Rust?

**Interview Answer**

`Context` wraps a `Waker` and is passed to `Future::poll`. It provides the waker to the future so it can register for wake notifications. When you call `future.poll(cx)`, the future uses `cx.waker()` to get the waker and register it with the I/O system. This is the bridge between futures and the executor.

---

### Q5. How do wakers work with I/O events?

**Interview Answer**

When a future calls `poll`, it registers its `Waker` with the reactor (e.g., Tokio's I/O driver). The reactor associates the waker with a file descriptor or timer. When epoll/kqueue reports an event, the reactor calls `waker.wake()`, which re-enqueues the task. This is how async I/O avoids blocking—wakers bridge the gap between OS events and task scheduling.

---

### Q6. Can a waker be used across threads?

**Interview Answer**

Yes, `Waker` is `Send + Sync` by design. This allows the I/O driver (which may run on a dedicated thread) to wake tasks on a different thread. Tokio's multi-threaded runtime uses this: the I/O driver thread calls `wake()` which pushes the task to a work-stealing queue that any worker thread can pick up.

---

### Q7. What is a noop waker and when is it used?

**Interview Answer**

A noop waker ignores all wake calls. It's created with `Waker::noop()` (nightly) or `futures::task::noop_waker()`. It's useful for polling a future once to check its initial state without registering for notifications. For example, in `FutureExt::poll_unparked` or when testing future behavior in isolation.

---

### Q8. How do wakers interact with `select!` and `join!`?

**Interview Answer**

`select!` polls multiple futures and returns when any completes. Each future gets a cloned waker, but the `select!` macro wraps them so only the winning future's result is returned. `join!` polls all futures concurrently, each with their own waker registration. Both macros carefully handle waker registration to avoid missing notifications.

---

### Q9. What is a self-waking future?

**Interview Answer**

A self-waking future calls its own waker during `poll` to ensure it gets polled again. This is useful for immediate re-scheduling, like `std::future::ready()` which returns `Poll::Ready` immediately. Custom implementations might wake themselves to implement polling loops or to defer work to the next scheduler tick.

---

### Q10. How do wakers affect performance?

**Interview Answer**

Waker overhead comes from allocation (if cloned per poll), the wake syscall, and executor queue operations. Tokio minimizes this by reusing wakers across polls and using lock-free queues. Excessive wake calls can cause "thundering herd" problems where many tasks are woken unnecessarily. Proper waker registration in `poll` is critical for efficiency.

---
