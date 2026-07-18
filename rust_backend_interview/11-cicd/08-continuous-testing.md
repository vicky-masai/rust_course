# Continuous Testing

## Interview Question

Explain continuous testing, shift-left testing, contract testing, and chaos engineering in the context of CI/CD pipelines.

## Interview Answer

Continuous testing means running automated tests at every stage of the software delivery lifecycle — not just during a dedicated testing phase, but during development, code review, CI/CD, and production monitoring. Shift-left testing moves testing earlier in the development process, catching bugs when they're cheapest to fix (during coding, not after deployment). Contract testing validates that service interfaces match between producers and consumers without requiring both services to be running. Chaos engineering intentionally injects failures (network partitions, service crashes, resource exhaustion) into production systems to verify resilience. For Rust backend developers, continuous testing means running `cargo test` on every commit, integration tests with testcontainers on every PR, contract tests against dependent services before deployment, and chaos experiments (killing Pods, injecting latency) in staging or production to verify your Rust services handle failures gracefully. This multi-layered approach catches bugs at the earliest, cheapest stage while validating that production systems are resilient to real-world failures.

---

## Follow-up Questions & Answers

### Q1. What is shift-left testing and how do you implement it?

**Interview Answer**

Shift-left testing means moving testing activities earlier in the development lifecycle — writing tests before or alongside code, running static analysis during development, and validating changes before they reach the testing phase. Implement by: using pre-commit hooks (`cargo clippy`, `cargo fmt --check`) that run on every commit, providing IDE integrations that show test results as you code, running unit tests in the editor with `cargo watch -x test`, and using GitHub Actions to run tests on every push (not just PRs). For Rust, use `cargo test` during development, `cargo clippy` to catch common mistakes, and `cargo audit` to check dependency vulnerabilities. Shift-left catches bugs when the context is fresh — fixing a bug during coding takes minutes, while fixing it after deployment takes hours. The goal is zero bugs reaching production by catching them at the earliest possible stage.

---

### Q2. What is contract testing and how does it work for microservices?

**Interview Answer**

Contract testing verifies that service interfaces (APIs) match between producers and consumers without requiring both services to be running simultaneously. The consumer writes a contract specifying expected requests and responses (e.g., "GET /users/123 should return 200 with name field"), and the provider verifies it fulfills the contract. Tools like Pact implement this: consumers define interactions in tests, contracts are published to a broker, and providers verify against all consumer contracts. For Rust microservices, contract testing catches interface mismatches early — if your user service changes its API response format, the order service's contract test fails before deployment. In CI, run consumer contract tests when the consumer changes, and provider verification when the provider changes. This catches breaking changes between services without E2E tests that require all services running together.

---

### Q3. What is chaos engineering and when should you practice it?

**Interview Answer**

Chaos engineering is the practice of intentionally injecting failures into systems to verify they handle real-world failures gracefully. Experiments include: killing Pods (verify replication works), injecting network latency (verify timeout handling), filling disk (verify error handling), and terminating nodes (verify workload rescheduling). Start with a hypothesis ("our Rust service handles database connection loss gracefully"), run the experiment in a controlled environment (staging first), observe the results, and improve based on findings. Tools like Chaos Mesh (Kubernetes-native) and Litmus provide controlled failure injection. For Rust services, chaos engineering verifies that your async runtime handles timeouts, your connection pool recovers from database failures, and your health probes correctly detect degraded states. Run chaos experiments regularly in staging, and gradually in production with strict safety controls (automated rollback if SLOs are violated).

---

### Q4. How do you implement continuous testing in a CI/CD pipeline?

**Interview Answer**

Implement continuous testing as a multi-stage pipeline: unit tests (every commit, < 2 minutes), integration tests (every PR, < 10 minutes), contract tests (when interfaces change, < 5 minutes), and E2E tests (after deployment to staging, < 30 minutes). Each stage validates different aspects: unit tests check individual functions, integration tests check component interactions, contract tests check service interfaces, and E2E tests check user workflows. In GitHub Actions, use separate jobs for each test type, running faster tests first for quick feedback. For Rust services, use `cargo test` for unit/integration tests, `cargo clippy` for static analysis, and `cargo audit` for security. Store test results as artifacts for trend analysis. Fail the pipeline at any stage to block merging. The goal is that developers get comprehensive feedback within 15 minutes of pushing code.

---

### Q5. How do you implement chaos engineering in Kubernetes?

**Interview Answer**

