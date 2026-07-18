# CI/CD Pipeline Design

## Interview Question

How do you design a CI/CD pipeline with stages for build, test, lint, security scanning, and deployment?

## Interview Answer

A well-designed CI/CD pipeline consists of sequential and parallel stages that validate code quality, security, and functionality before deploying. The typical flow is: Source (triggered by git event) → Build (compile code, build Docker image) → Unit Tests (fast, isolated tests) → Lint (cargo clippy, rustfmt) → Integration Tests (test against real dependencies) → Security Scan (cargo audit, SAST, container scanning) → Artifact Publishing (push to registry) → Staging Deployment (deploy to test environment) → Acceptance Tests (end-to-end validation) → Production Deployment (blue-green or canary release). Stages like unit tests and linting can run in parallel for faster feedback, while integration tests and deployment must be sequential. For Rust services, cache `~/.cargo/registry` and `target/` directories to dramatically reduce build times, and use `cargo clippy -- -D warnings` to treat warnings as errors. Each stage should have clear success criteria and fail fast to provide rapid feedback to developers.

---

## Follow-up Questions & Answers

### Q1. How do you design pipeline stages for fast feedback?

**Interview Answer**

Put the fastest, most likely-to-fail checks first: syntax validation and formatting checks (< 1 minute), then unit tests (< 5 minutes), then linting (< 5 minutes), then integration tests (> 5 minutes). Use parallelism within stages — run unit tests, linting, and formatting checks simultaneously. For Rust projects, the build stage is often the slowest (cargo build can take 10+ minutes on cold cache), so aggressive caching is essential. Use `actions/cache` to cache `~/.cargo` and `target/` directories, or use tools like `sccache` for distributed compilation caching. Structure the pipeline so developers see feedback within 5 minutes of pushing code — if your full pipeline takes 30 minutes, developers context-switch to other tasks and lose focus. Fast feedback loops are the primary value of CI.

---

### Q2. How do you handle flaky tests in CI/CD pipelines?

**Interview Answer**

Flaky tests (tests that sometimes pass, sometimes fail without code changes) destroy trust in CI/CD pipelines and should be prioritized for fixing. Identify flaky tests by tracking pass/fail rates across runs — GitHub Actions can show flaky test annotations. Common causes for Rust tests: async timing issues (use `tokio::time::pause()` for deterministic time), shared state between tests (use `tempfile` for test isolation), network-dependent tests (mock external services), and race conditions in concurrent code (use proper synchronization). Quarantine known flaky tests by marking them as allowed failures while investigating, but don't ignore them permanently. For Rust integration tests, use testcontainers-rs to spin up isolated database instances per test, eliminating shared-state flakiness. Aim for a flaky test rate below 1% — above that, developers start ignoring test failures.

---

### Q3. What is the role of linting and static analysis in CI/CD?

**Interview Answer**

Linting and static analysis catch code quality issues, potential bugs, and style violations before they reach production. For Rust, this includes `cargo fmt --check` (enforces consistent formatting), `cargo clippy` (catches common mistakes, performance issues, and idiomatic improvements), and `cargo audit` (scans dependencies for known vulnerabilities). In CI, run these as fast parallel stages that block merging if they fail — treat clippy warnings as errors with `cargo clippy -- -D warnings`. For additional static analysis, use `cargo-deny` to enforce license policies and dependency constraints, and `cargo-tarpaulin` for code coverage reporting. The pipeline should reject PRs that don't meet formatting standards, have clippy warnings, or have declining test coverage. This enforces code quality consistently without manual review overhead.

---

### Q4. How do you implement security scanning in CI/CD?

**Interview Answer**

Security scanning should be integrated at multiple pipeline stages: dependency scanning (cargo audit, Snyk, Dependabot) catches known vulnerabilities in dependencies, SAST (Static Application Security Analysis) scans your Rust source code for security patterns, container scanning (Trivy, Snyk Container) checks Docker images for OS-level vulnerabilities, and secret detection (git-secrets, truffleHog) prevents committing API keys or credentials. In GitHub Actions, add `cargo audit` as a pipeline step, integrate Trivy for container scanning after building the Docker image, and run secret detection on every push. For Rust services, also check for unsafe code usage with `cargo-geiger` and enforce that all dependencies have acceptable licenses with `cargo-deny`. Security scanning should block PRs with critical vulnerabilities while allowing low-severity issues to be tracked and resolved separately.

---

### Q5. How do you design pipeline stages for a monorepo with multiple services?

**Interview Answer**

