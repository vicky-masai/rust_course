# What is CI/CD

## Interview Question

Explain what CI/CD is, the benefits it provides, and how a pipeline concept works for modern software delivery.

## Interview Answer

CI/CD stands for Continuous Integration and Continuous Delivery/Deployment — CI is the practice of frequently merging code changes into a shared repository where automated builds and tests validate each change, while CD extends this by automatically deploying validated changes to staging or production environments. CI catches integration bugs early (within minutes of a commit) rather than discovering them days later during manual testing, and CD ensures that software is always in a deployable state. The pipeline concept chains stages (build → test → security scan → deploy) where each stage validates the output of the previous one, and a failure at any stage blocks progression to the next. For Rust backend developers, CI/CD means every `cargo test` and `cargo clippy` run automatically on every PR, and successful merges automatically deploy to production through a tested pipeline. The benefits include faster feedback loops, reduced manual errors, consistent build environments, and the ability to deploy small changes frequently with confidence.

---

## Follow-up Questions & Answers

### Q1. What is the difference between Continuous Delivery and Continuous Deployment?

**Interview Answer**

Continuous Delivery ensures that every change is automatically validated and prepared for production deployment, but requires manual approval before actually deploying. Continuous Deployment goes further by automatically deploying every validated change to production without human intervention. In practice, most teams adopt Continuous Delivery first — establishing automated testing and staging deployment — and gradually move toward Continuous Deployment as confidence in the pipeline grows. For Rust services, Continuous Delivery means your PR merges automatically deploy to staging, but production deployment requires a manual trigger or approval. Continuous Deployment means merging to main automatically rolls out to production. Choose based on your risk tolerance, regulatory requirements, and team maturity.

---

### Q2. What are the key stages in a CI/CD pipeline?

**Interview Answer**

A typical CI/CD pipeline includes: Source (triggered by git push/PR), Build (compile code, build Docker images), Unit Tests (run test suite), Integration Tests (test against real dependencies), Lint/Static Analysis (catch code quality issues), Security Scanning (vulnerability and secret detection), Artifact Publishing (push images to registry), Staging Deployment (deploy to test environment), Acceptance Tests (validate in staging), and Production Deployment (blue-green or canary release). For Rust projects, the Build stage runs `cargo build --release`, tests run `cargo test`, linting uses `cargo clippy`, and security scanning uses `cargo audit`. The pipeline is a directed acyclic graph (DAG) where stages can run in parallel (unit tests and linting) or sequentially (build before test). Failures at any stage stop the pipeline and notify the team.

---

### Q3. How does CI/CD improve developer productivity?

**Interview Answer**

CI/CD eliminates manual, repetitive tasks (building, testing, deploying) that consume developer time and are error-prone when done manually. Fast feedback loops (tests completing in minutes, not hours) let developers fix issues while the context is still fresh, reducing context-switching. Automated deployments remove the "deployment day" ceremony and enable deploying anytime, including Fridays. For Rust developers, CI/CD provides immediate feedback on compilation errors, test failures, clippy warnings, and dependency vulnerabilities — problems caught by CI are dramatically cheaper to fix than those caught in production. The pipeline becomes a quality gate that enforces standards consistently, freeing developers to focus on feature development rather than manual quality assurance.

---

### Q4. What tools are commonly used for CI/CD pipelines?

**Interview Answer**

Popular CI/CD tools include GitHub Actions (tight GitHub integration, YAML-based), GitLab CI/CD (built into GitLab, powerful for monorepos), Jenkins (highly customizable, plugin ecosystem), CircleCI (cloud-native, fast execution), and Azure DevOps (enterprise-focused, integrates with Azure). For Rust projects, GitHub Actions is particularly popular because of its free tier for open source, native Rust support, and caching of `cargo` dependencies. DeployArgoCD and Flux are used for GitOps-based continuous delivery to Kubernetes. For infrastructure-as-code integration, Terraform and Pulumi are often part of the deployment stage. The choice depends on your hosting platform, team size, and whether you prefer SaaS or self-hosted solutions.

---

### Q5. How do you handle secrets in CI/CD pipelines?

**Interview Answer**

