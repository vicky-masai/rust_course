# GitHub Actions

## Interview Question

Explain GitHub Actions workflows, jobs, and steps, including how to manage secrets, use caching, and optimize pipeline performance.

## Interview Answer

GitHub Actions is a CI/CD platform built into GitHub that automates build, test, and deployment workflows triggered by git events. A workflow is a YAML file in `.github/workflows/` containing one or more jobs that run in parallel by default (use `needs:` for dependencies). Each job runs on a runner (GitHub-hosted or self-hosted) and contains steps that execute commands, run actions, or use reusable workflows. Secrets are stored in repository settings and accessed as `${{ secrets.MY_SECRET }}`, and caching uses `actions/cache` or language-specific actions like `Swatinem/rust-cache` to persist dependencies between runs. For Rust projects, a typical workflow uses `dtolnay/rust-toolchain` to install Rust, `Swatinem/rust-cache` to cache Cargo artifacts, and `actions/upload-artifact` to persist build outputs. Workflow syntax supports matrix strategies for testing across multiple Rust versions and operating systems, conditional execution with `if:` statements, and reusable workflows for shared CI logic across repositories.

---

## Follow-up Questions & Answers

### Q1. What is the structure of a GitHub Actions workflow file?

**Interview Answer**

A workflow file (`.github/workflows/ci.yml`) starts with `name:`, then `on:` (triggers), then `jobs:` containing one or more jobs. Each job has `runs-on:` (runner type), optional `needs:` (job dependencies), `env:` (environment variables), and `steps:`. Each step can use `uses:` (reference an action), `run:` (execute a shell command), `with:` (action inputs), or `env:` (step-level variables). For Rust CI, a minimal workflow is: `on: [push, pull_request]`, `jobs: build: runs-on: ubuntu-latest, steps: [uses: dtolnay/rust-toolchain@stable, uses: Swatinem/rust-cache@v2, run: cargo test]`. Use `permissions:` to restrict the workflow's token scope, and `concurrency:` to cancel in-progress runs when new commits are pushed. YAML anchors and reusable workflows reduce duplication across multiple workflow files.

---

### Q2. How do you manage secrets in GitHub Actions?

**Interview Answer**

Store secrets in GitHub repository settings (Settings → Secrets and variables → Actions) and reference them in workflows as `${{ secrets.MY_SECRET }}`. Secrets are masked in logs — GitHub automatically redacts any occurrence of a secret value. For organization-wide secrets, use organization-level secrets shared across repositories. Use OIDC tokens for cloud provider authentication instead of static credentials — configure GitHub as an OIDC provider in AWS/GCP and use `aws-actions/configure-aws-credentials` with `role-to-assume` for short-lived tokens. For Rust projects, secrets might include: `DATABASE_URL` for integration tests, `DOCKERHUB_TOKEN` for pushing images, or `AWS_ACCESS_KEY_ID` for deployment. Never log secrets, never put them in workflow files, and rotate them regularly. Use environments with protection rules to restrict which secrets are available to which branches.

---

### Q3. How do you implement caching in GitHub Actions for Rust projects?

**Interview Answer**

Use `Swatinem/rust-cache@v2` which automatically caches `~/.cargo` and `target/` directories with smart key generation based on `Cargo.lock`. The action handles cache restoration, saving, and cleanup of old build artifacts. For manual caching, use `actions/cache` with `path: ~/.cargo` and a key based on the OS, Rust version, and `Cargo.lock` hash. In Docker-based builds, use multi-stage Dockerfiles with `cargo-chef` to create a dependency layer that's cached across builds. Enable `sccache` for distributed compilation caching in larger teams. For Rust monorepos, cache per workspace to avoid invalidation from unrelated changes. A well-configured cache reduces Rust CI build times from 15+ minutes to under 5 minutes. Always test cache effectiveness by comparing run times with and without cache hits.

---

### Q4. What are matrix strategies and when should you use them?

**Interview Answer**

Matrix strategies run a job multiple times with different parameter combinations, testing your Rust code across multiple Rust versions, operating systems, or dependency configurations. Define a matrix with `strategy: matrix: rust: [stable, beta, nightly]` and the job runs three times, once for each Rust version. Combine multiple dimensions: `matrix: { os: [ubuntu-latest, macos-latest], rust: [stable, 1.70] }` creates 4 combinations. Use `include:` and `exclude:` to add or remove specific combinations. For Rust services, test against stable (your production version), beta (upcoming changes), and nightly (future compatibility). Matrix builds run in parallel, so total time equals the slowest job, not the sum. Use `fail-fast: false` if you want all combinations to complete even if one fails (useful for identifying version-specific issues).

---

### Q5. How do you use reusable workflows and composite actions?

**Interview Answer**

