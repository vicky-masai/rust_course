# unwrap() vs ?

## Interview Question

unwrap() vs ?

## Interview Answer

"unwrap() panics on error. The ? operator propagates errors safely, making it the preferred choice in production."

---

## Follow-up Questions & Answers

### Q1. What is the difference between `unwrap()` and `expect()`?

**Interview Answer**

Both panic on `Err`, but `expect()` lets you provide a custom panic message describing what went wrong. In production code, `expect("Failed to read config")` gives much better diagnostics than a bare `unwrap()` when debugging failures.

---

### Q2. Can you use `?` inside `main()`?

**Interview Answer**

Yes, but only if `main()` returns `Result<T, E>` or `Option<T>`. Rust's standard library provides a `Termination` trait implementation for `Result`, so `fn main() -> Result<(), Box<dyn Error>>` works and the error is printed to stderr on failure.

---

### Q3. What happens if `unwrap()` panics inside a spawned thread?

**Interview Answer**

A panic inside a spawned thread causes that thread to unwind and terminate, but it does not crash the entire process unless you explicitly call `std::panic::set_hook` or use `catch_unwind`. The `JoinHandle` returned by `thread::spawn` will yield an `Err` when joined, allowing the parent to detect the failure.

---

### Q4. Why is `unwrap()` considered bad practice in library code?

**Interview Answer**

Libraries should propagate errors to the caller rather than deciding how to handle them, because the caller has better context for recovery. Using `unwrap()` in library code can crash the entire application on recoverable errors, violating the principle of least surprise and making the library harder to integrate.

---

### Q5. How does `?` work with custom error types?

**Interview Answer**

The `?` operator calls the `From` trait to convert the inner error type into the function's return error type. If your function returns `Result<T, MyError>` and an inner call returns `io::Error`, Rust auto-converts via `impl From<io::Error> for MyError`, which you can derive with `thiserror`.

---

### Q6. What is `unwrap_or` and how does it compare to `unwrap()`?

**Interview Answer**

`unwrap_or(default)` returns the inner value on `Ok` or falls back to the provided default on `Err`, without panicking. It's useful when a sensible default exists, like `let timeout = env::var("TIMEOUT").unwrap_or("30".to_string())`. This avoids both panics and verbose match expressions.

---

### Q7. Is there a performance difference between `unwrap()` and `?`?

**Interview Answer**

There is no meaningful performance difference in release builds because both paths are identical when the value is `Ok`—the compiler inlines everything. On the error path, `?` performs a conversion and early return while `unwrap()` calls `panic!`, which is far more expensive due to stack unwinding and allocation.

---

### Q8. When would you deliberately use `unwrap()` in production?

**Interview Answer**

You might use `unwrap()` when a failure is truly unrecoverable and indicates a bug, such as parsing a hard-coded constant string or initializing a `OnceLock`. Even then, `expect()` with a descriptive message is usually preferred. Prototyping and quick scripts also justify `unwrap()` for simplicity.

---

### Q9. How does the `?` operator interact with `Option` return types?

**Interview Answer**

When used in a function that returns `Option<T>`, the `?` operator converts `None` into an early `None` return. For example, `let val = some_option?;` inside `fn foo() -> Option<T>` will short-circuit and return `None` if `some_option` is `None`. This mirrors its behavior with `Result` but for optional values.

---

### Q10. What is the `Try` trait and how does it relate to `?`?

**Interview Answer**

The `Try` trait (stabilized as `Try` in nightly) is what the `?` operator actually uses under the hood. It defines `into_result()`, `from_error()`, and `from_ok()` methods that let custom types work with `?`. This is how `Result` and `Option` both support the `?` operator with different semantics.

---
