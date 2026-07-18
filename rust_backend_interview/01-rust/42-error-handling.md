# Error Handling

## Interview Question

Error Handling.

## Interview Answer

"I use Result, custom error types with thiserror, and anyhow for application-level errors."

---

## Follow-up Questions & Answers

### Q1. When should you use `thiserror` vs `anyhow`?

**Interview Answer**

Use `thiserror` for library code where you want callers to match on specific error variants. Use `anyhow` for application code where you just need to propagate and display errors. Libraries should never use `anyhow` because it erases error types, making it impossible for callers to handle specific failures programmatically.

---

### Q2. How do you create a custom error enum with `thiserror`?

**Interview Answer**

Derive `thiserror::Error` on an enum with `#[error("...")]` attributes on each variant. Use `#[from]` for automatic conversion from inner errors and `#[source]` to chain causes. For example: `#[error("database error")] Db(#[from] sqlx::Error)`. This generates `From` impls and `Display` automatically.

---

### Q3. What is error chaining and how does Rust support it?

**Interview Answer**

Error chaining links one error to its cause via the `source()` method from `std::error::Error`. In `thiserror`, `#[source]` marks a field as the cause. When printing the error chain with `{:?}` or using `anyhow::Report`, the full chain is displayed, making it easy to trace errors from high-level operations down to the root cause.

---

### Q4. How do you convert between error types?

**Interview Answer**

Use `From` trait implementations, which `thiserror` can auto-derive with `#[from]`. For ad-hoc conversions, use `.map_err(|e| MyError::from(e))` or `.map_err(|e| MyError::Custom(e.into()))`. The `?` operator automatically calls `From::from` on the error, so conversions happen implicitly at return points.

---

### Q5. What is the `map_err` pattern and when is it useful?

**Interview Answer**

`map_err` transforms one error type into another while keeping the `Result` structure. It's useful when wrapping external crate errors into your domain errors: `db.query().map_err(|e| AppError::Database(e.to_string()))`. This preserves the error chain while giving your application a unified error type.

---

### Q6. How do you handle errors in async code?

**Interview Answer**

Use `?` in async functions just like synchronous code—async functions return `Result`. For spawned tasks, `tokio::spawn` returns `JoinHandle<T>`, and you handle the `JoinError` separately from the task's own errors. Use `.await?` to propagate errors up the async call stack, and `anyhow`'s context method to add trace information.

---

### Q7. What is the `context` method in `anyhow`?

**Interview Answer**

`.context("msg")` wraps an error with additional context, converting any `Error` into `anyhow::Error`. It's useful for adding layer-specific messages: `file.open().context("Failed to open config")`. When printed, the full context chain shows the sequence of operations that led to the error, which is invaluable for debugging.

---

### Q8. Can you use `?` with `Option` in a function that returns `Result`?

**Interview Answer**

Yes, using `.ok_or()` or `.ok_or_else()` to convert `None` to an error first: `let val = option.ok_or(MyError::MissingField)?;`. Alternatively, the `Option::ok_or` and `Option::ok_or_else` methods bridge the gap. There's no direct `?` conversion from `Option` to `Result` without this explicit step.

---

### Q9. What are the pitfalls of `Box<dyn Error>` as an error type?

**Interview Answer**

`Box<dyn Error>` erases the concrete type, making pattern matching impossible. It also requires `'static` lifetimes, so borrowing error data is tricky. Performance is worse due to heap allocation and dynamic dispatch. In production code, prefer concrete error enums with `thiserror` for type safety and better error handling.

---

### Q10. How do you test error cases in Rust?

**Interview Answer**

Use `assert!(result.is_err())` or `assert_matches!` to verify the error variant. With `thiserror`, you can match on specific variants: `assert!(matches!(err, AppError::NotFound(_)))`. For `anyhow`, use `err.downcast_ref::<MyError>()`. Test both the error message content and that the correct variant is produced for different inputs.

---
