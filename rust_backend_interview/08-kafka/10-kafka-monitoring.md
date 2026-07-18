# Kafka Monitoring

## Interview Question

What key metrics should you monitor in a production Kafka cluster, and how do you set up alerting?

## Interview Answer

Critical Kafka metrics include broker health (under-replicated partitions, ISR shrink rate, request latency), consumer health (consumer lag, rebalance rate), producer health (produce latency, error rate, batch size), and cluster health (partition count, disk utilization, network throughput). Consumer lag is the most important metric as it directly impacts data freshness and SLAs. Monitoring is typically done via JMX metrics exported to Prometheus using Kafka Exporter, with dashboards in Grafana. Alerts should trigger on growing lag, ISR shrinkage, and broker disconnections.

---

## Follow-up Questions & Answers

### Q1. What is consumer lag and how do you monitor it at scale?

**Interview Answer**

Consumer lag is the difference between the latest offset in a partition and the consumer's committed offset. At scale, use Kafka Exporter (a Prometheus exporter) which exposes `kafka_consumergroup_lag` metrics per topic, partition, and consumer group. Grafana dashboards visualize lag trends over time. Alert when: lag grows monotonically for >5 minutes, lag exceeds SLA thresholds, or lag growth rate exceeds produce rate. In Rust services, expose per-partition lag via the `metrics` crate and a `/metrics` endpoint for Prometheus scraping.

---

### Q2. What are the most critical broker-level metrics to alert on?

**Interview Answer**

Critical broker metrics: (1) **Under-replicated partitions** - should be 0; any value >0 indicates replica lag; (2) **ISR shrink rate** - spikes indicate broker performance issues; (3) **Request handler idle ratio** - should be >0.3; low values indicate overload; (4) **Log flush latency** - high values indicate disk I/O issues; (5) **Active controller count** - should be exactly 1; (6) **Disk usage** - alert at 70-80% capacity. These metrics are available via JMX and should be scraped by Prometheus with Kafka Exporter or JMX Exporter.

---

### Q3. How do you set up Prometheus monitoring for Kafka?

**Interview Answer**

Deploy Kafka Exporter as a sidecar or standalone service exposing metrics on an HTTP port. Configure Prometheus to scrape the exporter's `/metrics` endpoint. Key metrics to scrape include `kafka_brokers`, `kafka_topic_partitions`, `kafka_consumergroup_lag`, and `kafka_consumergroup_members`. Use alerting rules in Prometheus Alertmanager for threshold-based alerts. For Rust services, expose application-level metrics (produce latency, consumer processing time) using the `metrics` crate and `metrics-exporter-prometheus`. Dashboards in Grafana provide real-time visibility into cluster health.

---

### Q4. How do you monitor Kafka in Kubernetes deployments?

**Interview Answer**

In Kubernetes, deploy Kafka Exporter as a Deployment or DaemonSet. Use Prometheus Operator with ServiceMonitor CRDs to automatically discover Kafka Exporter pods. Alert on under-replicated partitions, consumer lag, and broker availability. Use `kafka-statefulset` charts from Strimzi for managed Kafka on Kubernetes. Monitor broker pod resource usage (CPU, memory, disk) via Kubernetes metrics server. Use PodDisruptionBudgets to prevent simultaneous broker restarts. In Rust, deploy consumers as separate Kubernetes Deployments with HorizontalPodAutoscaler based on consumer lag metrics.

---

### Q5. What are the key metrics for Kafka producer performance in Rust services?

**Interview Answer**

Key producer metrics: (1) **Record send rate** - messages/second produced; (2) **Record error rate** - failed produce attempts; (3) **Request latency** - time from produce request to broker acknowledgment; (4) **Batch size** - average records per batch; (5) **Compression ratio** - effectiveness of compression; (6) **Buffer available ratio** - how full the producer buffer is. In `rdkafka`, these are available via the `Statistics` callback configured with `statistics.interval.ms`. Expose them via `metrics::counter!` and `metrics::histogram!` for Prometheus scraping.

---

### Q6. How do you detect and alert on consumer group health issues?

**Interview Answer**

Detect consumer group issues by monitoring: (1) **Consumer lag growth** - alert if lag grows continuously for >10 minutes; (2) **Rebalance rate** - alert if rebalances exceed threshold per hour; (3) **Consumer count** - alert if consumers drop below expected count; (4) **Processing latency** - alert if p99 exceeds SLA. Use Burrow (LinkedIn's consumer lag monitor) for lag-based alerting with configurable window checks. In Rust, implement health check endpoints that report per-partition lag and consumer group status. Alert via PagerDuty or Slack when consumer health degrades.

---

### Q7. What is the difference between Kafka metrics available via JMX vs the Metrics API?

**Interview Answer**

JMX (Java Management Extensions) is the traditional metrics source for Kafka, available on all JVM-based brokers and clients. The newer Kafka Metrics API provides a client-side metrics interface with lower overhead. For monitoring brokers, JMX is standard. For monitoring Rust services using `rdkafka`, the `statistics.interval.ms` callback provides librdkafka's internal metrics (similar to JMX). Both can be exported to Prometheus using appropriate exporters. JMX metrics are broker-centric; librdkafka statistics are client-centric and include per-topic, per-partition, and per-broker client-side metrics.

---

### Q8. How do you set up end-to-end latency monitoring for Kafka pipelines?

**Interview Answer**

End-to-end latency is the time from message production to consumer processing. Measure by: (1) embedding a timestamp in the message header at production time; (2) subtracting the produce timestamp from the consume timestamp at the consumer. Use `rdkafka`'s message timestamp (`timestamp()` method) for produce time. Track latency distribution (p50, p95, p99) using the `metrics::histogram!` macro. Alert when p99 latency exceeds SLA thresholds. In distributed systems, combine with OpenTelemetry for cross-service latency tracking. Kafka's own `record-age-ms` metric (via Kafka Exporter) provides broker-side age measurement.

---

### Q9. What dashboards should you have for Kafka monitoring?

**Interview Answer**

Essential dashboards: (1) **Broker Health** - under-replicated partitions, ISR count, request latency, disk usage; (2) **Topic Overview** - partition count, message rate per topic, byte throughput; (3) **Consumer Groups** - lag per group, rebalance count, consumer count; (4) **Producer Performance** - send rate, error rate, batch size, compression ratio; (5) **Cluster Overview** - broker count, controller status, network throughput. Use Grafana with Prometheus datasource. Pre-built dashboards from Confluent or community repos provide a starting point. Customize based on your SLA requirements.

---

### Q10. How do you handle Kafka monitoring in a multi-tenant environment?

**Interview Answer**

In multi-tenant Kafka, monitor per-tenant metrics: (1) per-tenant produce/consume rates; (2) per-tenant consumer lag; (3) per-tenant disk usage via topic-level metrics; (4) per-tenant quota violations. Use Kafka's built-in quotas (`producer_byte_rate`, `consumer_byte_rate`) to enforce resource limits. Tag metrics with tenant IDs in Prometheus labels for per-tenant dashboards and alerts. In Rust services, include tenant ID in Kafka message headers and use it as a metric label. Alert on tenants approaching quota limits to prevent noisy-neighbor issues. Per-tenant billing can be derived from metered produce/consume byte counts.
