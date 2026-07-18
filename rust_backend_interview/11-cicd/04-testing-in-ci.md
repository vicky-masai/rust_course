# Testing in CI

## Interview Question

How do you implement unit, integration, and end-to-end tests in CI/CD pipelines, and how do you handle test parallelization?

## Interview Answer

Testing in CI/CD requires a layered approach: unit tests (fast, isolated, run on every commit), integration tests (test component interactions, run against real dependencies), and end-to-end tests (validate full user flows in a deployed environment). Unit tests run first because they're fastest (seconds to minutes) and catch basic regressions. Integration tests use testcontainers-rs or docker-compose to spin up databases, message queues, and other dependencies, running in parallel to reduce execution time. E2E tests deploy the service to a test environment and validate HTTP endpoints, typically using tools like `reqwest` or `curl` in test scripts. For parallelization, use `cargo test -- --test-threads=N` to control parallel test execution, and split test suites across CI jobs using matrix strategies. In GitHub Actions, use `needs:` to create dependencies between stages (unit tests before integration tests) and run independent test jobs in parallel. The goal is fast feedback — developers should see unit test results within 2 minutes of pushing code.

---

## Follow-up Questions & Answers

### Q1. What is the difference between unit, integration, and E2E tests in Rust?

**Interview Answer**

Unit tests are written alongside your code in the same file using `#[cfg(test)]` modules, testing individual functions in isolation without external dependencies. Integration tests live in the `tests/` directory and test how multiple components work together, often requiring database connections or external services. E2E tests validate complete user flows by making HTTP requests to a deployed service and asserting on responses. For a Rust Axum service, a unit test verifies a handler function's logic, an integration test checks that the handler works with a real PostgreSQL database, and an E2E test sends a request to the deployed service and verifies the response. Run unit tests in CI on every commit (fastest), integration tests on merge to main (requires dependencies), and E2E tests after staging deployment (requires running service).

---

### Q2. How do you use testcontainers-rs in CI/CD pipelines?

**Interview Answer**

testcontainers-rs provides a Rust API for spinning up Docker containers (databases, message queues, emulators) for integration tests. Define containers in test code: `let postgres = Postgres::default().start().await;` creates a temporary PostgreSQL instance. In CI, ensure Docker is available (GitHub-hosted runners have Docker pre-installed) and that the pipeline has sufficient resources (containers use memory and CPU). For Rust services, use testcontainers to test database operations, cache interactions, and external API calls with mocked services. The container lifecycle is tied to the test — it starts before the test and stops after, ensuring clean isolation. For CI optimization, use container reuse across tests with `Reuse` policy, but be cautious of state leaking between tests. Testcontainers-rs eliminates the need for shared test databases, making integration tests reliable and parallelizable.

---

### Q3. How do you parallelize tests in CI/CD pipelines?

**Interview Answer**

Parallelize tests at two levels: within a single CI job using Rust's built-in parallelism (`cargo test -- --test-threads=8`) and across CI jobs using matrix strategies. For GitHub Actions, split tests by type (unit, integration, E2E) into separate jobs that run simultaneously. Use path filters to only run relevant test suites — if you changed the user module, only run user tests. For large test suites, use tools like `cargo-nextest` which provides better parallelism and test isolation than the default cargo test runner. In monorepos, use `cargo-workspaces` to test only changed crates in parallel. For E2E tests, run them against a deployed staging environment in parallel with other validation steps. Monitor test execution time and set budgets — if integration tests take over 10 minutes, investigate parallelization opportunities or test optimization.

---

### Q4. How do you handle flaky tests in CI/CD?

**Interview Answer**

Flaky tests undermine CI/CD trust and should be fixed immediately. Identify them by tracking pass/fail rates — GitHub Actions annotations show repeated failures on the same test. Common causes: async timing issues (use `tokio::time::pause()` in Rust tests), shared state between tests (use `tempfile::TempDir` for isolated file system state), network dependencies (mock external services with `mockall` or `wiremock`), and race conditions (use proper synchronization primitives). Quarantine known flaky tests by marking them as `#[ignore]` with a comment explaining the flakiness, or move them to a separate CI job that doesn't block merging. For Rust async tests, use `tokio::test` with `flavor = "current_thread"` to avoid multi-threading issues. Never merge code with known flaky tests without a plan to fix them — flaky tests compound over time and destroy developer confidence in CI.

---

### Q5. How do you implement contract testing in CI/CD?

**Interview Answer**

