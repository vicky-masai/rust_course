# Test Coverage in Rust

## Interview Question

How do you measure and improve test coverage in a Rust project?

## Interview Answer

The primary tool for measuring test coverage in Rust is `cargo-tarpaulin`, which instruments your code and reports line, branch, and function coverage. Run it with `cargo tarpaulin --out Html` to generate an HTML report. Coverage metrics help identify untested code paths, but high coverage doesn't guarantee correctness — focus on meaningful coverage of critical business logic. A good target is 80-90% for application code, while lower coverage for infrastructure code is acceptable. Combine coverage reports with mutation testing using `cargo-mutants` to assess test quality.

---

## Follow-up Questions & Answers

### Q1. How do you install and use cargo-tarpaulin?

**Interview Answer**

Install with `cargo install cargo-tarpaulin`. Run `cargo tarpaulin` to generate a terminal report, or `cargo tarpaulin --out Html` for an HTML report. Use `--skip-clean` to speed up repeated runs. You can exclude files with `--exclude-files` and focus on specific packages with `--packages`. For CI, use `--out xml` or `--out json` to generate machine-readable reports that integrate with coverage services like Codecov.

---

### Q2. What is the difference between line coverage and branch coverage?

**Interview Answer**

Line coverage measures whether each line of code was executed during tests. Branch coverage measures whether each branch (if/else, match arms, error paths) was taken. Branch coverage is more thorough because a line might be covered while certain branches within it are not. For example, `if condition { A } else { B }` might have 100% line coverage but only 50% branch coverage if only the `true` path is tested.

---

### Q3. What metrics should you track for test coverage?

**Interview Answer**

Track line coverage, branch coverage, function coverage, and mutation score. Line coverage tells you what percentage of code was executed. Branch coverage ensures conditional logic is tested in both directions. Function coverage verifies all functions are called. Mutation score (from cargo-mutants) measures whether your tests can detect injected bugs. Together, these metrics provide a comprehensive view of test quality.

---

### Q4. How do you improve test coverage for low-coverage areas?

**Interview Answer**

Identify uncovered code using the coverage report's highlighted lines. Focus on critical business logic, error handling paths, and edge cases first. Add tests for uncovered match arms, unwrap/expect calls, and error branches. For complex logic, use property-based testing to automatically explore code paths. Don't aim for 100% coverage — some code like boilerplate, logging, and trait implementations may not benefit from additional tests.

---

### Q5. What is mutation testing and how does it complement coverage?

**Interview Answer**

Mutation testing modifies your source code (mutants) and runs your tests to see if they catch the changes. If your tests still pass after a mutation, they're not sufficient to catch that type of bug. The mutation score is the percentage of mutants killed by tests. `cargo-mutants` automates this for Rust. High line coverage with low mutation score indicates tests that execute code but don't verify behavior effectively.

---

### Q6. How do you integrate coverage into CI/CD pipelines?

**Interview Answer**

Run `cargo tarpaulin --out xml` in your CI pipeline and upload results to Codecov, Coveralls, or a self-hosted solution. Set coverage thresholds to fail builds that drop below the target. Use `tarpaulin --fail-under 80` to fail if coverage drops below 80%. Cache tarpaulin's build artifacts to speed up CI. Generate badge URLs to display coverage status in your README. Consider running coverage on a schedule for full reports and on every PR for incremental changes.

---

### Q7. What are the limitations of code coverage metrics?

**Interview Answer**

Coverage doesn't measure test quality — a test that calls code but doesn't assert anything still counts as covered. Coverage can't detect missing tests for unimplemented features. Some code (macros, derive implementations, simple pass-through functions) may not benefit from explicit testing. Coverage metrics can create perverse incentives where developers write low-quality tests just to increase numbers. Always pair coverage with code review and mutation testing.

---

### Q8. How do you handle coverage for async code and macros?

**Interview Answer**

Async code coverage works with tarpaulin but may require `--tokio` flag or specific runtime configuration. Derive macros and procedural macros generate code that tarpaulin may not instrument correctly — test the behavior they produce rather than the macro code itself. Attribute macros like `#[instrument]` can be excluded with `--exclude-attributes`. For conditional compilation with `#[cfg]`, ensure tests run with the correct feature flags to cover all code paths.

---

### Q9. What is a good coverage strategy for a large Rust codebase?

**Interview Answer**

Start by measuring current coverage and setting realistic improvement targets. Enforce coverage thresholds on new code — require that PRs don't decrease overall coverage. Prioritize coverage for business-critical modules, public APIs, and error handling. Accept lower coverage for infrastructure code, boilerplate, and highly coupled integrations. Use coverage hotspots to guide testing efforts. Review coverage reports alongside code review to identify untested edge cases.

---

### Q10. How do you exclude code from coverage reports?

**Interview Answer**

Use `#[cfg(not(tarpaulin_include))]` to exclude specific functions or modules. In tarpaulin configuration, use `--exclude-files` patterns to skip entire files or directories. Create a `.tarpaulin.toml` file to configure exclusions persistently. Exclude test helper code, generated code, and trivial implementations. The `--exclude` flag removes specific packages from coverage calculations. Balance exclusions with completeness — don't exclude code that should be tested.