Never hardcode secrets in pipeline definitions or source code — use your CI platform's secret management (GitHub Secrets, GitLab CI Variables, Jenkins Credentials). For GitHub Actions, store secrets in Settings → Secrets and reference them as `${{ secrets.DATABASE_URL }}` in workflow files. Mask secrets in logs by ensuring your CI platform automatically redacts secret values. For Rust projects, secrets might include database URLs for integration tests, API keys for external services, or credentials for container registry access. Use OIDC tokens for cloud provider authentication instead of long-lived credentials (GitHub Actions supports AWS, GCP, and Azure OIDC). Rotate secrets regularly and audit who has access. For production deployments, use short-lived tokens from your CI platform's OIDC provider rather than storing static credentials.

---

### Q6. What is a build artifact and how is it managed in CI/CD?

**Interview Answer**

A build artifact is the output of a build stage — for Rust, this is typically the compiled binary, Docker image, or compiled test binaries. Artifacts are stored in registries (Docker Hub, ECR for container images), artifact repositories (JFrog Artifactory, GitHub Packages), or build caches (S3 for compiled binaries). In GitHub Actions, use the `actions/upload-artifact` action to persist files between jobs. For Rust services, the artifact is usually a Docker image tagged with the commit SHA or Git tag, pushed to a container registry during the build stage and referenced by deployment stages. Artifact management ensures reproducibility — the exact binary deployed to production is the one tested in CI, eliminating "works on my machine" issues. Always sign artifacts and scan them for vulnerabilities before promoting to production.

---

### Q7. How do you implement CI/CD for a monorepo with multiple Rust services?

**Interview Answer**

Monorepo CI/CD requires detecting which services changed and only running pipelines for those services. Use path-based triggers in GitHub Actions (`paths: ['services/user-api/**']`) or tools like `nx affected` and `bazel` to determine changed packages. For Rust monorepos, use Cargo workspaces and run `cargo test --workspace` for full validation or target specific crates with `cargo test -p user-api`. Parallelize pipelines across changed services for faster feedback. For deployment, use separate pipeline definitions per service with shared CI configuration. Tools like Turborepo or Pants can cache build outputs across services, significantly speeding up monorepo builds. The challenge is balancing thoroughness (testing all affected services) with speed (only testing what changed).

---

### Q8. What are pipeline triggers and how do you configure them?

**Interview Answer**

Pipeline triggers define when a pipeline runs: on push to specific branches (main, develop), on pull request creation/updates, on tag creation (v1.0.0), on schedule (nightly builds), or manually via API/webhook. In GitHub Actions, triggers are defined in the `on` section of the workflow file — `push: { branches: [main] }` runs on merges to main, `pull_request: { branches: [main] }` runs on PRs targeting main. For Rust services, common patterns include: run full CI on every PR (build, test, lint, security scan), deploy to staging on merge to main, deploy to production on tag creation (v1.0.0), and run nightly integration tests and dependency audits. Use path filters to avoid running pipelines for documentation-only changes. Webhooks from external services (database migration tools, monitoring systems) can also trigger pipelines for specific workflows.

---

### Q9. How do you measure the effectiveness of a CI/CD pipeline?

**Interview Answer**

Key metrics include: Lead Time (time from code commit to production deployment — aim for hours, not weeks), Deployment Frequency (how often you deploy — daily or per-commit is ideal), Mean Time to Recovery (time to restore service after a failure — aim for under 1 hour), Change Failure Rate (percentage of deployments causing failures — aim for under 5%). Track pipeline execution time (aim for under 10 minutes for CI, under 30 minutes for full CD), flaky test rate (tests that sometimes fail without code changes — aim for 0%), and pipeline success rate (percentage of pipeline runs that succeed on first attempt). For Rust services, monitor build times (cargo build can be slow without caching), test execution time, and the frequency of CI-related blockers. Use tools like DORA metrics, GitHub Actions analytics, or custom dashboards to track these metrics and identify bottlenecks.

---

### Q10. How do you handle database migrations in CI/CD pipelines?

**Interview Answer**

Database migrations in CI/CD should be version-controlled (using Diesel, SQLx, or Refinery for Rust), tested against a real database in CI, and applied automatically during deployment. The pipeline should: run `cargo sqlx migrate run` against a test database during CI to verify migrations are valid, then apply migrations as a pre-deployment step (Helm hook, init container, or separate Job) before deploying the new application version. Ensure migrations are backward-compatible so the previous application version can still function during rolling deployments. For production, use blue-green deployments where migrations are applied to the green environment before traffic switches, or use expand-and-contract patterns where you add new columns before removing old ones. Never skip migration testing in CI — failed migrations in production are among the most difficult issues to recover from.