# tracing

## Interview Question

Explain tracing.

## Interview Answer

"`tracing` provides structured, asynchronous-aware logging with spans and events, making distributed request tracking easier."

---

## Follow-up Questions & Answers

### Q1. What is the difference between spans and events in tracing?

**Interview Answer**

Spans represent a period of time during execution, like a function call or request handler, and can contain key-value metadata. Events are point-in-time occurrences within a span, like a log message or error. Spans form a tree structure that shows the call hierarchy, while events provide granular details within that context.

---

### Q2. How do you integrate tracing with Axum?

**Interview Answer**

Use `tracing-subscriber` with the `EnvFilter` and `fmt` layers to initialize tracing at application startup. Axum automatically instruments handlers with spans when using `axum::extract::Request` with `tower-http::trace`. Add `#[instrument]` to handler functions to create spans with request metadata automatically.

---

### Q3. How does tracing work across async task boundaries?

**Interview Answer**

tracing uses thread-local span storage that propagates across `.await` points via the `Span::enter()` mechanism. When spawning new Tokio tasks, use `Span::current()` and enter it in the new task to maintain context. This ensures logs from async tasks are associated with the correct request span.

---

### Q4. What is distributed tracing and how does tracing support it?

**Interview Answer**

Distributed tracing tracks requests across multiple services using trace IDs and span IDs. The `tracing-opentelemetry` crate exports spans to Jaeger or Zipkin for visualization. Propagate trace context in HTTP headers using `traceparent` to correlate requests across your Axum services.

---

### Q5. How do you filter tracing output effectively?

**Interview Answer**

Use `EnvFilter` to set log levels per module, like `my_app=debug,sqlx=warn`. Configure via the `RUST_LOG` environment variable for runtime flexibility. In production, use info level for most modules and debug only for specific handlers experiencing issues.

---

### Q6. What is the performance overhead of tracing in production?

**Interview Answer**

tracing has minimal overhead when spans are disabled at the current level, as it short-circuits evaluation. Enabled spans add 1-5 microseconds per span creation and event emission. Use sampling in distributed tracing to reduce overhead in high-throughput Axum applications.

---

### Q7. How do you visualize tracing output from an Axum backend?

**Interview Answer**

Export traces to Jaeger using `tracing-opentelemetry` and `opentelemetry-jaeger` crates. Use `tracing-subscriber` with the `json` feature to output structured logs for ELK or Datadog. For local development, use the `fmt` layer with pretty-printed output and colored spans.

---

### Q8. How do you trace database queries with sqlx and tracing?

**Interview Answer**

sqlx automatically creates tracing spans for each query when the `tracing` feature is enabled in Cargo.toml. Use `sqlx::query!().instrument(span)` to add custom metadata to query spans. This helps correlate slow queries with specific request handlers in your Axum application.
