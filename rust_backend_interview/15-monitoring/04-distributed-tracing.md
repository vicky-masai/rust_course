# Distributed Tracing

## Interview Question

How do you implement distributed tracing in a Rust microservices architecture?

## Interview Answer

Use OpenTelemetry (OTel) as the tracing SDK, configured with `tracing-opentelemetry` to bridge Rust's `tracing` crate with OTel. Each request gets a trace ID at the entry point, propagated via `traceparent` headers between services. Spans represent individual operations (HTTP call, DB query) with timing and metadata. Export traces to a backend like Jaeger or Tempo for visualization. The trace context flows automatically through HTTP clients and servers when properly instrumented.

---

## Follow-up Questions & Answers

### Q1. What is the difference between a trace, a span, and a context?

**Interview Answer**

A **trace** represents the complete journey of a request across services, identified by a unique trace ID. A **span** is a single operation within a trace (e.g., an HTTP call, a database query) with a start time, end time, parent span, and attributes. A **context** carries the trace ID and span ID across service boundaries through headers. Together, they form a tree of spans that shows the full request lifecycle and timing.

---

### Q2. How do you propagate trace context in HTTP headers?

**Interview Answer**

The W3C Trace Context standard uses the `traceparent` header: `traceparent: 00-<trace-id>-<span-id>-<trace-flags>`. Use `opentelemetry-http` to automatically inject headers in outgoing requests and extract them in incoming requests. In axum, use `tower-http` with trace context propagation middleware. The `reqwest` client can be instrumented with OTel to automatically include trace headers. Ensure all services use the same propagation format.

---

### Q3. How do you instrument a Rust service with OpenTelemetry?

**Interview Answer**

Initialize the OTel tracer provider with `opentelemetry_sdk::trace::TracerProvider`, configure the exporter (Jaeger, OTLP), and install it as the global provider. Bridge with `tracing` using `tracing_opentelemetry::layer()` and `tracing_subscriber::registry()`. Use `#[tracing::instrument]` on functions to create spans automatically. Configure sampling to control trace volume. The `opentelemetry-otlp` crate handles exporting to OTel-compatible backends.

---

### Q4. What is trace sampling and why is it important?

**Interview Answer**

Sampling controls what percentage of traces are recorded. Always-on sampling records everything (expensive). Always-off records nothing. Probabilistic sampling records a configured percentage (e.g., 10% of requests). Adaptive sampling adjusts based on traffic volume. Head-based sampling makes decisions at trace creation; tail-based sampling decides after seeing the full trace (keeps error traces). Sampling reduces storage costs while maintaining visibility into problematic requests.

---

### Q5. How do you debug latency issues with distributed traces?

**Interview Answer**

Open the trace in Jaeger or Tempo and examine the span waterfall diagram. Look for spans with disproportionately long durations. Check span attributes for relevant details (query text, HTTP status, error messages). Compare traces from fast and slow requests to identify differences. Use trace comparison features to see where timing diverges. Look for cascading delays where one slow upstream call blocks downstream operations.

---

### Q6. How do you add custom attributes to trace spans?

**Interview Answer**

Use `tracing::span!` or `#[tracing::instrument]` with field parameters to add attributes: `tracing::info!(user_id = %user.id, order_id = %order.id, "processing order")`. These fields become OTel span attributes. Use `opentelemetry::Context::current().span().set_attribute()` for programmatic attribute setting. Add business-relevant attributes (user ID, order total, feature flags) that help with debugging. Be mindful of attribute cardinality — too many unique values increases storage costs.

---

### Q7. How do you handle tracing in async Rust code?

**Interview Answer**

`tracing` is async-aware — spans automatically track task context across `.await` points. Use `#[tracing::instrument]` on async functions. Be careful with `tokio::spawn` — spawned tasks need explicit span propagation with `.instrument(span)`. Use `tracing-futures` for future instrumentation. The `opentelemetry` crate's context propagation is also async-aware. Ensure span guards are held across `.await` points to maintain correct timing.

---

### Q8. How do you correlate traces with logs and metrics?

**Interview Answer**

Include the trace ID and span ID in all log entries using tracing span fields. Use the same trace ID as the `trace_id` label in Prometheus metrics. In Grafana, use the trace ID to pivot from metrics to traces to logs. OpenTelemetry's context propagation ensures trace context flows into logs automatically when using `tracing-opentelemetry`. This three-way correlation enables debugging from any starting point: metrics → traces → logs.

---

### Q9. How do you trace database queries in Rust?

**Interview Answer**

Instrument your database client with `tracing` to create spans for each query. Use `tracing::instrument` on repository functions. The `sqlx` crate supports tracing natively — enable the `tracing` feature to get automatic spans for queries. Include query parameters as span attributes (be careful with sensitive data). Record query duration as span attributes for easy identification of slow queries. Use span events for query results (rows affected, errors).

---

### Q10. How do you deploy and manage a trace backend in production?

**Interview Answer**

Deploy Jaeger or Grafana Tempo in Kubernetes with persistent storage. Use the OpenTelemetry Collector as a intermediary to buffer and transform traces before they reach the backend. Configure retention policies (7-30 days is common). Use object storage (S3, GCS) for cost-effective long-term trace storage with Tempo. Monitor the trace pipeline itself — if the collector drops traces, you lose visibility. Set up alerts on trace ingestion rates and storage usage.
