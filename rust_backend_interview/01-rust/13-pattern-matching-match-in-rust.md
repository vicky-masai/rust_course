# Pattern Matching (`match`) in Rust

## Interview Question

Explain Pattern Matching (`match`) in Rust.

## Interview Answer

> "Pattern matching in Rust is implemented using the `match` expression. It allows us to compare a value against multiple patterns and execute the corresponding branch. Unlike switch statements in many languages, Rust's match is exhaustive, meaning every possible case must be handled.
>
> I commonly use match with Option, Result, and custom enums because it provides safe and readable control flow. Since the compiler verifies that all cases are covered, it helps prevent runtime errors caused by unhandled states."

---

## Follow-up Questions & Answers

### Q1. What is pattern matching in Rust?

**Interview Answer:**

> "Pattern matching allows us to compare a value against multiple patterns and execute the matching branch. It's implemented using the `match` expression."

---

### Q2. Why is `match` better than `switch`?

**Interview Answer:**

> "Rust's `match` supports enums, structs, tuples, ranges, guards, and destructuring. It also enforces exhaustive handling, making it safer than traditional switch statements."

---

### Q3. What does exhaustive matching mean?

**Interview Answer:**

> "It means every possible value or enum variant must be handled. If any case is missing, the compiler reports an error."

---

### Q4. What does `_` mean in a `match` statement?

**Interview Answer:**

> "`_` is the wildcard pattern. It matches any value that wasn't matched by previous patterns."

---

### Q5. Can `match` return a value?

**Interview Answer:**

> "Yes. `match` is an expression, so each branch can return a value that can be assigned to a variable."

---

### Q6. Where do you use `match` most in backend applications?

**Interview Answer:**

> "Primarily with `Option`, `Result`, authentication results, database queries, request validation, and state management."

---

### Q7. What happens if you forget one enum variant?

**Interview Answer:**

> "The compiler throws an error because `match` must handle every possible variant."

---

### Q8. Is `match` slower than `if-else`?

**Interview Answer:**

> "No. Rust optimizes `match` during compilation, and in many cases it generates very efficient machine code."

---

### Q9. When would you use `if-else` instead of `match`?

**Interview Answer:**

> "I use `if-else` for simple boolean conditions. I use `match` when working with enums, `Option`, `Result`, or multiple distinct patterns."

---

### Q10. Why is `match` considered one of Rust's strongest features?

**Interview Answer:**

> "Because it combines safety, readability, and compile-time exhaustiveness checking, making code easier to maintain and less error-prone."
