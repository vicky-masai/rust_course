# Unit Testing in Rust

## Interview Question

How do you write unit tests in Rust, and how do you organize them within your codebase?

## Interview Answer

Rust has built-in unit testing support via the `#[cfg(test)]` attribute and the `#[test]` macro. Unit tests live in the same file as the code they test, inside a `mod tests` block annotated with `#[cfg(test)]` so they are only compiled during `cargo test`. You use assertion macros like `assert!`, `assert_eq!`, `assert_ne!`, and `panic!` to validate behavior. Tests are organized by marking them with `#[test]`, and you can use `#[ignore]` to skip slow tests by default. Private functions are accessible within the same module's test block, making unit testing straightforward without needing to expose internal APIs.

---

## Follow-up Questions & Answers

### Q1. What is the difference between `assert_eq!` and `assert!`?

**Interview Answer**

`assert!` checks that a boolean expression evaluates to `true` and panics with no useful output on failure. `assert_eq!` checks that two values are equal and, on failure, prints both values using their `Debug` formatting, making debugging much easier. There is also `assert_ne!` which checks inequality. When using `assert_eq!`, both types must implement `PartialEq` and `Debug`.

---

### Q2. How do you run specific tests using cargo?

**Interview Answer**

You can run a specific test by name using `cargo test <test_name>`, which runs all tests whose name contains the given string. To run tests in a specific module, use `cargo test <module_name>::`. To run a single exact test, use `cargo test -- --exact <full_test_name>`. The `--` separator passes arguments to the test binary rather than to cargo itself.

---

### Q3. What are test helpers and how do you organize them?

**Interview Answer**

Test helpers are shared utility functions used across multiple tests. In Rust, you can place them in a `#[cfg(test)]` module or a separate `tests/common/mod.rs` file. A common pattern is creating a `setup()` function that initializes test fixtures, database connections, or mock data. Using `#[cfg(test)]` ensures helpers are only compiled during testing and don't bloat production binaries.

---

### Q4. How do you test code that returns `Result` types?

**Interview Answer**

You can annotate a test with `#[test]` and have it return `Result<(), Box<dyn std::error::Error>>`. If the function returns `Err`, the test automatically fails and prints the error. This is cleaner than using `.unwrap()` inside tests because it gives meaningful error messages. You can also use `#[should_panic]` for testing code that is expected to panic rather than return an error.

---

### Q5. What is `#[ignore]` and when should you use it?

**Interview Answer**

The `#[ignore]` attribute tells the test runner to skip a test during normal `cargo test` execution. It's useful for slow tests, tests requiring external services, or tests that need specific environment variables. You can run ignored tests explicitly with `cargo test -- --ignored`. You can also add `#[ignore = "reason"]` to document why the test is ignored.

---

### Q6. How do you test private functions in Rust?

**Interview Answer**

In Rust, unit tests defined in the same module as the code can access private functions because modules have visibility into their own items. You place a `mod tests` block inside the module containing the private function, and then write tests that call it directly. This is a key advantage of Rust's testing model — you don't need to make functions public just to test them.

---

### Q7. What are some common assertion macros beyond `assert_eq!`?

**Interview Answer**

Beyond `assert_eq!`, Rust provides `assert_ne!` for inequality checks, `debug_assert!`, `debug_assert_eq!`, and `debug_assert_ne!` which only run in debug builds. The `matches!` macro is useful for pattern matching in assertions. You can also use the `pretty_assertions` crate for colorful diff output on equality failures, and `rstest` for parameterized tests.

---

### Q8. How do you structure test modules for a large Rust file?

**Interview Answer**

For large files, use nested `mod tests` blocks to group related tests by functionality or sub-component. Name each submodule descriptively, such as `mod parsing_tests` or `mod serialization_tests`. Keep the `tests` module at the bottom of the file. Use `use super::*;` to import all items from the parent module. This structure mirrors the code organization and makes it easy to find related tests.

---

### Q9. How do you test code that depends on the current time?

**Interview Answer**

Directly calling `std::time::Instant::now()` or `chrono::Utc::now()` in production code makes testing difficult. The solution is to inject the clock as a parameter — define a trait like `Clock` with a `now()` method, provide a real implementation and a mock implementation. During tests, use the mock clock to control the time. Alternatively, crates like `fake-timers` or the `mockall` crate can intercept time-dependent calls.

---

### Q10. What is the `#[should_panic]` attribute and how do you use it?

**Interview Answer**

`#[should_panic]` tells the test runner that the test is expected to panic — if it doesn't panic, the test fails. You can optionally provide an expected message with `#[should_panic(expected = "error message")]` to verify the correct panic occurred. This is useful for testing input validation, unwrap calls on invalid data, and any code path that should halt execution. Always use it sparingly and prefer `Result`-based error handling where possible.
