# How do you profile Rust applications?

## Interview Question

How do you profile Rust applications?

## Interview Answer

"I use `cargo flamegraph`, `perf`, Tokio Console, tracing, and Prometheus metrics to identify CPU, memory, and async bottlenecks."

---

## Follow-up Questions & Answers

### Q1. How do you generate a flamegraph for a Rust application?

**Interview Answer**

Install `cargo-flamegraph` with `cargo install flamegraph`, then run `cargo flamegraph --bin your_app`. It uses `perf` on Linux or `dtrace` on macOS to sample CPU stacks and generate an SVG flamegraph. Focus on wide, tall bars which indicate functions consuming the most CPU time.

---

### Q2. What is Tokio Console and when should you use it?

**Interview Answer**

Tokio Console is a real-time diagnostics tool for Tokio applications that shows task state, resource usage, and scheduling information. Use it when you suspect async bottlenecks like blocked tasks, excessive polling, or runtime starvation. Enable it with `console-subscriber` and connect via the console UI.

---

### Q3. How do you profile memory usage in a Rust backend?

**Interview Answer**

Use `dhat-rs` for heap allocation profiling or `jemalloc` with `jemalloc-ctl` for detailed memory statistics. Run `valgrind --tool=massif` to capture heap snapshots over time. In Axum, monitor per-request memory allocation by instrumenting handlers with allocation tracking.

---

### Q4. How do you profile async task scheduling in Tokio?

**Interview Answer**

Use `tokio-console` to visualize task scheduling latency, poll counts, and waker frequency. Enable the `tracing` subscriber with `console-subscriber` and connect to the console dashboard. Look for tasks with high scheduling latency or excessive polling as indicators of runtime contention.

---

### Q5. What are the common performance bottlenecks in Axum applications?

**Interview Answer**

Common bottlenecks include blocking database calls, excessive JSON serialization, lock contention in shared state, and unoptimized middleware chains. Profile with `cargo flamegraph` to identify CPU-bound issues and `tokio-console` for async-specific problems. Monitor request latency with `tracing` and Prometheus metrics.

---

### Q6. How do you use `perf` to profile a Rust application on Linux?

**Interview Answer**

Run `perf record -g ./target/release/your_app` to collect profiling data, then `perf report` to analyze it. Use `perf stat` to check cache misses, branch prediction failures, and CPU cycles. Compile with debug symbols in `Cargo.toml` using `debug = true` for meaningful stack traces.

---

### Q7. How do you benchmark Axum API endpoints?

**Interview Answer**

Use `wrk`, `hey`, or `drill` for HTTP load testing and measure requests per second, latency percentiles, and error rates. For micro-benchmarks, use `criterion` crate to benchmark individual functions. Run benchmarks in production-like conditions with realistic data and network latency.

---

### Q8. How do you profile database query performance in sqlx?

**Interview Answer**

Enable PostgreSQL `pg_stat_statements` to track slow queries and their execution plans. Use `EXPLAIN ANALYZE` on specific queries to identify missing indexes or inefficient joins. Instrument sqlx queries with `tracing` spans to measure query duration in production and correlate with request latency.
