# What is work stealing?

## Interview Question

What is work stealing?

## Interview Answer

"Each worker thread has a local task queue. Idle workers steal tasks from busy workers, improving CPU utilization."

---

## Follow-up Questions & Answers

### Q1. How does work stealing differ from a global task queue?

**Interview Answer**

A global queue requires synchronization (mutex or lock-free queue) for every task push/pop. Work stealing uses per-thread deques—pushes and pops are local (fast), and only idle threads steal from others (rare). This reduces contention dramatically. Tokio, Rayon, and Go's scheduler all use work stealing variants.

---

### Q2. How does Tokio implement work stealing?

**Interview Answer**

Tokio's multi-threaded runtime gives each worker a local `LocalQueue` and shares a global injection queue. Tasks spawned with `tokio::spawn` go to the global queue. Workers first check their local queue, then the global queue, then steal from other workers' queues. This balances load across cores while minimizing contention.

---

### Q3. What data structure is used for the per-thread task queue?

**Interview Answer**

Work-stealing deques typically use Chase-Lev or similar lock-free deques. The owner pushes/pops from the bottom (LIFO), while thieves steal from the top (FIFO). This gives the owner cache-local LIFO behavior (good for data locality) while thieves get FIFO (fair distribution). `tokio` uses a custom bounded work-stealing queue.

---

### Q4. What happens when all workers are busy and new tasks arrive?

**Interview Answer**

New tasks go to the global injection queue. When a worker finishes its local queue, it checks the global queue first. If empty, it tries to steal from other workers. If all queues are empty, the worker parks (sleeps) and is woken when new tasks are injected. This prevents busy-waiting while maintaining low latency for new tasks.

---

### Q5. How does work stealing affect task locality?

**Interview Answer**

Tasks in the same future or spawned together tend to stay on the same worker, preserving cache locality. The LIFO local queue means the most recently spawned task is executed first, which is often the most cache-relevant. Stealing breaks locality but is necessary for load balancing—it's a tradeoff between cache efficiency and CPU utilization.

---

### Q6. Can work stealing cause starvation?

**Interview Answer**

In theory, a worker could have all its tasks stolen. In practice, stealing only happens when a thief is idle, so the victim was already doing useful work. The LIFO owner policy means recently pushed tasks are less likely to be stolen. Tokio's implementation ensures fair distribution by limiting how many tasks can be stolen per operation.

---

### Q7. How does work stealing interact with async/await?

**Interview Answer**

When a task `.await`s, it yields back to the worker's local queue. If the task is blocked on I/O, the reactor wakes it later and it's re-enqueued. This means a single spawned task might execute on different workers across its lifetime, but the work-stealing scheduler ensures it runs on an available core rather than being pinned to a potentially idle one.

---

### Q8. What is the overhead of work stealing?

**Interview Answer**

The overhead includes checking multiple queues (local, global, other workers'), which adds a few atomic operations per task dispatch. Stealing itself involves cache-line transfers between cores. In practice, this overhead is 10-50 nanoseconds per task, which is negligible for most async workloads. The benefit of better CPU utilization far outweighs this cost.

---

### Q9. How do you tune work-stealing parameters?

**Interview Answer**

Key parameters include local queue capacity, steal batch size, and injection queue capacity. Tokio allows configuring the number of worker threads via `tokio::runtime::Builder::worker_threads`. For compute-bound workloads, set workers equal to CPU cores. For I/O-bound workloads, more workers than cores can improve throughput by keeping the CPU busy during I/O waits.

---

### Q10. How does work stealing compare to work pushing?

**Interview Answer**

Work pushing (used by some schedulers) has workers push tasks directly to other workers' queues when local queues are full. Work stealing is lazy—only idle workers take from busy ones. Stealing has lower overhead because it's less proactive, but pushing can distribute load more evenly. Most production schedulers use stealing with a global fallback.

---
