# Load Testing in Rust

## Interview Question

How do you perform load testing on a Rust backend service, and what tools and patterns do you use?

## Interview Answer

Load testing validates that your service handles expected (and unexpected) traffic volumes. Common tools include k6 for scriptable load tests, Locust for Python-based distributed testing, and `criterion` for Rust micro-benchmarks. For Rust-specific benchmarks, `criterion` measures throughput and latency of critical code paths. Production-like load tests use k6 scripts that simulate user behavior patterns. Always test from a separate machine to avoid resource contention, and monitor server metrics (CPU, memory, p99 latency) during the test.

---

## Follow-up Questions & Answers

### Q1. What is the difference between load testing, stress testing, and spike testing?

**Interview Answer**

Load testing simulates expected traffic to verify performance under normal conditions. Stress testing pushes beyond expected limits to find the breaking point. Spike testing simulates sudden traffic surges (like a flash sale) to test auto-scaling and recovery. Load testing validates SLOs, stress testing finds capacity limits, and spike testing validates resilience to transient overload.

---

### Q2. How do you use k6 to load test a Rust API?

**Interview Answer**

Install k6, write a JavaScript script defining user scenarios (login, browse, purchase), and run it with `k6 run script.js`. Configure stages for ramp-up, sustained load, and ramp-down. Use `http.get()` and `http.post()` to make requests. k6 reports metrics like response time percentiles, throughput, and error rates. Run k6 from a separate machine or cloud service to avoid local resource constraints affecting results.

---

### Q3. How do you use `criterion` for Rust benchmarks?

**Interview Answer**

Add `criterion` as a dev-dependency and create benchmarks in `benches/`. Define a benchmark group, add functions with `c.bench_function()`, and use `criterion_group!` and `criterion_main!` macros. Run with `cargo bench`. Criterion provides statistical analysis, compares results against previous runs, and generates HTML reports. Use it for micro-benchmarks of hot paths like serialization, database queries, and crypto operations.

---

### Q4. What metrics should you collect during load tests?

**Interview Answer**

Track response time percentiles (p50, p95, p99), throughput (requests per second), error rate, CPU utilization, memory usage, connection pool saturation, and queue depths. Monitor for latency spikes that indicate GC pauses or lock contention. Compare results against your SLOs. Log all metrics with timestamps for post-test analysis. Use tools like Prometheus and Grafana to visualize server-side metrics during the test.

---

### Q5. How do you design realistic load test scenarios?

**Interview Answer**

Analyze production traffic patterns using APM data to understand peak hours, common user journeys, and request distribution. Model realistic think times between requests. Mix read-heavy and write-heavy operations in the same ratio as production. Include authentication flows, cache warming, and data-dependent scenarios. Use production-like data volumes to test database query performance realistically.

---

### Q6. How do you handle authentication in load tests?

**Interview Answer**

Pre-generate tokens or use a setup phase that authenticates test users. Share tokens across virtual users where appropriate. For JWT tokens, ensure they have long enough expiry for the test duration. Use bulk user creation scripts for tests that need many unique accounts. Don't hammer the auth endpoint — it should be warmed up separately and excluded from throughput metrics.

---

### Q7. What are common anti-patterns in load testing?

**Interview Answer**

Common mistakes include testing from the same machine as the server, using insufficient warm-up time, ignoring client-side bottlenecks, testing with unrealistic data, not monitoring server metrics, and drawing conclusions from a single run. Don't optimize based on load test results alone — always verify with production monitoring. Avoid testing only happy paths; include error scenarios and edge cases in your load tests.

---

### Q8. How do you use Locust for distributed load testing?

**Interview Answer**

Write Python classes inheriting from `HttpUser` with `@task` decorated methods defining user behavior. Run with `locust -f locustfile.py --host=http://localhost:3000`. For distributed testing, run `locust --master` and `locust --worker` on multiple machines. Locust provides a web UI for real-time metrics and supports ramping up users gradually. It's good for complex user scenarios but requires Python knowledge.

---

### Q9. How do you validate SLOs during load tests?

**Interview Answer**

Define pass/fail criteria before the test: p99 latency under 200ms, error rate below 0.1%, throughput above 1000 rps. Use k6 thresholds or criterion's comparison features to automatically fail tests that don't meet SLOs. Compare load test results against production APM data. If load test results are significantly better than production, your test scenario may not be realistic enough.

---

### Q10. How do you perform load testing in CI/CD pipelines?

**Interview Answer**

Run lightweight performance regression tests in CI using `criterion` to detect performance degradation. Reserve full load tests for pre-deployment staging environments. Use GitHub Actions or GitLab CI with dedicated runners for load testing. Store benchmark results over time to track performance trends. Fail deployments if performance regresses beyond a threshold. For critical services, run load tests against canary deployments before full rollout.
