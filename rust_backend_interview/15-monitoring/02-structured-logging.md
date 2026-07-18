# Structured Logging in Rust

## Interview Question

How do you implement structured logging in a Rust backend, and why is it important for production systems?

## Interview Answer

Structured logging outputs logs in a machine-parseable format (typically JSON) with consistent fields, replacing free-form text messages. In Rust, use the `tracing` crate with `tracing-subscriber` and a JSON formatter to produce structured logs. Each log entry includes fields like `timestamp`, `level`, `message`, `request_id`, `user_id`, and `service_name`. Structured logs enable powerful querying in tools like Loki, ELK, and Datadog. They also support correlation IDs to trace requests across distributed services.

---

## Follow-up Questions & Answers

### Q1. How do you configure `tracing-subscriber` for JSON logging?

**Interview Answer**

Use `tracing_subscriber::fmt().json().init()` for basic JSON output. For production, customize with `EnvFilter` for log level control, add fields like service name and version, and configure field sanitization. Use `tracing-bunyan-formatter` for Bunyan-compatible JSON that works with many log aggregators. Combine with `tracing-opentelemetry` to attach trace context to log entries automatically.

---

### Q2. What are correlation IDs and how do you propagate them?

**Interview Answer**

Correlation IDs (also called request IDs or trace IDs) uniquely identify a request across all services. Generate one at the API gateway or first service using `uuid::Uuid::new_v4()`. Pass it in HTTP headers (`X-Request-ID` or `traceparent`). Attach it to the tracing span so all log entries within the request include it. Return it in API responses so clients can reference it in support requests. Use `tracing::instrument` with span fields to automatically include the ID.

---

### Q3. What log levels should you use in production?

**Interview Answer**

Use `ERROR` for failures requiring immediate attention (data loss, service down). Use `WARN` for degraded conditions that aren't failures yet (high latency, retry succeeded). Use `INFO` for significant business events (user signup, order placed). Use `DEBUG` for detailed diagnostic information. Use `TRACE` for extremely verbose tracing. In production, default to `INFO` level and use `RUST_LOG` environment variable to adjust per-service without redeploying.

---

### Q4. How do you avoid logging sensitive data?

**Interview Answer**

Never log passwords, tokens, PII (emails, SSNs), or credit card numbers. Use the `secrecy` crate's `Secret<T>` wrapper which redacts values in Debug output. Implement log field sanitization to strip sensitive fields before output. Use structured logging to control exactly what fields are logged rather than string interpolation that might include secrets. Review log output regularly and use automated tools to detect PII in logs.

---

### Q5. How do you implement request-scoped logging in axum?

**Interview Answer**

Use axum middleware to create a tracing span for each request with the request ID, method, path, and remote address. All log entries within the request handler inherit these span fields. Use `tracing::instrument` on handler functions to add handler-specific context. The `tower-http` crate provides a `TraceLayer` that automatically creates request spans. This ensures every log line has request context for easy debugging.

---

### Q6. What is the difference between `log` and `tracing` crates?

**Interview Answer**

The `log` crate provides a simple logging facade with levels (error, warn, info, debug, trace) but no structural context. `tracing` is a superset that adds hierarchical spans, structured fields, and async-aware instrumentation. `tracing` supports distributed tracing context propagation, while `log` doesn't. New Rust projects should use `tracing` — it's the community standard and integrates with OpenTelemetry. The `log` crate is still used by many libraries but `tracing` provides a compatibility bridge.

---

### Q7. How do you test that logs are emitted correctly?

**Interview Answer**

Use `tracing-subscriber`'s test utilities to capture log output in tests. Create a test subscriber with `with_test_defaults()` and verify that specific log lines are emitted. For structured logs, parse the JSON output and assert on specific fields. Use `tracing_test` crate for simpler test setup. Verify that error conditions emit ERROR-level logs with appropriate context. Test that sensitive data is properly redacted in log output.

---

### Q8. How do you handle log rotation and retention?

**Interview Answer**

Configure log rotation at the collection layer (Filebeat, Fluentd, Promtail) rather than in the application. Use time-based rotation (daily) and size-based rotation as a backup. Set retention policies based on compliance requirements — 30 days for debug logs, 90 days for info, 1 year for audit logs. Use log levels to control retention — keep ERROR logs longer than TRACE logs. For containerized applications, write to stdout/stderr and let the container runtime handle rotation.

---

### Q9. How do you structure log fields for effective querying?

**Interview Answer**

Use consistent field names across all services: `timestamp`, `level`, `message`, `service`, `trace_id`, `span_id`, `user_id`, `request_id`. Use nested objects for related data: `http.method`, `http.status_code`, `db.duration_ms`. Follow a naming convention (snake_case for fields, dot notation for hierarchy). Include enough context to debug without accessing other systems. Avoid putting variable-length data in field names — use consistent keys with variable values.

---

### Q10. How do you implement audit logging in Rust?

**Interview Answer**

Audit logs track who did what and when for compliance purposes. Create a dedicated audit log format with fields: `actor_id`, `action`, `resource_type`, `resource_id`, `timestamp`, `ip_address`, `result`. Use a separate log stream or file for audit data. Never modify or delete audit logs — write them immutably. Use `tracing` with a custom layer that filters for audit events. Store audit logs in a tamper-evident system for compliance with SOC2, GDPR, and similar regulations.
