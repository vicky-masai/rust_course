# Monitoring Deployments

## Interview Question

How do you monitor deployments, implement rollback triggers, set up metrics and alerts, and ensure safe releases in production?

## Interview Answer

Monitoring deployments requires tracking application health, performance metrics, and deployment events to detect issues early and trigger automatic rollbacks when necessary. Implement deployment monitoring by: instrumenting your Rust service with Prometheus metrics (request rate, error rate, latency histograms, resource usage), defining SLOs (Service Level Objectives) that quantify acceptable performance, and configuring alerts that trigger when metrics deviate from SLOs. For deployment safety, use tools like Argo Rollouts or Flagger that automatically analyze metrics during canary deployments and rollback if error rates exceed thresholds. Key metrics to monitor: error rate (4xx/5xx responses), p99 latency, CPU/memory utilization, Pod restart count, and deployment rollout status. In Grafana, create dashboards that show pre/post deployment metrics side-by-side for comparison. For Rust services, instrument with the `prometheus` crate, expose a `/metrics` endpoint, and configure alerting rules in Alertmanager that notify on-call engineers when deployments degrade performance. The goal is automated rollback for common failures and fast manual intervention for complex issues.

---

## Follow-up Questions & Answers

### Q1. What key metrics should you monitor during and after deployments?

**Interview Answer**

Monitor these metrics during and after deployments: request rate (sudden drops indicate traffic routing issues), error rate (spike in 4xx/5xx indicates breaking changes), p50/p95/p99 latency (degradation indicates performance regressions), CPU and memory utilization (spikes indicate resource issues), Pod restart count (restarts indicate crashes or OOM kills), deployment rollout status (Kubernetes events showing progress), and saturation metrics (queue depth, connection pool usage). For Rust services, also monitor tokio runtime metrics (task count, poll duration), garbage collection pauses (if using jemalloc), and connection pool metrics. Compare pre-deployment and post-deployment baselines — if error rate increases by more than 1% or p99 latency increases by more than 20%, trigger investigation or automatic rollback. Use Grafana dashboards with time markers for deployments to visualize the correlation between deployments and metric changes.

---

### Q2. How do you implement automatic rollback triggers?

**Interview Answer**

Automatic rollback triggers monitor metrics during deployment and revert if thresholds are exceeded. Argo Rollouts provides this natively: define `analysis` steps that query Prometheus metrics during canary deployment, and if metrics fail thresholds, automatic rollback occurs. For custom implementations, use a Kubernetes CronJob or deployment controller that: queries Prometheus for error rate and latency after deployment, compares against thresholds, and runs `kubectl rollout undo` or `helm rollback` if exceeded. For Rust services, define SLOs like "error rate < 1%" and "p99 latency < 500ms" — if either is violated for 5 minutes after deployment, trigger rollback. Use Alertmanager with a dedicated rollback webhook that calls your CI/CD API. Always log rollback events for post-incident review. Test rollback triggers in staging by intentionally deploying a broken version and verifying the automatic rollback works correctly.

---

### Q3. How do you set up Prometheus metrics for a Rust service?

**Interview Answer**

Use the `prometheus` crate to define and expose metrics in your Rust Axum/Actix service. Define counters for request counts (`register_counter!("http_requests_total", &["method", "path", "status"])`), histograms for latency (`register_histogram!("http_request_duration_seconds", &["method", "path"])`), and gauges for active connections. Create a `/metrics` handler that serializes all metrics in Prometheus format. For Axum: `async fn metrics() -> String { prometheus::TextEncoder::new().encode(&prometheus::gather()).unwrap() }`. Configure Prometheus to scrape your service's `/metrics` endpoint. For Rust services, also instrument database query duration, cache hit rate, and queue depth. Use labels to distinguish between endpoints, status codes, and versions. Store metric definitions in a central module and ensure consistent label naming across your codebase.

---

### Q4. How do you create effective deployment dashboards in Grafana?

**Interview Answer**

A deployment dashboard should show: deployment timeline (annotations marking when deployments occurred), key SLOs (error rate, latency, throughput) with threshold lines, infrastructure metrics (Pod count, CPU, memory), and deployment status (rollout progress, revision history). Use Grafana's annotation feature to mark deployment events — either manually or automatically via webhook from your CI/CD pipeline. Create panels for: request rate by endpoint, error rate by status code, latency percentiles (p50, p95, p99), Pod restart count, and resource utilization vs. requests. For Rust services, add panels for tokio runtime metrics and connection pool stats. Use variables for namespace, service name, and time range. Set up Grafana alerts that notify when metrics deviate from baselines post-deployment. The dashboard should give operators a single view to assess deployment health at a glance.

---

### Q5. How do you implement deployment notifications?

**Interview Answer**

