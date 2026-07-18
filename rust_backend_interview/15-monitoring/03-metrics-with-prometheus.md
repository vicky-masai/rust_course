# Metrics with Prometheus

## Interview Question

How do you expose and collect metrics from a Rust backend service using Prometheus?

## Interview Answer

Use the `metrics` crate as a facade for recording metrics, and `metrics-exporter-prometheus` to expose them in Prometheus exposition format. Define counters (request count), gauges (connection pool size), and histograms (request latency). Expose a `/metrics` endpoint that Prometheus scrapes on a configured interval. Use labels to differentiate metric dimensions (method, path, status code). The Prometheus data model uses time-series data with labels as the unique key for each series.

---

## Follow-up Questions & Answers

### Q1. What are the three main Prometheus metric types?

**Interview Answer**

**Counter** is a monotonically increasing value (request count, error count). It can only go up or reset to zero on process restart. **Gauge** is a value that can go up and down (queue size, memory usage, temperature). **Histogram** samples observations into configurable buckets (request duration). Histograms provide count, sum, and quantile calculations. Use counters for rates, gauges for current state, and histograms for distributions.

---

### Q2. How do you configure Prometheus to scrape a Rust service?

**Interview Answer**

In your Prometheus configuration, add a scrape job pointing at your service's `/metrics` endpoint. Set `scrape_interval` (typically 15s-30s). Use `metrics-exporter-prometheus` to create the HTTP endpoint: `PrometheusBuilder::new().install_recorder()`. Configure the listen address with `PrometheusBuilder::new().with_http_listener(addr)`. The endpoint returns text-format metrics by default. Use service discovery (Kubernetes, Consul) for dynamic endpoint management.

---

### Q3. How do you use labels effectively in Prometheus metrics?

**Interview Answer**

Labels add dimensions to metrics: `http_requests_total{method="GET", path="/api/users", status="200"}`. Use labels for high-cardinality-low-variation dimensions (method, status code, endpoint). Avoid high-cardinality labels (user IDs, request IDs) as they explode the time-series count. Keep label values bounded — a label with 1000 unique values creates 1000 time series per metric. Use `_total` suffix for counters and consistent naming across services.

---

### Q4. What is the Prometheus exposition format?

**Interview Answer**

Prometheus exposition format is a text-based format where each line contains a metric name, optional labels, and a value with a timestamp. Example: `http_requests_total{method="GET"} 1234 1625000000000`. Histograms include `_bucket`, `_sum`, and `_count` lines. The format is simple, human-readable, and easy to parse. Use `metrics-exporter-prometheus` to generate this format automatically. For high-throughput services, consider the OpenMetrics or protobuf formats.

---

### Q5. How do you create meaningful Prometheus dashboards for Rust services?

**Interview Answer**

Dashboard panels should show request rate (rate of counter), error percentage (errors/total), latency percentiles (histogram_quantile), and resource usage (gauges). Use PromQL queries like `rate(http_requests_total[5m])` for request rate and `histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m]))` for p99 latency. Create service-specific dashboards and an overview dashboard showing all services. Use template variables for service selection and time range.

---

### Q6. How do you instrument database queries with Prometheus metrics?

**Interview Answer**

Record metrics for query count, duration, and errors. Use labels for operation type (select, insert, update, delete) and table name. Wrap database calls with a metrics layer or middleware. Use histograms for query duration to track latency distributions. Monitor connection pool metrics: active connections, idle connections, wait time. These metrics help identify slow queries, connection exhaustion, and database bottlenecks. The `metrics` crate makes it easy to add instrumentation without cluttering business logic.

---

### Q7. How do you handle metric cardinality explosions?

**Interview Answer**

Cardinality explosion occurs when label values create too many unique time series. Prevent it by bounding label values: use endpoint patterns (`/api/users/{id}`) instead of actual paths, limit status code labels to 5xx/4xx/2xx categories, and avoid user-specific labels. Monitor your metric count using `prometheus_tsdb_head_series`. Use metric recording rules in Prometheus to pre-aggregate high-cardinality data. Set alerts on metric count growth rates.

---

### Q8. How do you test Prometheus metrics in Rust?

**Interview Answer**

Use the `metrics` crate's test utilities to capture metrics without exporting them. Verify that specific metrics are recorded with expected values after calling your code. For integration tests, query the `/metrics` endpoint and parse the output. Use `prometheus::TextFormat::parse` to validate metric output format. Test that labels are correct and metric values increment as expected. Ensure metrics are reset between tests or use unique metric names.

---

### Q9. What are recording rules in Prometheus?

**Interview Answer**

Recording rules pre-compute frequently used or expensive PromQL queries and save the result as a new metric. For example, pre-compute `job:http_requests_total:rate5m` from the raw rate query. This reduces dashboard load time and Prometheus resource usage. Use recording rules for complex aggregations, cross-service calculations, and SLO error budget computations. Define rules in Prometheus config files and reload with `kill -HUP` or the API.

---

### Q10. How do you use Prometheus for SLO monitoring?

**Interview Answer**

Define SLIs (Service Level Indicators) as Prometheus metrics: request success rate and latency. Create recording rules for SLI calculations: `sum(rate(http_requests_total{status!~"5.."}[5m])) / sum(rate(http_requests_total[5m]))`. Use Prometheus alerting rules to trigger when error budgets are consumed. The `prometheus-slo` crate helps define SLOs programmatically. Combine with Grafana SLO dashboards for visibility into error budget status and burn rate.
