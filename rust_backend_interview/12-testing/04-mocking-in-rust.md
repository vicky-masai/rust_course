# Mocking in Rust

## Interview Question

How do you implement mocking in Rust, and what are the trade-offs between different mocking approaches?

## Interview Answer

Rust mocking primarily relies on trait-based abstractions using the `mockall` crate or hand-written test doubles. The `mockall` crate auto-generates mock structs from trait definitions with `#[automock]`. You can also write manual mock implementations that return canned responses. The key trade-off is that Rust requires explicit trait abstractions for mocking, which adds upfront design cost but produces cleaner interfaces. Alternatives include `mockito` for HTTP mocking and `wiremock` for more expressive request matching.

---

## Follow-up Questions & Answers

### Q1. How does the `mockall` crate work?

**Interview Answer**

`mockall` generates mock implementations from trait definitions using the `#[automock]` attribute or `mock!` macro. It creates a struct with the same methods as the trait that can record expectations and return preset values. You set expectations with `expect_method_name().returning(value)`. The mock verifies all expectations were met when dropped. It works well with async traits and generic functions.

---

### Q2. What is the difference between mocks, stubs, and fakes?

**Interview Answer**

Mocks verify interactions — you assert that specific methods were called with specific arguments. Stubs provide canned responses without verifying how they were called. Fakes are simplified working implementations (like an in-memory database) used in place of real dependencies. Mocks are behavior-focused, stubs are output-focused, and fakes are state-focused. In Rust, `mockall` creates mocks, while fakes are hand-written structs implementing the same trait.

---

### Q3. When should you prefer dependency injection over mocking?

**Interview Answer**

Dependency injection through trait parameters makes your code more testable and follows the Dependency Inversion Principle. Use DI when you need to swap entire implementations (real vs. test database). Use mocking when you need to verify specific interactions or control call sequencing. DI with fakes is often better than mocking because fakes are simpler and less brittle. Reserve mocks for cases where verifying interaction patterns is critical.

---

### Q4. How do you mock async functions in Rust?

**Interview Answer**

`mockall` supports async traits natively — just define `async fn` in your trait and the generated mock handles it. For manual mocks, return `BoxFuture` or use `async_trait` macro. With `wiremock`, you set up mock HTTP servers that respond to specific requests. The `tokio::test` macro provides the async runtime needed to execute async mock tests.

---

### Q5. What are the limitations of mocking in Rust?

**Interview Answer**

Rust doesn't have runtime trait implementation like Java, so mocking requires compile-time trait abstractions. Functions not behind traits are difficult to mock without wrapper types. Mocking closures and free functions requires additional indirection. Over-mocking leads to fragile tests that break with implementation changes. Rust's ownership model makes mocking objects with complex lifetimes challenging. Some patterns like mocking `std::fs::File` require crate-level shims.

---

### Q6. How do you mock HTTP services in Rust tests?

**Interview Answer**

`wiremock` provides a mock HTTP server that can match requests by method, path, headers, and body. Start a `MockServer`, set up `Mock::given(method).and(path).respond_with(ResponseTemplate::new(200).set_body_json(...))`, and point your HTTP client at the mock server's URL. It supports recording and verifying requests. `mockito` is another option with a similar API. Both are useful for testing code that calls external APIs.

---

### Q7. How do you test error paths with mocks?

**Interview Answer**

Set up mock expectations to return `Err` variants for specific method calls. Use `returning(Err(MyError::NotFound))` or `panic!("unexpected call")` for methods that shouldn't be called. Test retry logic by setting up sequential expectations where the first call fails and the second succeeds. Use `times(n)` to control how many times a method should be called. This validates that your error handling code correctly propagates or recovers from failures.

---

### Q8. What is the `mockall::expectation!` macro and when would you use it?

**Interview Answer**

The `expectation!` macro allows creating ad-hoc expectations without a full trait definition. It's useful for one-off mocking scenarios where defining a trait is overkill. You can set return values, call counts, and argument matchers inline. It's more flexible than `#[automock]` but less discoverable. Use it when you need to mock a specific function signature in a single test without creating a full trait abstraction.

---

### Q9. How do you avoid over-mocking in your test suite?

**Interview Answer**

Follow the principle of testing behavior, not implementation. Mock at architectural boundaries (external services, databases), not internal components. Use real objects when they're fast and deterministic. Prefer integration tests over heavily mocked unit tests. If a test requires mocking more than 2-3 things, consider whether the code is too coupled. Use fakes over mocks when a simple in-memory implementation suffices.

---

### Q10. How do you test code that uses global state or singletons?

**Interview Answer**

Refactor global state behind traits so it can be injected and mocked. Use `thread_local!` or `OnceCell` to manage global state in a way that can be reset between tests. In tests, set up the global state to a known value before each test and tear it down after. Alternatively, use environment variables or feature flags to switch between real and test configurations. The `std::sync::OnceLock` type can be used for lazy initialization that's resettable in tests.
