# LEVEL 18 — Performance Engineering

### 0342. Benchmarking

Measure before and after. Control warmups, pin CPU if needed, watch noise. Microbenchmarks lie without realism; macro (load tests) lie without production likeness.

**Talk track:** *"Benchmark with a hypothesis — measure end-to-end and the hot spot."*

---

### 0343. Criterion

Rust statistics-aware benchmark harness. Handles warm-up and noise better than naive timers. Use for library/hot-function regressions in CI.

**Talk track:** *"Criterion makes Rust microbenchmarks statistically honest."*

---

### 0344. CPU Profiling

Find where CPU time goes — `perf`, flamegraphs, sampling profilers. Don't optimize cold code.

**Talk track:** *"CPU profiles show hot stacks — optimize what the samples hit."*

---

### 0345. Memory Profiling

Find allocations, leaks, RSS growth — heaptrack, DHAT, jemalloc stats, custom alloc metrics. Memory issues show up as latency and OOM.

**Talk track:** *"Memory profiles find churn and leaks — allocation rate is a latency predictor."*

---

### 0346. Flamegraphs

Visualization: width = time/samples in a stack frame. Wide plateaus are suspects. Standard language of performance conversations.

**Talk track:** *"Flamegraphs make hot paths obvious — read width, not just height."*

---

### 0347. Allocation Reduction

Reuse buffers, preallocate, avoid format! in hot loops, pool objects, stream instead of collect. In Rust, ownership helps you see allocations.

**Talk track:** *"Fewer allocations in hot paths → better latency and less allocator contention."*

---

### 0348. Cache Optimization

Improve data locality, shrink hot structs, SoA vs AoS, avoid false sharing, batch work. Align algorithms with CPU caches.

**Talk track:** *"Cache optimization is data layout engineering — make the hot set small and sequential."*

---

### 0349. Latency Analysis

Break latency into queueing, network, serialization, DB, lock wait. Histograms/percentiles (p50/p95/p99) matter more than averages. Tail latency dominates user pain.

**Talk track:** *"Latency is a distribution — chase p99 with stage-by-stage budgets."*
