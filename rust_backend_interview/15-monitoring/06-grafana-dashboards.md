# Grafana Dashboards

## Interview Question

How do you design and build effective Grafana dashboards for monitoring a Rust backend service?

## Interview Answer

Grafana dashboards visualize metrics from Prometheus, logs from Loki, and traces from Tempo in a unified interface. Design dashboards around use cases (service health, SLO tracking, resource utilization) rather than dumping all available metrics. Start with a high-level overview panel showing request rate, error rate, and latency. Drill down into detailed panels for debugging. Use variables for service selection, environment, and time range. Keep dashboards focused — a dashboard with too many panels is harder to use during incidents.

---

## Follow-up Questions & Answers

### Q1. What makes a good Grafana dashboard?

**Interview Answer**

A good dashboard tells a story: top-level health at the top, detailed metrics below. Use consistent panel sizing and layout. Group related panels logically (request metrics, resource metrics, error metrics). Include links to runbooks and related dashboards. Use color coding consistently (green = healthy, yellow = warning, red = critical). Keep the most important panels "above the fold" without scrolling. Test by asking: "Can an engineer diagnose a common issue in under 2 minutes?"

---

### Q2. How do you use Grafana variables for dynamic dashboards?

**Interview Answer**

Define variables in dashboard settings with queries like `label_values(http_requests_total, service)` to populate dropdowns. Reference variables in panel queries as `$service`. Variables can be single-value, multi-value, or all. Use chained variables where one variable filters the options of another (environment → service → endpoint). Custom variables can list fixed options like deployment regions. This makes one dashboard work across all services and environments.

---

### Q3. What are the key panels for a Rust backend service dashboard?

**Interview Answer**

Include request rate (QPS), error rate (%), latency percentiles (p50, p95, p99), saturation metrics (CPU, memory, connections), dependency health (database, cache, external APIs), and SLO status (error budget remaining). Add panels for tokio task count, runtime metrics, and garbage collection (if using a mixed runtime). Include deployment markers to correlate changes with metric shifts. Use row panels to group and collapse sections.

---

### Q4. How do you create effective Prometheus queries in Grafana?

**Interview Answer**

Use `rate()` for counters over a reasonable window (5m for stable rates). Use `histogram_quantile(0.99, rate(...))` for latency percentiles. Use `sum by (label)` for aggregations. Apply `increase()` for counter increments over specific periods. Use `predict_linear()` for capacity forecasting. Test queries in Prometheus's expression browser before adding to Grafana. Use recording rules for complex queries to improve dashboard performance.

---

### Q5. How do you set up Grafana alerts within dashboards?

**Interview Answer**

Click on a panel title → Alert → Create Alert. Define conditions using the panel's query (e.g., error rate > 1% for 5 minutes). Set evaluation intervals, notification channels (Slack, PagerDuty, email), and severity labels. Use Grafana's unified alerting for alert rules, contact points, and notification policies. Test alerts by clicking "Test" before saving. Link alert annotations to dashboards so incidents appear as markers on time-series panels.

---

### Q6. How do you design dashboards for SLO monitoring?

**Interview Answer**

Create a dedicated SLO dashboard showing: current SLI value, target SLI, error budget remaining (%), error budget burn rate, and historical SLI trend. Use the `slo` panel plugin or custom panels for error budget visualization. Show burn rate alerts as annotations. Include links to detailed service dashboards for debugging. Use multi-service SLO dashboards for platform teams. Color-code error budget status: green (>50% remaining), yellow (25-50%), red (<25%).

---

### Q7. How do you share and manage dashboards across teams?

**Interview Answer**

Store dashboard JSON in Git for version control. Use Grafana's provisioning to deploy dashboards from Git repositories. Create dashboard folders organized by team or service. Use template dashboards that teams can customize. Share dashboards via links or embed them in internal documentation. Use Grafana's API to automate dashboard creation. Implement review processes for dashboard changes, similar to code reviews.

---

### Q8. How do you use annotations in Grafana dashboards?

**Interview Answer**

Annotations overlay markers on time-series panels to show events like deployments, configuration changes, or incidents. Create annotations from Prometheus alerts, manual entries, or external sources. Use `grafana.annotate()` API to programmatically add annotations. Correlate deployments with metric changes by annotating release times. Annotate incidents so engineers can see historical context. This helps identify the root cause of metric changes quickly.

---

### Q9. How do you optimize Grafana dashboard performance?

**Interview Answer**

Use recording rules for complex PromQL queries instead of running them at dashboard load time. Limit the number of panels per dashboard (20-30 max). Use template variables to reduce query scope. Set appropriate time ranges — don't query 30 days when 1 hour suffices. Use panel queries efficiently: avoid expensive aggregations and use `max_data_points` to limit resolution. Cache frequent queries with Grafana's server-side caching.

---

### Q10. How do you implement multi-service overview dashboards?

**Interview Answer**

Create a top-level dashboard with a table showing all services: name, request rate, error rate, p99 latency, CPU usage, and SLO status. Use `label_values()` for service discovery. Each row links to the service-specific dashboard. Use conditional coloring to highlight unhealthy services. Add sparkline charts in table cells for trend visibility. Use Grafana's library panels to maintain consistent component panels across dashboards. This overview serves as the "home base" for operations teams.
