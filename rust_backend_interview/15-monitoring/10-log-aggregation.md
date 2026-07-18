# Log Aggregation

## Interview Question

How do you aggregate and manage logs from multiple Rust backend services in a production environment?

## Interview Answer

Use a log aggregation pipeline: Rust services write structured logs to stdout/stderr using `tracing`, a log shipper (Promtail, Fluentd, Filebeat) collects and forwards them, and a storage backend (Loki, Elasticsearch) indexes and stores them. For Kubernetes, Promtail runs as a DaemonSet collecting container logs. The ELK stack (Elasticsearch, Logstash, Kibana) provides powerful full-text search. Grafana Loki is a lightweight alternative that indexes labels rather than full text, reducing storage costs. Centralized logs enable searching across services and correlating events.

---

## Follow-up Questions & Answers

### Q1. What is the ELK stack and how does it work?

**Interview Answer**

ELK stands for Elasticsearch (search and storage), Logstash (processing and transformation), and Kibana (visualization). Filebeat ships logs to Logstash, which parses, enriches, and forwards to Elasticsearch. Kibana provides search, dashboards, and alerting. The ELK stack offers powerful full-text search and complex aggregations but requires significant resources (Elasticsearch clusters are expensive). For simpler use cases, Grafana Loki is a lighter alternative.

---

### Q2. How does Grafana Loki differ from Elasticsearch for log aggregation?

**Interview Answer**

Loki indexes only labels (metadata), not full log content, making it much cheaper to store. It uses LogQL (PromQL-inspired) for querying. Elasticsearch indexes the full text, enabling powerful search but requiring more storage and compute. Loki integrates natively with Grafana and Prometheus. For most backend services, Loki is sufficient and more cost-effective. Use Elasticsearch when you need complex full-text search or analytics on log content.

---

### Q3. How do you configure log shipping in Kubernetes?

**Interview Answer**

Deploy Promtail as a DaemonSet (one pod per node) that reads container logs from `/var/log/pods/`. Configure Promtail to parse structured logs and add Kubernetes metadata labels (namespace, pod, container). Use `pipeline_stages` to extract fields from JSON logs. For Fluentd, use the fluentd DaemonSet with the kubernetes metadata plugin. Ensure log files have appropriate retention at the filesystem level. Monitor the log pipeline itself — if Promtail fails, you lose logs.

---

### Q4. How do you implement log rotation for Rust services?

**Interview Answer**

For containerized services, write to stdout/stderr and let the container runtime handle rotation (Docker's `--log-opt max-size` and `max-file`). For non-containerized deployments, use `tracing-appender` with rolling file appenders: `RollingFileAppender::builder().rotation(Rotation::DAILY).filename_prefix("app").build("logs/")`. Configure maximum file size and retention period. Use log levels to separate files: error.log, app.log, debug.log. Ensure rotation doesn't lose logs during the switch.

---

### Q5. How do you set up log retention policies?

**Interview Answer**

Define retention based on log level and compliance requirements: ERROR logs retained for 90 days, INFO for 30 days, DEBUG for 7 days. In Loki, configure `retention_period` in `limits_config`. In Elasticsearch, use Index Lifecycle Management (ILM) to move old indices to cheaper storage and delete after retention period. Archive critical logs to object storage (S3) for long-term compliance. Monitor storage usage and adjust retention as data volume grows.

---

### Q6. How do you search logs effectively across multiple services?

**Interview Answer**

Use consistent structured log formats across all services with common fields (service, timestamp, level, trace_id). In Loki, use LogQL queries: `{service="auth"} |= "login failed" | json | user_id="123"`. In Elasticsearch, use Kibana's query DSL. Correlate across services using trace IDs — find all logs for a specific request. Create saved searches for common debugging scenarios. Build dashboards that combine log patterns with metrics for comprehensive debugging.

---

### Q7. How do you handle high-volume log ingestion?

**Interview Answer**

Buffer logs at the shipper level (Promtail, Fluentd) with batch sending. Use compression (gzip) for log transport. Sample debug-level logs if volume is excessive. Use async I/O for log writing to avoid blocking application code. Monitor ingestion rate and set up alerts for log drops. Consider tiered storage: hot (SSD) for recent logs, warm (HDD) for older, cold (object storage) for archived. Use log deduplication for repeated error messages.

---

### Q8. How do you test log aggregation in development?

**Interview Answer**

Run a local Loki + Grafana stack using Docker Compose for development. Use `tracing` with `fmt::Layer` for local console output during development. Test structured log parsing by writing to a local file and verifying Promtail picks it up. Use `tracing-test` crate to capture and assert on logs in unit tests. For integration tests, use the `loki-rs` crate or mock the log shipping endpoint. Verify log formats with sample data before deploying to production.

---

### Q9. How do you implement log-based alerting?

**Interview Answer**

In Grafana Loki, create alert rules using LogQL: `{service="auth"} |= "login failed" | line_format "{{.count}}" | unwrap count | count_over_time[5m] > 100`. Configure alertmanager integration to send notifications. In Elasticsearch, use Watcher or ElastAlert for log-based alerts. Alert on error rate spikes, specific error messages, or anomalous log patterns. Combine log alerts with metric-based alerts for comprehensive monitoring.

---

### Q10. How do you ensure log quality and consistency across teams?

**Interview Answer**

Create a logging standards document specifying: log format (JSON), required fields (timestamp, level, message, service, trace_id), naming conventions, and what not to log (secrets, PII). Provide a shared logging library or configuration template. Use linting tools to validate log formats. Review log output during code reviews. Monitor log quality metrics: field completeness, format consistency, and volume trends. Conduct regular log audits to identify and fix logging issues.