For monorepos, use path-based triggers to only run pipelines for changed services, and structure pipelines to share common stages. In GitHub Actions, use `paths` filters: `on: push: paths: ['services/user-api/**']`. Create reusable workflow files for common stages (build, test, lint) and reference them from service-specific workflows. For Rust monorepos with Cargo workspaces, run `cargo test -p <changed-crate>` to test only affected packages, or use tools like `cargo-workspaces` to detect changes. Parallelize pipelines across changed services — if user-api and order-api both changed, run their pipelines simultaneously. Share common infrastructure (Docker build caching, test database provisioning) across services to reduce total pipeline time. The goal is that CI for a single-service change completes in under 10 minutes, not the full monorepo test suite.

---

### Q6. What is artifact versioning and how do you implement it?

**Interview Answer**

Artifact versioning ensures that every build produces a unique, traceable artifact that can be deployed and rolled back reliably. Common strategies include: Git SHA (`abc1234`), semantic versioning (`1.2.3`), timestamp (`20240101-120000`), or auto-incrementing build numbers. For Docker images, use the Git SHA as the primary tag (`my-api:abc1234`) and semantic version for releases (`my-api:1.2.3`). In CI/CD pipelines, derive the version from Git: `VERSION=$(git describe --tags --always)` and pass it to `cargo build` and `docker build`. Tag the Git commit when creating a release version, and use that tag in the Docker image tag. For Rust services, embed the version at compile time using `env!("CARGO_PKG_VERSION")` or `built` crate for detailed build metadata. Always push both the SHA tag (for every build) and version tag (for releases) to the container registry.

---

### Q7. How do you implement rollback strategies in CI/CD?

**Interview Answer**

Rollback strategies ensure you can quickly revert to a previous working version when a deployment fails. For Kubernetes, Helm maintains release history — `helm rollback my-release 1` reverts to revision 1. For containerized Rust services, the simplest rollback is re-deploying the previous Docker image tag (Git SHA). Implement rollback triggers: automatic rollback if health checks fail after deployment, or manual rollback via CI/CD pipeline with a "rollback" workflow. For blue-green deployments, switching traffic back to the blue environment is instant rollback. For canary deployments, automatically promote to full rollout only after metrics meet thresholds, otherwise rollback. Always test your rollback procedure — an untested rollback is not a rollback. Store previous artifact tags and ensure they remain in the container registry (don't purge old tags aggressively).

---

### Q8. How do you handle caching in CI/CD for Rust projects?

**Interview Answer**

Rust builds are notoriously slow without caching, so aggressive caching is essential for CI/CD performance. Cache three things: `~/.cargo/registry` (downloaded crate sources), `~/.cargo/git` (git dependencies), and `target/` (compiled artifacts). In GitHub Actions, use `actions/cache` with a key based on `Cargo.lock` — when dependencies don't change, the cache is restored and compilation is significantly faster. For even better caching, use `sccache` (shared compilation cache) or `cargo-chef` for Docker-based builds (creates a dependency layer that's cached across builds). The `Swatinem/rust-cache` action automates Rust-specific caching. For monorepos, cache per-workspace to avoid invalidation from unrelated changes. A well-cached Rust CI pipeline should complete build + test in under 5 minutes (compared to 15+ minutes without caching).

---

### Q9. How do you implement environment-specific pipeline configurations?

**Interview Answer**

Use separate pipeline stages or workflows for each environment (dev, staging, prod) with environment-specific configurations. In GitHub Actions, use environments with protection rules: `environments: [staging]` for automatic deployment, `environments: [production]` with required reviewers for manual approval. Store environment-specific values in separate files (values-staging.yaml, values-prod.yaml) and reference them during deployment. For Rust services, this means: the CI pipeline runs identically for all environments, but the CD pipeline deploys to staging automatically on merge to main, and to production only on tag creation with manual approval. Use Helm values files or environment variables to inject environment-specific configuration (database URLs, API endpoints, resource limits). Ensure the pipeline enforces that production deployments can only come from staging-tested artifacts.

---

### Q10. How do you design pipelines for high-availability and disaster recovery?

**Interview Answer**

For HA pipelines, ensure your CI/CD platform itself is resilient — GitHub Actions runs on GitHub's infrastructure, but self-hosted runners need redundancy. Use multiple runners across availability zones, and configure pipeline retries for transient failures (network issues, temporary resource constraints). For Rust services, design deployments for zero downtime: rolling updates with health checks, database migrations that are backward-compatible, and feature flags that allow deploying code before enabling features. Implement disaster recovery by: storing pipeline definitions in Git (infrastructure-as-code), maintaining the ability to rebuild all artifacts from source, testing backup restoration procedures regularly, and ensuring your CI/CD pipeline can deploy to a secondary cluster if the primary fails. For production Rust services, practice "game days" where you simulate failures and verify that your pipeline and deployment processes handle them correctly.