Contract testing verifies that service interfaces match between producers and consumers without requiring both services to be running. For Rust microservices, use tools like `pact` to define contracts — the consumer writes a test specifying expected requests/responses, and the provider verifies it fulfills the contract. In CI, run consumer contract tests when the consumer changes, and provider verification tests when the provider changes. This catches interface mismatches early without deploying services together. For REST APIs, validate OpenAPI specs against actual responses using `openapi-validator`. Contract tests run faster than E2E tests (no deployment needed) and provide stronger guarantees than unit tests (real interface validation). For Rust services, store contract files in a shared repository or use Pact Broker to manage contract versions between teams.

---

### Q6. How do you implement test coverage tracking in CI/CD?

**Interview Answer**

Use `cargo-tarpaulin` to generate code coverage reports for Rust projects — it instruments your binary and measures which lines are executed during tests. In CI, run `cargo tarpaulin --out xml` to generate coverage in a format compatible with Codecov, Coveralls, or SonarQube. Upload coverage reports with `codecov/codecov-action` and configure quality gates (e.g., fail CI if coverage drops below 80%). For Rust services, focus coverage on business logic rather than boilerplate — target coverage above 70% for meaningful modules. Track coverage trends over time, not just absolute numbers. Coverage is a useful metric but not the only one — a test with 100% line coverage might miss edge cases, while a focused integration test with lower coverage might catch more bugs. Use coverage to identify untested code, not as a strict quality gate.

---

### Q7. How do you test Docker images in CI/CD?

**Interview Answer**

After building a Docker image in CI, test it before publishing by running the image in a container and executing health checks, smoke tests, or integration tests. Use `docker run` with environment variables matching production configuration, then `curl` the health endpoint to verify the service starts correctly. For Rust services, test that the binary runs correctly in the container (musl vs. glibc compatibility), that environment variables are properly configured, and that the health check endpoint responds. Use `docker-compose` in CI for testing with dependencies (database, cache). For image security, scan with Trivy or Snyk Container before pushing. Publish only tested images with the commit SHA tag, and use semantic version tags for releases. For multi-platform builds (linux/amd64, linux/arm64), use `docker buildx` with `--platform` flag in CI.

---

### Q8. How do you implement load testing in CI/CD pipelines?

**Interview Answer**

Load testing in CI/CD validates performance characteristics before production deployment. Use tools like `wrk`, `k6`, or `drill` to simulate concurrent users against a deployed staging service. In the pipeline, deploy to staging, run load tests as a validation step, and block deployment if performance degrades beyond thresholds (e.g., p99 latency exceeds 200ms). For Rust services, load test after staging deployment to verify that connection pooling, async runtime, and resource limits handle expected traffic. Store load test results as artifacts for comparison across versions. Don't run load tests on every commit (too slow and resource-intensive) — run them nightly or before production releases. Use CI-triggered load tests for performance-critical services, and set up automated rollback if load test failures exceed thresholds.

---

### Q9. How do you implement snapshot testing in CI/CD?

**Interview Answer**

Snapshot testing captures expected output (API responses, HTML pages, protobuf messages) and compares against future runs to detect unintended changes. In Rust, use the `insta` crate for snapshot testing — `insta::assert_snapshot!(actual_value)` captures the first run and compares on subsequent runs. In CI, run snapshot tests and use `cargo insta review` to approve intentional changes (updates snapshots) and reject unintentional ones. Store snapshot files in Git so changes are reviewed in PRs. For Rust services, snapshot test API responses, serialized data structures, and error messages. The CI pipeline should fail if unreviewed snapshot changes exist, forcing developers to review and approve or reject changes. This catches unexpected output changes that traditional assertions might miss.

---

### Q10. How do you balance test thoroughness with CI/CD speed?

**Interview Answer**

Balance is achieved through test pyramid principles: many fast unit tests (seconds), fewer integration tests (minutes), and minimal E2E tests (minutes to hours). Use path filters to run only relevant tests — changed code triggers targeted test suites, not the full suite. Implement test tiers: run unit tests on every commit (fast feedback), integration tests on PR merge (thorough validation), and E2E tests after staging deployment (end-to-end verification). For Rust, `cargo test` for unit tests completes in seconds, while integration tests with testcontainers may take minutes. Use `cargo-nextest` for faster test execution with better parallelism. Monitor test execution time and set budgets — if total CI time exceeds 10 minutes, investigate optimization opportunities. Accept that some tests (E2E, load tests) run only before production releases, not on every commit. The goal is maximum confidence with minimum wait time.