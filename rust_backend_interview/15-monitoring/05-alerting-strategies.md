# Alerting Strategies

## Interview Question

How do you design an effective alerting strategy for a backend service, and what are the common pitfalls?

## Interview Answer

Effective alerting focuses on symptoms (user impact) rather than causes (individual metrics). Define alerts based on SLO violations — when error budgets are being consumed too quickly. Use multi-window, multi-burn-rate alerts to reduce false positives. Structure alerts with severity levels (critical, warning, info) and include runbooks with actionable steps. The goal is to page humans only for user-impacting issues, while lower-severity issues are handled during business hours or through automation.

---

## Follow-up Questions & Answers

### Q1. What is the difference between symptom-based and cause-based alerts?

**Interview Answer**

Symptom-based alerts detect user impact directly: "error rate above 1% for 5 minutes." Cause-based alerts detect component failures: "CPU above 90%." Symptom alerts are better because they directly reflect user experience and reduce noise — a high CPU alert might fire during a batch job that doesn't affect users. Cause-based alerts create alert fatigue. Always prefer symptom alerts and use cause-based alerts only for capacity planning.

---

### Q2. What are the different alert severity levels?

**Interview Answer**

**Critical** (P1): User-facing outage, data loss, security incident — page immediately with PagerDuty/Opsgenie. **Warning** (P2): Degraded performance, approaching thresholds — notify during business hours via Slack/email. **Info** (P3): Noteworthy events, deployment notifications — log and dashboard only. Define clear criteria for each level and document the expected response time. Review and adjust severity levels based on incident history.

---

### Q3. How do you write effective alert descriptions and runbooks?

**Interview Answer**

Include what the alert means, why it matters, and what to do about it. Format: "Alert Name: [what is detected]. Impact: [user effect]. Steps: [1. check dashboard X, 2. look for Y, 3. escalate if Z]". Link to dashboards, documentation, and escalation paths. Use consistent formatting across all alerts. Include the affected service, environment, and duration. Write runbooks for common alerts so new on-call engineers can handle them without senior assistance.

---

### Q4. What is PagerDuty and how do you integrate it with Rust services?

**Interview Answer**

PagerDuty is an incident management platform that routes alerts to on-call engineers via multiple channels (phone, SMS, push). Integrate by configuring Prometheus Alertmanager to send webhooks to PagerDuty. Set up escalation policies, schedules, and notification rules. Use the PagerDuty API to create incidents programmatically from your application when critical errors occur. Configure quiet hours and escalation delays to avoid alerting for transient issues.

---

### Q5. How do you avoid alert fatigue?

**Interview Answer**

Tune alert thresholds to minimize false positives — use longer evaluation windows and burn-rate-based alerts. Consolidate related alerts into single incidents. Remove or silence alerts that regularly fire without action. Use alert grouping to combine related alerts. Review alert history monthly and retire unused alerts. The "would I wake up for this?" test helps determine if an alert should be critical. Aim for fewer than 5 critical alerts per week per team.

---

### Q6. What are multi-window burn-rate alerts?

**Interview Answer**

Burn-rate alerts compare error rates over two time windows to reduce false positives. A fast burn-rate (e.g., 14.4x over 5 minutes) catches sudden spikes, while a slow burn-rate (e.g., 1x over 6 hours) catches gradual degradation. An alert fires only when both windows indicate an SLO violation. This approach is more accurate than single-window alerts because it considers both immediate impact and sustained issues. Google SRE provides standard burn-rate configurations.

---

### Q7. How do you implement alerting for Rust services using Prometheus?

**Interview Answer**

Define alert rules in Prometheus configuration using PromQL expressions. Route alerts through Alertmanager, which handles deduplication, grouping, and routing to receivers (PagerDuty, Slack, email). Use `prometheus::alerting` rules or the `prometheus-alertmanager` crate to manage rules programmatically. Test alert rules with `promtool test rules`. Monitor alert metrics (`prometheus_notifications_total`) to ensure the pipeline is functioning.

---

### Q8. How do you handle cascading alerts across multiple services?

**Interview Answer**

Use alert routing to suppress downstream alerts when an upstream service is known to be failing. Implement alert inhibition rules in Alertmanager — if service A is down, suppress alerts for services B and C that depend on it. Use dependency maps to understand blast radius. Group alerts by incident rather than by service. The goal is to identify the root cause alert and suppress symptoms, reducing noise during major incidents.

---

### Q9. How do you test your alerting setup?

**Interview Answer**

Use `promtool test rules` to validate alert rule logic with sample data. Inject faults in staging to verify alerts fire correctly (chaos engineering). Run game days to practice alert response procedures. Review past incidents to check if alerts would have fired. Test escalation policies by sending test notifications. Verify that runbooks are accurate by having someone follow them during a drill. Track alert accuracy metrics: true positive rate, false positive rate, time to detect.

---

### Q10. How do you integrate alerts with on-call rotation schedules?

**Interview Answer**

Configure on-call schedules in PagerDuty or Opsgenie with primary and secondary responders. Set escalation delays — if the primary doesn't acknowledge within 5 minutes, escalate to secondary. Use follow-up rotations for different severity levels. Document the on-call role expectations, response times, and authority levels. Schedule regular on-call handoffs with status updates. Consider timezone distribution for global teams. Track on-call burden metrics to ensure equitable rotation.
