# Observability Overview

## Interview Question

What are the three pillars of observability, and how do they work together to help you understand system behavior?

## Interview Answer

The three pillars of observability are **logs**, **metrics**, and **traces**. Logs provide discrete event records with contextual information about what happened. Metrics are numerical measurements over time that show trends and patterns (request rate, error rate, latency). Traces follow a single request as it flows through distributed services, showing timing and dependencies. Together, they provide a complete picture: metrics tell you something is wrong, logs tell you why, and traces tell you where the problem is in the request path.

---

## Follow-up Questions & Answers

### Q1. What is the difference between monitoring and observability?

**Interview Answer**

Monitoring answers known questions — "is the server up?" using predefined dashboards and alerts. Observability answers unknown questions — "why are users in Brazil experiencing slow responses?" by allowing you to ask arbitrary questions of your system's internal state. A well-observed system lets you debug novel problems without deploying new code. Monitoring is a subset of observability focused on known failure modes.

---

### Q2. How do you implement observability in a Rust backend?

**Interview Answer**

Use `tracing` for structured logs and distributed traces, `metrics` crate for counters/histograms/gauges, and export to backends like Prometheus (metrics), Loki (logs), and Jaeger (traces). The `tracing` crate is the standard for Rust observability — it provides hierarchical span-based logging that supports both logs and traces. Export metrics with `metrics-exporter-prometheus` and traces with `tracing-opentelemetry` to Jaeger.

---

### Q3. What is the RED method for monitoring?

**Interview Answer**

RED stands for Rate (requests per second), Errors (error rate), and Duration (latency distribution). It focuses on the three most important signals for user-facing services. Apply RED to each microservice endpoint: measure request rate, track error percentages, and record latency percentiles (p50, p95, p99). This method provides quick insight into service health without overwhelming dashboards with irrelevant metrics.

---

### Q4. What is the USE method for monitoring resources?

**Interview Answer**

USE stands for Utilization, Saturation, and Errors for each resource (CPU, memory, disk, network). Utilization is the percentage of time the resource is busy. Saturation is the queue depth or number of waiting requests. Errors are resource-specific error counts. Apply USE to infrastructure monitoring: CPU utilization, memory saturation (swap usage), disk errors, and network saturation. USE complements RED by focusing on resource health rather than request health.

---

### Q5. How do you choose between different observability backends?

**Interview Answer**

Choose based on your team's expertise, scale, and cost. Prometheus + Grafana is the standard for metrics. For logs, Loki integrates well with Grafana, while ELK (Elasticsearch, Logstash, Kibana) offers more powerful search. For traces, Jaeger or Tempo work well with Grafana. OpenTelemetry is vendor-agnostic and lets you switch backends. Managed services (Datadog, New Relic, Honeycomb) reduce operational burden but increase cost.

---

### Q6. What is OpenTelemetry and why is it important?

**Interview Answer**

OpenTelemetry (OTel) is a vendor-neutral standard for collecting telemetry data (logs, metrics, traces). It provides SDKs for many languages including Rust. OTel lets you instrument once and export to multiple backends. It provides a unified API for context propagation, trace creation, and metric collection. Using OTel avoids vendor lock-in and simplifies multi-service observability in distributed systems.

---

### Q7. How do you structure your observability stack for microservices?

**Interview Answer**

Instrument all services with OpenTelemetry for consistent telemetry format. Use a central collector (OTel Collector) to receive, process, and export telemetry to backends. Ensure all services propagate trace context in headers. Use consistent naming conventions for metrics and logs across services. Deploy a unified dashboarding layer (Grafana) that correlates logs, metrics, and traces. Set up alerting based on SLOs rather than individual metrics.

---

### Q8. What are golden signals in observability?

**Interview Answer**

The golden signals (from Google SRE) are Latency (time to serve a request), Traffic (demand on the system), Errors (rate of failed requests), and Saturation (how full the system is). They provide a standardized way to assess service health. Focus dashboards and alerts on these four signals for each service. They align closely with RED and together provide comprehensive service-level monitoring.

---

### Q9. How do you handle observability costs at scale?

**Interview Answer**

Sample traces rather than recording every request (1-10% for low-traffic services, lower for high-traffic). Use log level filtering to reduce volume. Set retention policies based on data age — keep recent data granular, aggregate older data. Use tiered storage (hot/warm/cold) for different data ages. Aggregate metrics at the edge before sending to backends. Review and prune unused dashboards, alerts, and metric series regularly.

---

### Q10. How do you correlate logs, metrics, and traces?

**Interview Answer**

Use trace IDs as the common identifier across all three pillars. Include trace IDs in log messages so you can find all logs for a request. Link metrics to traces by recording metrics within trace spans. Use Grafana's explore feature to pivot between logs, metrics, and traces. OpenTelemetry's context propagation ensures trace context flows across services and into all telemetry types. Structured logging with trace IDs makes correlation automatic.
