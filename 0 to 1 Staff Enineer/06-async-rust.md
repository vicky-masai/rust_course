# LEVEL 06 — Async Rust

### 0180. Future Trait

A `Future` represents a value that may not be ready yet. `poll` asks: ready or pending? Async functions desugar into state machines implementing `Future`.

Nothing runs until something polls it (usually an executor like Tokio).

**Talk track:** *"A Future is lazy computation — polling drives it forward."*

---

### 0181. Poll

`Poll::Ready(v)` or `Poll::Pending`. The executor polls futures when progress might be possible. Returning `Pending` without arranging a wake = deadlock hang.

**Talk track:** *"Poll is the heartbeat of async — Ready or Pending, and Pending must register a wake."*

---

### 0182. Waker

Handle to wake a task so the executor polls it again. I/O drivers call `wake` when a socket is readable. Lost wakens = stuck tasks; extra wakes = wasted polls.

**Talk track:** *"The Waker is how the world tells the executor 'try this task again'."*

---

### 0183. Context

Passed into `poll` carrying the `Waker`. That's how nested futures register wakeups with the current task.

**Talk track:** *"Context is the poll environment — mainly the Waker for this task."*

---

### 0184. Async/Await Internals

`async` compiles to an enum state machine storing locals across `.await` points. `.await` polls a subfuture until ready, then continues. Not a OS thread block (if you use async I/O correctly).

**Talk track:** *"Async/await is syntax sugar over a state machine Future — awaits are yield points."*

---

### 0185. Async State Machines

Each await boundary is a state. Locals live across states → often need pinning. Understanding this explains weird lifetime errors and why large async fns create big futures (stack/frame size on executors).

**Talk track:** *"Your async fn is an enum of suspended states — size and lifetimes follow from that."*

---

### 0186. Async Cancellation

Dropping a future cancels it at the next await (roughly). Cleanup must be safe on cancel — use `Drop`, careful critical sections, `AbortHandle` patterns, and know that cancellation is normal (timeouts).

**Talk track:** *"Cancellation is drop. Design awaits so partial work can be abandoned safely."*

---

### 0187. Async Streams

Async sequence of values (`Stream` trait) — like `Iterator` but async. Backends: reading frames, Kafka-like consumption, chunked HTTP bodies.

Combine with `StreamExt` for combinators.

**Talk track:** *"Streams are async iterators — pull the next item when ready."*

---

### 0188. Backpressure

Slow consumers must slow producers or buffer (and eventually OOM). Bounded channels, pending polls, and HTTP flow control implement backpressure.

Staff systems design always asks where pressure propagates.

**Talk track:** *"Backpressure is how overload becomes 'slow down' instead of 'crash'."*
