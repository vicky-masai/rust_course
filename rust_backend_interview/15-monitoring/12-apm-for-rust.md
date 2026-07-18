# Application Performance Monitoring for Rust

## Interview Question

How do you implement comprehensive application performance monitoring (APM) for a Rust backend service?

## Interview Answer

APM combines metrics, traces, and logs to provide end-to-end visibility into application performance. In Rust, use the `metrics` crate for custom metrics, `tracing` with OpenTelemetry for distributed traces, and structured logging for context. Instrument critical code paths: HTTP handlers, database queries, cache operations, and external API calls. Export to an APM platform (Datadog, New Relic, Honeycomb) or build a custom stack with Prometheus, Jaeger, and Loki. Focus on business-critical transactions and user-facing operations.

---

## Follow-up Questions & Answers

### Q1. What are custom metrics and when should you create them?

**Interview Answer**

Custom metrics are application-specific measurements beyond standard HTTP/DB metrics. Create them for business operations: orders processed per minute, payment success rate, user registrations, search query latency. Use counters for rates, histograms for distributions, and gauges for current state. Name them consistently: `app_orders_total`, `app_payment_duration_seconds`, `app_search_results_count`. Custom metrics help you understand business health, not just technical health.

---

### Q2. How do you instrument Rust code for APM without cluttering business logic?

**Interview Answer**

Use `#[tracing::instrument]` on functions to auto-generate spans with timing. Create tower middleware layers for cross-cutting instrumentation (metrics, tracing). Use the `metrics` facade with recorders that can be swapped between test and production implementations. Instrument at architectural boundaries: HTTP handlers, repository methods, service calls. Use `tracing`'s span-based approach to add context without modifying function signatures. Keep instrumentation code separate from business logic using traits and layers.

---

### Q3. How do you monitor external API dependencies?

**Interview Answer**

Instrument HTTP clients with middleware that records request count, latency, and errors per external API. Use `reqwest-middleware` with a custom metrics layer. Track response codes, timeout rates, and circuit breaker states. Create per-dependency dashboards showing health and performance. Alert on degradation: increased latency, error rate spikes, or timeout increases. Use tracing to capture request/response metadata for debugging external API issues.

---

### Q4. How do you implement custom spans in Rust for APM?

**Interview Answer**

Use `tracing::info_span!("operation_name", field = value)` to create custom spans. The `#[tracing::instrument]` macro auto-generates spans with function parameters. Add business-relevant fields: `user_id`, `order_total`, `cache_hit`. Span duration becomes the operation's latency metric. Use `tracing::Instrument` trait to propagate spans across async task boundaries. Custom spans appear in distributed traces, providing detailed execution flow for debugging.

---

### Q5. How do you use the `metrics` crate effectively?

**Interview Answer**

Register metrics once at startup using `metrics::describe_counter!` and `metrics::describe_histogram!`. Use `metrics::counter!` and `metrics::histogram!` macros to record values. Choose the right type: counter for monotonically increasing values, gauge for bidirectional values, histogram for distributions. Use labels for dimensions but keep cardinality bounded. The `metrics` crate is a facade — configure the recorder (Prometheus, Datadog) at application start. Use `metrics-exporter-prometheus` for self-hosted monitoring.

---

### Q6. How do you integrate APM with CI/CD pipelines?

**Interview Answer**

Run performance benchmarks in CI using `criterion` to detect regressions. Compare benchmark results against baselines. Use synthetic monitoring in staging to validate performance before production. Track deployment impact on APM metrics (error rate, latency) in deployment dashboards. Automate rollback if metrics degrade. Store APM metric history to correlate with code changes. Use APM data to inform capacity planning in deployment pipelines.

---

### Q7. How do you monitor Rust application memory usage in production?

**Interview Answer**

Use `dhat` for allocation profiling in development. In production, monitor RSS and VSZ via system metrics exported by `prometheus-process-exporter`. Track allocation rates using custom metrics with `std::alloc::GlobalAlloc`. Monitor for memory leaks by tracking heap size over time. Set alerts on memory usage growth rates. Use `jemalloc` or `mimalloc` as the global allocator and monitor their statistics. Combine with `heaptrack` analysis during load testing.

---

### Q8. How do you implement error tracking in APM?

**Interview Answer**

Record error metrics with context: `metrics::counter!("app_errors_total", "type" => "validation", "code" => "400")`. Use `tracing::error!` with structured fields for error context. Export errors to error tracking services (Sentry, Bugsnag). Group errors by type, location, and impact. Alert on new error types or error rate spikes. Track error recovery rates — how many errors are retried successfully. Combine error metrics with traces to understand the full context of failures.

---

### Q9. How do you monitor async task performance in Rust?

**Interview Answer**

Track task spawn rate, completion rate, and active count using metrics. Use `tokio-console` for real-time async diagnostics. Measure poll duration for individual tasks using tracing spans. Alert on task accumulation (spawned >> completed). Monitor async runtime utilization (busy vs. idle time). Track queue depth for task channels. Use `tracing` with async-aware span propagation to maintain context across `.await` points. Combine async metrics with system metrics for a complete picture.

---

### Q10. How do you build a custom APM dashboard for a Rust service?

**Interview Answer**

Start with four key panels: request rate (RED), error rate, p99 latency, and throughput. Add resource panels: CPU, memory, connections. Include dependency panels: database latency, cache hit rate, external API health. Add business panels: key transaction counts, revenue metrics, user activity. Use Grafana variables for service and time range selection. Include deployment annotations. Link panels to detailed debugging dashboards. Review dashboard effectiveness monthly and remove unused panels.
