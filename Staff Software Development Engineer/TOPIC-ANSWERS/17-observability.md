# LEVEL 17 — Observability

### 0334. tracing

Rust crate ecosystem for structured, contextual diagnostics — spans and events. Propagate request IDs across async tasks. Foundation of serious Rust service telemetry.

**Talk track:** *"tracing gives spans/events with context — the Rust-native observability spine."*

---

### 0335. Structured Logging

Log as key-value fields (JSON), not free-text only. Enables search/filter in aggregators. Include request_id, user_id (careful PII), error codes.

**Talk track:** *"Structured logs are queryable data — stop grepping poetry."*

---

### 0336. OpenTelemetry

Vendor-neutral standard for traces, metrics, logs export. Instrument once; send to Jaeger/Grafana/Datadog/etc.

**Talk track:** *"OpenTelemetry is the portable instrumentation API — avoid vendor lock-in at the code layer."*

---

### 0337. Metrics

Numeric measurements over time: counters, gauges, histograms. RED (Rate, Errors, Duration) and USE (Utilization, Saturation, Errors) methods guide what to measure.

**Talk track:** *"Metrics tell you what is happening at aggregate speed — SLOs live here."*

---

### 0338. Prometheus

Pull-based metrics system with a powerful query language (PromQL). Industry default for cloud-native metrics. Label cardinality can explode — design labels carefully.

**Talk track:** *"Prometheus scrapes metrics and PromQL alerts on them — watch cardinality."*

---

### 0339. Grafana

Dashboards and visualization over Prometheus and other datasources. Make golden signals visible; avoid dashboard sprawl without owners.

**Talk track:** *"Grafana is how humans see metrics — dashboards need owners and purpose."*

---

### 0340. Jaeger

Distributed tracing UI/backend (OpenTelemetry compatible lineage). Follow a request across services via trace IDs.

**Talk track:** *"Jaeger visualizes request journeys across microservices."*

---

### 0341. Distributed Tracing

Propagate context (trace/span IDs) across process boundaries. Answers "where did time go?" in microservice calls. Sample wisely to control cost.

**Talk track:** *"Tracing connects the story of one request across many services."*