Reusable workflows are defined in one repository and called from others using `uses: owner/repo/.github/workflows/workflow.yml@ref`, accepting inputs and secrets as parameters. Composite actions bundle multiple steps into a single action using `runs: composite` with `steps:` that can include both `uses:` and `run:` commands. For Rust projects, create a reusable workflow for common CI tasks (build, test, lint) and reference it from multiple service repositories, reducing duplication. Composite actions are useful for封装 common patterns like "setup Rust with caching" or "build and push Docker image." Share actions within an organization using a dedicated `actions` repository. Reusable workflows reduce maintenance overhead — a change to the shared workflow automatically applies to all repositories that use it.

---

### Q6. How do you handle concurrency in GitHub Actions?

**Interview Answer**

Use `concurrency` to control how many workflow runs execute simultaneously. `concurrency: { group: ${{ github.workflow }}-${{ github.ref }} }` ensures only one run per branch (new pushes cancel in-progress runs for the same branch). `cancel-in-progress: true` automatically cancels running workflows when a newer commit is pushed, saving runner minutes. For deployment workflows, use `concurrency: { group: deploy-production, cancel-in-progress: false }` to queue deployments rather than cancel them. For Rust projects, concurrency is critical because builds are expensive — canceling an in-progress build when a new commit is pushed saves 10+ minutes of compute time. Use `group` with descriptive names to control which runs conflict, and `cancel-in-progress` based on whether you want to queue or replace in-progress runs.

---

### Q7. How do you implement environment protection rules in GitHub Actions?

**Interview Answer**

GitHub Actions environments provide protection rules that control when workflows can deploy to specific environments. Create environments in Settings → Environments, then configure: required reviewers (must be approved before deployment), wait timer (delay before deployment proceeds), and deployment branches (restrict which branches can deploy). In workflows, reference the environment with `environment: production`, and the protection rules are enforced automatically. For Rust services, use a staging environment with no protection (automatic deployment on merge to main) and a production environment with required reviewers (manual approval before deployment). Environment secrets are only available to workflows targeting that environment, providing additional security isolation. Combine with OIDC tokens for cloud provider authentication — the environment restricts who can deploy, and OIDC restricts what the deployment can access.

---

### Q8. How do you debug failed GitHub Actions workflows?

**Interview Answer**

Start by checking the workflow run in the Actions tab — click on the failed job to see which step failed and its output. Use `run: echo "::debug::message"` for debug output, or enable debug logging by setting `ACTIONS_STEP_DEBUG=true` in repository secrets. For Rust-specific failures, check cargo output for compilation errors, test failures, or clippy warnings. Use `actions/upload-artifact` to persist build outputs (test results, coverage reports) for post-mortem analysis. For Docker build failures, use `docker buildx` with `--progress=plain` for detailed build output. Reproduce failures locally with `act` (runs GitHub Actions locally using Docker). Add debugging steps like `run: ls -la target/` to inspect filesystem state. For intermittent failures, check runner logs and consider if network issues, resource limits, or race conditions are the cause. Enable step debug logging to see the exact commands executed by each action.

---

### Q9. How do you optimize GitHub Actions workflow execution time?

**Interview Answer**

Optimize execution time through: caching (restore Cargo dependencies and build artifacts), parallelism (run independent jobs simultaneously), path filters (only trigger on relevant file changes), minimal checkout (use `fetch-depth: 1` for shallow clones), self-hosted runners (faster than GitHub-hosted for heavy workloads), and artifact reuse (pass build outputs between jobs instead of rebuilding). For Rust, caching is the biggest win — `Swatinem/rust-cache` can reduce build times from 15+ minutes to under 5 minutes. Use `concurrency` with `cancel-in-progress: true` to avoid wasting time on superseded runs. For monorepos, use path filters to only run CI for changed services. Monitor execution time with GitHub's workflow analytics and set performance budgets — if CI takes over 10 minutes, investigate bottlenecks. Consider `sccache` for shared compilation caching across team members.

---

### Q10. How do you implement automated releases with GitHub Actions?

**Interview Answer**

Create a release workflow triggered on tag push (`on: push: tags: ['v*']`) that builds artifacts, creates release notes, and publishes to package registries. For Rust, the workflow: checks out the tagged commit, builds release binaries for multiple platforms (using matrix strategy for linux/mac/windows), creates a GitHub Release with auto-generated release notes, and uploads binaries as release assets. For Docker images, tag the image with both the Git SHA and semantic version, push to the container registry, and optionally update the Helm chart values. Use `actions/create-release` or `gh release create` for GitHub Releases, and `softprops/action-gh-release` for uploading artifacts. Automate Cargo.toml version bumping with `cargo-release` or `cargo-edit`. For Rust libraries, publish to crates.io with `cargo publish`. Ensure the release workflow runs only on tags to prevent accidental releases from regular commits.