# Monitoring Backend Services

## Interview Question

What monitoring methodologies and patterns do you use for backend services, and how do you implement them in Rust?

## Interview Answer

Apply established monitoring methodologies: the **RED method** (Rate, Errors, Duration) for request-driven services, the **USE method** (Utilization, Saturation, Errors) for resource monitoring, and **golden signals** (Latency, Traffic, Errors, Saturation) for service-level health. Implement these using the `metrics` crate in Rust, exposing counters, gauges, and histograms. Combine with distributed tracing via `tracing` and OpenTelemetry for request-level visibility. Build tiered dashboards: executive overview → service health → debugging detail. Alert on symptoms (user impact) using SLO-based burn rates.

---

## Follow-up Questions & Answers

### Q1. How do you apply the RED method to a Rust axum service?

**Interview Answer**

Use `tower` middleware to automatically record RED metrics for every request: increment a counter for request rate (`http_requests_total{method, path, status}`), track error rate from status codes, and record latency in a histogram. The `metrics` crate makes this straightforward with `increment!` and `histogram!` macros. Create a `MetricsLayer` that wraps all routes. This provides per-endpoint RED metrics without adding instrumentation to each handler individually.

---

### Q2. How do you implement the USE method for monitoring Rust runtime resources?

**Interview Answer**

Track tokio runtime metrics using `metrics`: task count, worker count, busy time vs. idle time. Monitor system-level resources with `sysinfo` crate: CPU utilization, memory usage, open file descriptors. Use `prometheus-process-exporter` for OS-level metrics. Create gauges for connection pool saturation (active vs. max connections). Record error rates for resource exhaustion (OOM kills, connection timeouts). Combine runtime and system metrics in a resource health dashboard.

---

### Q3. What are the golden signals and how do you implement them?

**Interview Answer**

Latency: histogram of request duration from middleware. Traffic: counter of total requests. Errors: counter of error responses (4xx/5xx) or counter of error log entries. Saturation: gauge of resource usage (CPU, memory, connections). Implement all four using the `metrics` crate with a `MetricsLayer`. Export via `metrics-exporter-prometheus`. Build a Grafana dashboard with one panel per golden signal. Alert when any signal degrades beyond SLO thresholds.

---

### Q4. How do you monitor Rust async runtime health?

**Interview Answer**

Use `tokio-console` for real-time async runtime diagnostics: task count, busy/idle time, polling duration. Export tokio metrics to Prometheus using `metrics` crate hooks. Monitor task spawn rates and completion rates. Alert on task accumulation (more spawned than completed = potential leak). Track poll duration percentiles to detect blocking operations in async context. Use `tracing` spans with timing to measure async operation durations.

---

### Q5. How do you monitor database connection pools?

**Interview Answer**

Record pool metrics: active connections, idle connections, wait count, wait time. Use the `metrics` crate with custom instrumentation around the pool. In `sqlx`, enable pool metrics with the `metrics` feature. Alert on pool exhaustion (active = max_connections). Track connection wait time as a latency histogram. Monitor connection creation and destruction rates. Create a dashboard showing pool utilization trends to plan capacity. Configure pool timeouts and monitor timeout rates.

---

### Q6. How do you implement health scoring for services?

**Interview Answer**

Combine multiple signals into a composite health score: SLI compliance (40%), latency performance (25%), error rate (20%), resource saturation (15%). Calculate each component as a 0-100 score based on SLO targets. Weight and sum for overall health. Display on dashboards with traffic-light coloring. Use health scores in incident prioritization — lower-scoring services get attention first. Automate health score calculations using Prometheus recording rules and Grafana calculations.

---

### Q7. How do you set up service dependency monitoring?

**Interview Answer**

Instrument all outbound calls (HTTP clients, database, cache, message queue) with metrics: request count, error rate, latency per dependency. Use `tracing` to create spans for each dependency call. Build dependency health dashboards showing status of all downstream services. Alert when dependency error rates exceed thresholds. Monitor circuit breaker state transitions. Use distributed traces to identify which dependency is causing latency. Create a service map visualization from trace data.

---

### Q8. How do you monitor deployment health?

**Interview Answer**

Track deployment markers using annotations or metrics. Compare error rates and latency before and after deployment. Use canary deployments and monitor canary vs. stable metrics. Automate rollback if canary metrics degrade beyond thresholds. Track deployment frequency, lead time, and change failure rate (DORA metrics). Monitor container restart counts and readiness probe failures. Create deployment-specific dashboards showing the impact of each release.

---

### Q9. How do you implement capacity monitoring and forecasting?

**Interview Answer**

Record resource utilization over time: CPU, memory, storage, network. Use Prometheus's `predict_linear()` to forecast when resources will be exhausted. Set alerts for projected capacity breaches (7 days ahead). Track throughput trends and model resource consumption per request. Conduct regular load tests and record the results. Create capacity dashboards showing current usage, growth trends, and headroom. Plan scaling events before capacity runs out.

---

### Q10. How do you monitor microservice communication patterns?

**Interview Answer**

Use distributed tracing to map service-to-service communication: who calls whom, how often, and with what latency. Build a service dependency graph from trace data. Monitor inter-service error propagation. Track async communication (message queues) separately from synchronous (HTTP). Use OpenTelemetry collector to analyze trace data and generate service maps. Alert on new dependencies or changed communication patterns that might indicate architectural drift.
