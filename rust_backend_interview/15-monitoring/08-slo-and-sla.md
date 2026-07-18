# SLO, SLA, and SLI

## Interview Question

What are SLOs, SLAs, and SLIs, and how do you implement SLO-based monitoring for a Rust backend?

## Interview Answer

**SLIs** (Service Level Indicators) are quantitative metrics measuring service behavior — request success rate, latency percentiles, throughput. **SLOs** (Service Level Objectives) are target values for SLIs — "99.9% of requests succeed" or "p99 latency under 200ms." **SLAs** (Service Level Agreements) are contractual commitments with consequences (credits, termination) for missing SLOs. Implement SLO monitoring by recording SLI values in Prometheus, computing error budgets, and alerting when budgets are consumed too quickly. SLOs let you make data-driven decisions about reliability investment.

---

## Follow-up Questions & Answers

### Q1. How do you define good SLOs for a backend service?

**Interview Answer**

Start with user expectations — what level of reliability do users need? Common targets: 99.9% (three nines, ~8.7 hours downtime/year), 99.95%, 99.99% (four nines, ~52 minutes/year). Choose SLIs that directly measure user experience: availability (successful requests/total), latency (p99 under threshold), and correctness (correct responses/total). Don't set SLOs higher than you can sustain — each additional nine is exponentially harder and more expensive.

---

### Q2. What is an error budget and how do you calculate it?

**Interview Answer**

Error budget = 1 - SLO target. For a 99.9% availability SLO, the error budget is 0.1% — you can have 0.1% of requests fail before violating the SLO. Calculate it over a rolling window (30 days is common): `error_budget_remaining = (SLO_target - actual_success_rate) / (1 - SLO_target)`. When the error budget is depleted, stop deploying new features and focus on reliability improvements. Track error budget consumption rate to predict when you'll run out.

---

### Q3. How do you implement SLO monitoring with Prometheus?

**Interview Answer**

Record SLI values using `metrics` crate counters for total requests and successful requests. Use recording rules to pre-compute SLO compliance: `sum(rate(http_requests_total{status!~"5.."}[5m])) / sum(rate(http_requests_total[5m]))`. Create alerts for error budget burn rate — fast burn (14.4x over 5 min) and slow burn (1x over 6 hours). Build Grafana dashboards showing error budget remaining, burn rate, and historical SLI trends. Use `prometheus-slo` crate for programmatic SLO definitions.

---

### Q4. What is the difference between availability and reliability SLOs?

**Interview Answer**

Availability SLOs measure the percentage of time the service is accessible: "99.9% uptime." Reliability SLOs measure the quality of service: "99% of requests return correct results within 200ms." Reliability is more comprehensive because a service can be "available" but returning errors or being slow. Most modern SLOs focus on reliability rather than simple uptime. A service at 100% uptime but 50% error rate is not reliable.

---

### Q5. How do you handle multiple SLOs for different endpoints?

**Interview Answer**

Define different SLO tiers based on criticality: payment endpoints might have 99.99% SLO while analytics endpoints have 99.9%. Create separate SLI metrics per endpoint or group. Compute error budgets per endpoint. Use weighted averages for overall service SLO. Some teams define SLIs at the service level (aggregate) and endpoint level (specific). The most critical endpoints drive the overall SLO target.

---

### Q6. What happens when you exhaust your error budget?

**Interview Answer**

When the error budget is depleted, prioritize reliability over feature development. Freeze non-critical deployments. Invest in testing, monitoring, and infrastructure improvements. Conduct post-mortems for incidents that consumed budget. Communicate with stakeholders about reliability priorities. Some teams formalize this as "reliability sprint" where the team focuses solely on reducing error rates. The budget should recover naturally as good performance over time replenishes it.

---

### Q7. How do you communicate SLO status to stakeholders?

**Interview Answer**

Create executive-level dashboards showing SLO compliance, error budget status, and trends. Use red/yellow/green status indicators. Generate weekly SLO reports summarizing performance. During incidents, communicate expected error budget impact. For engineering teams, provide detailed dashboards with burn rate and SLI breakdowns. Use SLO status in sprint planning to inform deployment risk decisions. Some organizations tie SLO performance to team OKRs.

---

### Q8. How do you test that your SLO monitoring is working?

**Interview Answer**

Inject failures in staging to verify alerts fire at the correct burn rate. Use chaos engineering to simulate outages and confirm SLO dashboards reflect the impact. Test alert routing to ensure the right team is notified. Review past incidents to verify SLO monitoring would have detected the issue. Run "game days" where you practice responding to SLO-based alerts. Validate recording rules with `promtool test rules` using sample data.

---

### Q9. What are the common mistakes when defining SLOs?

**Interview Answer**

Setting SLOs too high (99.999% is unsustainable for most teams). Defining SLOs based on internal metrics rather than user experience. Not having SLOs at all — "we'll alert on everything" leads to alert fatigue. Using SLOs as performance targets instead of reliability commitments. Not updating SLOs as the service evolves. Choosing SLIs that are easy to measure but don't reflect user experience. Not involving product and business teams in SLO definition.

---

### Q10. How do you use SLOs for capacity planning?

**Interview Answer**

Track SLI trends over time to identify gradual degradation that might indicate capacity limits approaching. When latency SLOs are consistently close to targets, it may indicate the service needs scaling. Use error budget trends to predict when capacity upgrades are needed. Combine SLO data with resource utilization metrics to identify bottlenecks. SLO-based capacity planning is more meaningful than arbitrary utilization thresholds because it ties capacity decisions to user impact.
