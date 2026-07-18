# Integration Testing in Rust

## Interview Question

How does Rust's integration testing system work, and how do you set up integration tests for a backend service?

## Interview Answer

Rust integration tests live in a `tests/` directory at the project root, with each `.rs` file being a separate integration test crate. Unlike unit tests, they can only access your library's public API through your crate's name. You run them with `cargo test --test <test_name>` or just `cargo test` to run all of them. For backend services, integration tests often spin up a test server, make HTTP requests, and verify responses. You can use shared helper modules by creating a `tests/common/mod.rs` file and referencing it with `mod common;` in your test files.

---

## Follow-up Questions & Answers

### Q1. What is the difference between unit tests and integration tests in Rust?

**Interview Answer**

Unit tests are embedded in the source code using `#[cfg(test)]` modules and can access private items. Integration tests live in the `tests/` directory and can only test the public API of your library crate. Integration tests compile as separate crates, providing a realistic consumer perspective. This separation encourages clean API design and ensures tests validate external contracts.

---

### Q2. How do you share test setup code across integration tests?

**Interview Answer**

Place shared utilities in `tests/common/mod.rs` and import them with `mod common;` in each integration test file. This file is not treated as a test itself because it doesn't contain `#[test]` functions. You can create setup functions for database seeding, server startup, or test data generation. Some projects also use a shared test fixtures crate as a dev-dependency.

---

### Q3. How do you test HTTP APIs in integration tests?

**Interview Answer**

Use `reqwest` to make HTTP requests to your running test server. Start your axum/actix-web server on a random port in a test setup function, then construct URLs with `format!("http://localhost:{port}/endpoint")`. Use `tokio::runtime::Runtime` or `#[tokio::test]` for async tests. Libraries like `axum::test` or `tower::ServiceExt::oneshot` can test handlers without starting a full server.

---

### Q4. How do you handle test databases in integration tests?

**Interview Answer**

Create a dedicated test database that gets reset between test runs. Use migration scripts in your test setup to ensure a clean schema. Many teams use `docker-compose` to spin up a PostgreSQL container specifically for tests. Alternatively, use `testcontainers` to programmatically manage database lifecycle. Each test should run in a transaction that gets rolled back to avoid cross-test contamination.

---

### Q5. What are testcontainers and when should you use them?

**Interview Answer**

Testcontainers is a library that provides lightweight, disposable Docker containers for integration testing. It supports databases, message brokers, and other services. You create a container in your test, wait for it to be ready, extract connection details, run the test, and the container is automatically cleaned up. This eliminates "works on my machine" issues and ensures tests run consistently in CI without external dependencies.

---

### Q6. How do you run integration tests in CI/CD pipelines?

**Interview Answer**

Add `cargo test --all` to your CI pipeline steps. For tests requiring services, use Docker Compose or testcontainers to provision dependencies. Cache Docker images and Cargo dependencies to speed up builds. Use separate CI stages for unit tests, integration tests, and end-to-end tests. Set `DATABASE_URL` and other environment variables from CI secrets. Consider using `cargo nextest` for faster parallel test execution.

---

### Q7. How do you test error scenarios in integration tests?

**Interview Answer**

Test both success and failure paths by sending invalid inputs, missing authentication, and malformed requests. Use `assert_eq!(response.status(), 400)` to verify correct HTTP status codes. Test rate limiting by sending rapid requests and checking for 429 responses. Verify that error responses have proper JSON error structures and that sensitive information isn't leaked in error messages.

---

### Q8. What is the `tests/common/mod.rs` pattern and why does Rust treat it specially?

**Interview Answer**

Rust treats files directly in `tests/` as integration test crates but ignores files in subdirectories unless they are referenced as modules. By placing shared code in `tests/common/mod.rs`, it won't be compiled as a separate test binary. This is Rust's convention for shared test utilities. You import it with `mod common;` and access functions like `common::setup_database()`.

---

### Q9. How do you test async code in integration tests?

**Interview Answer**

Use `#[tokio::test]` as the test attribute instead of `#[test]` to get an async runtime. This requires adding `tokio` with the `macros` and `rt-multi-thread` features as a dev-dependency. Each async test function runs on a separate tokio runtime. Be careful with shared state — use `Arc<Mutex<>>` or `tokio::sync` primitives for concurrent access in async test contexts.

---

### Q10. How do you handle test ordering and dependencies between integration tests?

**Interview Answer**

Cargo runs tests in parallel by default, so tests should be independent and idempotent. Never rely on test execution order. Use unique test data (like UUIDs) per test to avoid collisions. If you need sequential execution, use file locks or database locks, but prefer designing tests to be independent. The `--test-threads=1` flag forces sequential execution but is a code smell if relied upon.
