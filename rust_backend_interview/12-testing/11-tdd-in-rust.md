# Test-Driven Development in Rust

## Interview Question

How do you practice test-driven development (TDD) in Rust, and what are the benefits and challenges?

## Interview Answer

TDD follows the red-green-refactor cycle: write a failing test (red), implement the minimum code to pass (green), then improve the code without changing behavior (refactor). In Rust, this cycle works well because the type system catches many errors at compile time. You start with `#[test]` functions that describe expected behavior, watch them fail, implement the logic, and then clean up the code. Rust's ownership model forces you to think about data flow upfront, making the design phase more deliberate. The compiler becomes an ally in the refactor step, catching regressions immediately.

---

## Follow-up Questions & Answers

### Q1. What is the red-green-refactor cycle in detail?

**Interview Answer**

Red: write a test that describes the next behavior you want. Run `cargo test` and watch it fail — confirm the test is testing the right thing. Green: write the simplest code that makes the test pass. Don't optimize or over-engineer. Refactor: improve the code structure, remove duplication, and apply patterns. Run tests again to ensure nothing broke. Repeat this cycle for each small behavior increment. Each cycle should take 5-15 minutes.

---

### Q2. How does Rust's type system help with TDD?

**Interview Answer**

Rust's type system catches many errors before tests even run. If your test won't compile, you immediately know the types don't match your intended design. Enums force you to handle all cases, so you can't forget error paths. The borrow checker ensures data flow correctness. Traits guide interface design. This compile-time feedback makes the red phase more about behavior than syntax, and the green phase more about logic than type errors.

---

### Q3. What are the challenges of TDD in Rust?

**Interview Answer**

Rust's stricter type system means more upfront work to satisfy the compiler before tests can run. Async code adds complexity with runtime setup. Some patterns (especially those involving lifetimes) are harder to test incrementally. Compile times can slow down the red-green cycle. Macros and derive attributes generate code that's difficult to test directly. Despite these challenges, the type system's safety guarantees often result in fewer bugs reaching integration tests.

---

### Q4. How do you TDD an axum HTTP handler?

**Interview Answer**

Start by writing a test that sends a request and asserts the response. Use `axum::Router` with `tower::ServiceExt::oneshot` for in-process testing. The test will fail because the handler doesn't exist yet. Implement the handler to satisfy the test. Add more tests for edge cases: invalid input, missing authentication, database errors. Refactor handlers to extract business logic into testable service functions. Use mock state for dependencies.

---

### Q5. How do you TDD database queries in Rust?

**Interview Answer**

Write a test that calls a repository function and asserts on the result. Use `sqlx` with a test database or in-memory SQLite. The first test should fail because the function doesn't exist or returns wrong data. Implement the query to pass the test. Add tests for edge cases: empty results, multiple rows, null values. Use migrations to set up the test schema. Roll back transactions after each test for isolation.

---

### Q6. How do you handle compile errors during the red phase?

**Interview Answer**

When a test won't compile because the function doesn't exist, create a minimal stub that satisfies the type signature (return `todo!()` or a default value). This lets the test compile and fail at runtime, confirming the test logic is correct. Then implement the actual logic. Use `todo!()` liberally during TDD — it's a placeholder that makes the code compile while you focus on the test-driven design.

---

### Q7. How do you TDD async Rust code?

**Interview Answer**

Use `#[tokio::test]` for async tests. Start with a test that calls an async function and asserts on the result. Create the async function stub with `todo!()`. Implement the async logic to pass the test. For services with async dependencies, mock them with async trait implementations. Use `tokio::select!` and timeout utilities to test concurrent behavior. Keep async test setup minimal to avoid complexity.

---

### Q8. What is the London school vs. Chicago school of TDD?

**Interview Answer**

The London school (mockist) uses mocks for all collaborators, testing interactions between objects. The Chicago school (classicist) uses real objects where possible, testing state changes. In Rust, the type system naturally pushes toward Chicago-school TDD because real objects are often as easy as mocks. Use London-school when testing interaction-heavy code (event systems, orchestrators) and Chicago-school when testing data transformations and business logic.

---

### Q9. How do you refactor safely during TDD?

**Interview Answer**

Run all tests to ensure they pass before refactoring. Make small, incremental changes — rename a function, extract a module, simplify a conditional. Run tests after each change. Use Rust's compiler as a safety net — if it compiles and tests pass, the behavior hasn't changed. Use `cargo clippy` for additional refactoring suggestions. Commit frequently during refactoring so you can revert if needed. Never refactor and add features simultaneously.

---

### Q10. How do you write good test names in TDD?

**Interview Answer**

Use descriptive names that explain the scenario and expected outcome: `test_returns_error_when_user_not_found`, `test_creates_order_with_valid_items`, `test_applies_discount_for_premium_users`. Name tests as sentences with underscores. The test name should document the business rule being tested. When a test fails, the name should immediately tell you what broke. Group related tests in modules within the test file.