Send deployment notifications to Slack, Teams, or PagerDuty at key pipeline stages: deployment started, deployment succeeded, deployment failed, and rollback triggered. In GitHub Actions, use `slackapi/slack-github-action` to post messages on workflow completion. Include: service name, version deployed, environment, deployer, and links to dashboards and logs. For Rust services, structure notifications as: "Deployed user-api v1.2.3 (abc1234) to production by @developer — Dashboard: <link> — Rollback: <link>". For failed deployments, include error details and logs. Use ArgoCD notifications or Flux notification-controller to send deployment events from GitOps tools. Configure PagerDuty integration for production deployment failures that require immediate attention. Automated notifications ensure the team is aware of deployment activity without checking CI/CD dashboards manually.

---

### Q6. How do you handle deployment verification in production?

**Interview Answer**

Deployment verification confirms that a deployment is healthy after it reaches production. Implement as a post-deployment pipeline stage that: waits for all Pods to be ready, verifies health endpoints return 200, runs smoke tests against the deployed service, checks key metrics against baselines, and monitors for 5-10 minutes for degradation. For Rust services, the verification script might: curl the health endpoint, send test requests to critical endpoints, verify response times are within SLOs, and check that no error spikes occur. If verification fails, automatically rollback. Tools like Flagger handle this automatically during canary analysis. For manual verification, use Grafana dashboards and Kubernetes events. Document verification steps in runbooks so on-call engineers know what to check. The goal is to catch issues within minutes of deployment, not hours when users report problems.

---

### Q7. How do you implement feature flag monitoring?

**Interview Answer**

Feature flags require monitoring because they decouple deployment from release — code is deployed but not active until the flag is enabled. Monitor flag state changes (who enabled/disabled which flag, when), flag evaluation rates (how many requests hit the new code path), and metric differences between flag-on and flag-off groups. For Rust services with feature flags, instrument metrics with flag state labels: `http_requests_total{flag_new_checkout="true"}`. Compare error rates and latency between flag groups to detect issues with the new code path. Use feature flag platforms (LaunchDarkly, Unleash) that provide built-in monitoring and gradual rollout capabilities. Set up alerts for flag-related metric deviations — if the new code path has a significantly higher error rate, alert and consider disabling the flag. This provides observability into feature releases, not just deployments.

---

### Q8. How do you track deployment frequency and lead time?

**Interview Answer**

Track deployment frequency (deploys per day/week) and lead time (time from commit to production) using your CI/CD platform's analytics or custom metrics. GitHub Actions provides workflow run history with timestamps. Calculate lead time as: `production_deploy_time - git_commit_time`. For DORA metrics, track: deployment frequency (number of deployments per day), lead time for changes (time from commit to production), change failure rate (percentage of deployments causing failures), and mean time to recovery (time to restore service after failure). For Rust services, these metrics indicate CI/CD maturity — aim for deployment frequency of multiple times per day, lead time under 1 hour, change failure rate under 5%, and MTTR under 1 hour. Export deployment events to Prometheus with labels for service, version, and environment, and create Grafana dashboards tracking these metrics over time.

---

### Q9. How do you implement rollback procedures and test them?

**Interview Answer**

Rollback procedures should be automated, tested, and documented. For Kubernetes with Helm, rollback is `helm rollback <release> <revision>`. For GitOps with ArgoCD, rollback is `git revert <commit>` or ArgoCD UI rollback. Implement rollback as a CI/CD pipeline step that can be triggered manually or automatically. Test rollback procedures regularly: deploy a known-good version, deploy a broken version, verify the automatic rollback triggers, and verify the service recovers. For Rust services, ensure rollbacks are safe by: using backward-compatible database migrations, maintaining previous Docker image tags in the registry, and verifying that rollback doesn't break clients expecting the new API. Document rollback procedures in runbooks, practice them in game days, and track rollback frequency as a deployment quality metric. An untested rollback is not a rollback — always verify your rollback works before you need it.

---

### Q10. How do you implement progressive delivery monitoring?

**Interview Answer**

Progressive delivery monitoring tracks metrics across canary analysis stages, comparing canary against baseline versions to make promote/rollback decisions. Implement with Flagger or Argo Rollouts that query Prometheus during canary deployment, comparing error rate, latency, and throughput between versions. Define analysis intervals (every 1 minute) and success criteria (error rate < 1%, p99 < 500ms for 5 consecutive checks). For Rust services, instrument request metrics with version labels (`http_requests_total{version="canary"}`) so canary analysis can compare versions. Grafana dashboards should show canary vs. baseline metrics side-by-side during deployment. Set up automated promotion (increase traffic percentage when metrics pass) and automated rollback (revert when metrics fail). Log all analysis decisions for post-deployment review. Progressive delivery monitoring provides confidence that production changes are validated under real traffic before full exposure.