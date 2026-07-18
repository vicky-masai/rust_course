# Property-Based Testing in Rust

## Interview Question

What is property-based testing, and how do you use it in Rust to improve test quality?

## Interview Answer

Property-based testing defines properties (invariants) that should hold for all valid inputs, then a framework generates hundreds of random inputs to verify those properties. Instead of testing specific cases, you test general rules like "serializing then deserializing returns the original value." In Rust, `proptest` and `quickcheck` are the primary crates for this approach. They generate random inputs, shrink failing cases to minimal reproductions, and run extensive input exploration. This technique finds edge cases that manual test case design often misses.

---

## Follow-up Questions & Answers

### Q1. What is the difference between property-based testing and fuzzing?

**Interview Answer**

Property-based testing verifies that defined properties hold across many generated inputs — it's specification-driven. Fuzzing feeds random or mutated inputs to find crashes, panics, or undefined behavior — it's crash-driven. Property-based testing is more targeted (you define what should be true), while fuzzing is broader (anything that crashes is a bug). Both use random input generation but serve different purposes. In Rust, `proptest` is for property testing and `cargo-fuzz` is for fuzzing.

---

### Q2. How do you use the `proptest` crate in Rust?

**Interview Answer**

Add `proptest` as a dev-dependency, define strategies for generating test data with `prop_compose!`, and use `proptest!` macro to write test cases. Strategies define how to generate random values (e.g., `any::<u32>()`, `"[a-z]{1,10}"`, custom struct generators). The macro runs the property check hundreds of times with different inputs. On failure, proptest automatically shrinks the input to find the minimal failing case.

---

### Q3. What is shrinking and why is it important?

**Interview Answer**

Shrinking is the process of reducing a failing input to the smallest or simplest case that still triggers the failure. When proptest finds a counterexample, it automatically tries smaller variants to find the root cause. This makes debugging much easier — instead of a complex 1000-element vector, you might get a 2-element vector that clearly shows the bug. Not all frameworks support shrinking; proptest does it automatically for most strategies.

---

### Q4. How do you define custom strategies for complex types?

**Interview Answer**

Use `prop_compose!` to combine simpler strategies into complex ones. For example, generate a `User` struct by composing `name: "[A-Z][a-z]{2,10}"`, `age: 0..150u32`, `email: "[a-z]+@[a-z]+\\.com"`. You can use `prop_flat_map` for dependent generation and `prop_oneof!` to randomly select between different generators. Strategies can also be derived using `#[derive(Arbitrary)]` with the `arbitrary` crate.

---

### Q5. What properties should you test in a backend service?

**Interview Answer**

Test serialization round-trips: `serde_json::from_str(&serde_json::to_string(&x)) == x`. Test API idempotency: calling the same endpoint twice produces the same result. Test database operations: insert then select returns the same data. Test business rules: valid inputs always produce valid outputs. Test encoding/decoding: base64 encode then decode returns the original. Test ordering: sorted output contains all input elements.

---

### Q6. How do you handle flaky property-based tests?

**Interview Answer**

Set a fixed seed for reproducibility during debugging with `ProptestConfig`. Increase the number of test cases only when you have confidence in the test. Use `prop_assume!` to filter out invalid generated inputs that don't meet preconditions. If a test is consistently flaky, the property may be incorrectly defined. Fix the property definition rather than reducing test iterations. Use `#[cfg(test)]` with deterministic strategies when needed.

---

### Q7. What is `quickcheck` and how does it differ from `proptest`?

**Interview Answer**

`quickcheck` was the original property-based testing crate for Rust, inspired by Haskell's QuickCheck. It uses the `Arbitrary` trait for input generation and supports shrinking. `proptest` is more feature-rich with better shrinking, strategy composition, and error messages. `quickcheck` is simpler and lighter, while `proptest` offers more control. New projects typically prefer `proptest`, but `quickcheck` is still widely used in existing codebases.

---

### Q8. How do you use property-based testing with async code?

**Interview Answer**

Use `proptest` with `proptest-async` or `tokio::test` integration. Define async test functions within `proptest!` macro using the async variant. Some property tests don't need async — test pure functions synchronously. For async properties, ensure the proptest runtime and tokio runtime are properly configured. Use `#[tokio::test]` outside of proptest for simpler async unit tests.

---

### Q9. How do you integrate property-based testing into a test suite?

**Interview Answer**

Start with critical data transformations (serialization, parsing, encoding). Add properties for business rules that must always hold. Use property tests alongside unit tests — unit tests for known edge cases, properties for general invariants. Run property tests in CI with reasonable iteration counts (100-1000 cases). Track property test coverage as part of your overall test strategy. Use property tests to discover edge cases that inform new unit tests.

---

### Q10. What are the limitations of property-based testing?

**Interview Answer**

Property-based testing requires you to articulate properties explicitly, which can be difficult for complex business logic. Generated inputs may not reflect realistic production data. Shrinking may not work well for all custom types. The tests are non-deterministic by default, making debugging harder without seed control. Property tests are slower than unit tests due to hundreds of iterations. They complement but don't replace unit and integration tests.
