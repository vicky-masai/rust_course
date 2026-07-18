# Logging

## Interview Question

Logging.

## Interview Answer

"I use structured JSON logging with request IDs, user IDs, timestamps, and error details."

---

## Follow-up Questions & Answers

### Q1. Why do you prefer structured JSON logging over plain text logs?

**Interview Answer**

Structured JSON logs can be parsed and queried efficiently by log aggregation tools like Elasticsearch, Loki, or CloudWatch. Each log entry is a JSON object with named fields like `request_id`, `user_id`, `status`, and `latency_ms`, making it easy to filter and search. Plain text logs require regex parsing which is fragile and slow at scale.

---

### Q2. How do you implement structured logging in a Rust/Axum project?

**Interview Answer**

I use the `tracing` crate with `tracing-subscriber` configured with `JsonLayer` to output structured logs. I create spans with `tracing::info_span!` for each request that include the request ID, method, and path. The `tracing` framework integrates naturally with Axum and Tokio, so async task context is preserved across await points.

---

### Q3. What information do you include in each log entry?

**Interview Answer**

Every log entry includes a timestamp, log level, message, and structured fields like `request_id`, `user_id`, `method`, `path`, `status_code`, and `latency_ms`. Error logs additionally include the error type, stack trace equivalent through `tracing` spans, and the relevant input that caused the failure. This makes debugging production issues fast without needing to reproduce the problem locally.

---

### Q4. How do you propagate request IDs across services and async tasks?

**Interview Answer**

I generate a UUID request ID at the middleware layer and store it in the tracing span. When spawning Tokio tasks, I clone the span so the request ID propagates automatically. For inter-service calls, I pass the request ID in the `X-Request-ID` header so downstream services include it in their logs, enabling end-to-end request tracing across the entire system.

---

### Q5. What is the difference between logging and tracing?

**Interview Answer**

Logging produces discrete events like "request completed" while tracing captures the full lifecycle of a request across multiple spans like "db_query", "cache_lookup", and "handler_execution". I use `tracing` in Rust because it supports both paradigms and maintains context through async task boundaries. Tracing is more powerful for performance analysis since you can see the latency breakdown of each operation.

---

### Q6. How do you handle log levels in production vs development?

**Interview Answer**

In development I use `RUST_LOG=debug` to get detailed output including SQL queries and cache hits. In production I set `RUST_LOG=info` to reduce noise while still capturing important events. I use `warn` for recoverable issues like rate limit hits and `error` for failures that need attention. Critical errors also trigger alerts through a separate pipeline.

---

### Q7. How do you prevent logging sensitive information like passwords or tokens?

**Interview Answer**

I implement `Serialize` on log structs but use `#[serde(skip)]` on sensitive fields like passwords, tokens, and API keys. Request bodies are logged at debug level only and sanitized before output. I also review all `.info!()` and `.error!()` macro calls to ensure no secrets leak into production logs where they could be accessed by unauthorized personnel.

---

### Q8. What log rotation strategy do you use in production?

**Interview Answer**

I rely on the container runtime or systemd to handle log rotation since Rust applications write to stdout in containerized environments. For non-containerized deployments, I use `logrotate` with daily rotation and a 30-day retention period. I also set maximum file sizes to prevent disk exhaustion, especially on instances with limited storage.

---
