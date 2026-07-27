# LEVEL 07 — Tokio

### 0189. Runtime

Tokio's executor + I/O driver + timer wheel. Runs async tasks, waits on OS events, wakes ready tasks. Multi-threaded or current-thread flavors.

Without a runtime, futures sit inert.

**Talk track:** *"The runtime is the engine that polls futures and drives I/O."*

---

### 0190. Scheduler Internals

Tasks sit on run queues; workers pull and poll them. When Pending on I/O, task parks until woken. Understanding this helps explain latency and "why is my task starved?"

**Talk track:** *"The scheduler decides which ready task runs next on which worker."*

---

### 0191. Work Stealing

Idle workers steal tasks from busy workers' queues — balancing load across cores. Improves CPU utilization for uneven task sizes.

**Talk track:** *"Work stealing keeps cores busy when task load is uneven."*

---

### 0192. Cooperative Scheduling

Async tasks must yield at await points. A tight CPU loop without `.await` blocks the worker — stalls other tasks on that thread. Use `spawn_blocking` or `tokio::task::yield_now` / chunk work.

**Talk track:** *"Tokio is cooperative — hog a worker without awaiting and everyone on that thread suffers."*

---

### 0193. spawn()

Schedule a `'static` task on the runtime. Returns `JoinHandle`. Fire-and-forget or await the handle. Panics are caught in the handle (usually).

**Talk track:** *"spawn schedules a background task — it needs owned/'static data."*

---

### 0194. spawn_blocking()

Run blocking/CPU-heavy/sync I/O on a dedicated blocking thread pool so async workers stay free. Database sync drivers, file I/O, heavy crypto.

Don't flood it — it's bounded.

**Talk track:** *"spawn_blocking quarantines blocking work off the async workers."*

---

### 0195. JoinHandle

Future that completes when a spawned task finishes, yielding its result or join error. Abort via `abort()` (cooperative cancel on next await).

**Talk track:** *"JoinHandle is your handle to a task's result and cancellation."*

---

### 0196. JoinSet

Dynamic set of tasks — spawn many, wait for next completion, structured concurrency helper. Great for worker pools and fan-out.

**Talk track:** *"JoinSet manages a herd of tasks and lets you reap them as they finish."*

---

### 0197. select!

Wait on multiple futures; proceed with the first that completes. Cancel losers (drop). Essential for timeouts, shutdown, multi-source events.

Biased vs random fairness options matter under load.

**Talk track:** *"select! races futures — first winner runs, others cancel."*

---

### 0198. timeout()

Wrap a future with a deadline. On expiry, returns error and cancels the inner future. Always consider cleanup and idempotent retries after timeout (the work might still have committed).

**Talk track:** *"timeout bounds latency — treat expiry as cancel + possible uncertainty."*

---

### 0199. Semaphore

Limit concurrent permits — e.g., max 100 in-flight DB ops. Classic backpressure/concurrency governor.

**Talk track:** *"Semaphores cap concurrency so you don't melt downstream."*

---

### 0200. Notify

Simple wake-one/wake-all signal without carrying data. Lightweight coordination: "something changed, re-check state."

**Talk track:** *"Notify is a thin wakeup signal — pair it with a shared state check."*

---

### 0201. Barrier

Async barrier: N tasks wait until all arrive, then proceed. Rare but useful for phased startup/tests.

**Talk track:** *"Barriers synchronize 'everyone ready' phases."*

---

### 0202. MPSC

Tokio's async multi-producer single-consumer channel. Bounded variants apply backpressure when full (`send().await`).

**Talk track:** *"Tokio MPSC moves messages between tasks with optional backpressure."*

---

### 0203. Broadcast

One message, many receivers (each gets a clone/lag handling). Fan-out events. Slow consumers may lag and miss (documented behavior).

**Talk track:** *"Broadcast fans out the same event to many listeners — watch lag."*

---

### 0204. Watch Channels

Single-value slot many can watch — "latest config" pattern. Receivers see updates; great for settings and readiness flags.

**Talk track:** *"Watch is a latest-value channel — perfect for config propagation."*