Kubernetes chaos engineering uses tools like Chaos Mesh, Litmus, or tc (traffic control) to inject failures at the infrastructure level. Common experiments: `kubectl delete pod <pod>` (verify ReplicaSet recreates it), inject network latency with `tc qdisc add dev eth0 root netem delay 100ms` (verify timeout handling), fill disk with a temporary container writing large files (verify disk pressure handling), and kill kubelet on a node (verify node auto-recovery). Chaos Mesh provides CRDs for declarative chaos: `NetworkChaos`, `PodChaos`, `IOChaos` resources that the controller applies. For Rust services, verify that your connection pool handles database Pod restarts, your circuit breaker trips on dependency failures, and your health probes detect degraded states. Always run chaos experiments in staging first, define abort conditions (SLO violations trigger rollback), and run with safety controls (time limits, blast radius restrictions).

---

### Q6. How do you measure the effectiveness of continuous testing?

**Interview Answer**

Key metrics: defect escape rate (bugs found in production vs. caught by tests — aim for < 5%), test execution time (aim for < 10 minutes total CI), code coverage (aim for > 70% on business logic), mean time to detect (how quickly tests catch regressions — aim for < 5 minutes), and flaky test rate (tests that fail without code changes — aim for < 1%). Track test trend data: are test suites growing proportionally with code? Are new features accompanied by tests? Do integration tests catch real bugs that unit tests miss? For Rust services, monitor `cargo test` execution time, test reliability (flaky tests), and bug detection (tests that caught real production issues). Use coverage reports to identify untested code and prioritize testing efforts. The ultimate metric is production incident rate — effective continuous testing should reduce production bugs over time.

---

### Q7. What is the testing pyramid and how does it apply to CI/CD?

**Interview Answer**

The testing pyramid recommends: many fast unit tests at the base (seconds, isolated, cheap), fewer integration tests in the middle (minutes, test component interactions), and minimal E2E tests at the top (minutes to hours, validate complete workflows). This structure maximizes feedback speed while providing comprehensive coverage. In CI/CD, run unit tests on every commit (fastest feedback), integration tests on every PR (thorough validation), and E2E tests after staging deployment (end-to-end verification). For Rust services, the pyramid translates to: many `#[cfg(test)]` unit tests (fastest), integration tests with testcontainers (moderate speed), and E2E tests against deployed staging services (slowest). Don't skip layers — unit tests alone miss integration bugs, E2E tests alone are too slow for fast feedback. The pyramid shape ensures fast feedback while comprehensive coverage.

---

### Q8. How do you handle test data management in CI/CD?

**Interview Answer**

Test data management ensures tests have realistic, isolated data without depending on shared state. Strategies: synthetic data generation (create test data programmatically in tests), database seeding (run seed scripts before integration tests), testcontainers (spin up fresh database per test suite), and fixtures (define expected data in test files). For Rust services, use `fake` or `rand` crates to generate test data, `sqlx::query` to seed test databases, and testcontainers to provide isolated PostgreSQL instances. Never use production data in tests (privacy, compliance) — generate realistic synthetic data instead. For E2E tests, use dedicated test accounts with controlled data. Store test fixtures in version control and update them when schemas change. For contract tests, maintain consumer contract files that reflect expected API behavior.

---

### Q9. How do you implement performance testing in CI/CD?

**Interview Answer**

Performance testing in CI/CD validates that changes don't degrade performance. Implement as a pipeline stage: deploy to a performance testing environment, run load tests with tools like `k6` or `wrk`, compare results against baseline, and fail the pipeline if performance degrades beyond thresholds. For Rust services, test p99 latency, throughput (requests/second), and resource utilization (CPU, memory). Store performance baselines and compare against them on every PR — if p99 latency increases by more than 10%, fail the pipeline. Run performance tests on a schedule (nightly) rather than every commit to avoid slowing CI. Use consistent hardware for performance tests (same instance type, same network) to ensure reproducibility. For critical services, implement automated rollback if production performance degrades after deployment, using real-time metrics from Prometheus or Datadog.

---

### Q10. How do you implement security testing in continuous testing?

**Interview Answer**

Integrate security testing at multiple pipeline stages: SAST (static analysis) with `cargo clippy` and `cargo-audit` on every commit, dependency scanning with Snyk or Dependabot on every PR, container scanning with Trivy after building Docker images, and DAST (dynamic analysis) against deployed staging environments. For Rust, also use `cargo-geiger` to audit unsafe code usage and `cargo-deny` to enforce license policies. Implement secret detection with `git-secrets` or `truffleHog` to prevent committing credentials. For API security, run OWASP ZAP against your Rust service's endpoints to detect common vulnerabilities (injection, broken authentication). Security tests should block PRs with critical vulnerabilities while allowing low-severity issues to be tracked as technical debt. Regular security audits (quarterly) supplement automated testing for comprehensive coverage.