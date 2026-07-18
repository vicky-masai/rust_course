# `Option<T>` in Rust

## Interview Question

Explain `Option<T>` in Rust.

## Interview Answer

> "Option<T> is Rust's way of representing an optional value. Instead of using null pointers, Rust uses the Option enum, which has two variants: Some, representing a valid value, and None, representing the absence of a value.
>
> Since Rust forces developers to explicitly handle both cases, it prevents null pointer exceptions at compile time. I commonly use Option when searching for data, accessing collections, parsing optional values, or working with database queries where a result may or may not exist."

---

## Follow-up Questions & Answers

### Q1. Why doesn't Rust have null?

**Interview Answer:**

> "Rust replaces null with Option<T> to eliminate null pointer exceptions and force developers to handle missing values explicitly."

---

### Q2. What are the variants of Option?

**Interview Answer:**

> "`Option<T>` has two variants: `Some(T)` for a value and `None` for no value."

---

### Q3. When should you use Option?

**Interview Answer:**

> "Whenever a value may or may not exist, such as database lookups, HashMap access, optional request fields, or search operations."

---

### Q4. What is the difference between Option and Result?

**Interview Answer:**

> "Option represents the presence or absence of a value. Result represents success or failure with an associated error."

---

### Q5. Why should you avoid `unwrap()` in production?

**Interview Answer:**

> "Because calling `unwrap()` on `None` causes a panic, which can crash the application."

---

### Q6. What is a safer alternative to `unwrap()`?

**Interview Answer:**

> "`unwrap_or()`, `unwrap_or_else()`, `match`, or `if let`, depending on the use case."

---

### Q7. Does Option allocate memory on the heap?

**Interview Answer:**

> "No. `Option` itself is an enum and usually does not require heap allocation unless the contained type does."

---

### Q8. Can Option contain any type?

**Interview Answer:**

> "Yes. `Option<T>` is generic and can wrap any type, such as `Option<String>`, `Option<User>`, or `Option<Vec<T>>`."

---

### Q9. How is Option used in backend applications?

**Interview Answer:**

> "It's commonly used for database queries, optional request parameters, cache lookups, configuration values, and search operations where data may not exist."

---

### Q10. Why is Option considered safer than null?

**Interview Answer:**

> "Because the compiler forces developers to handle both `Some` and `None`, preventing null pointer exceptions at runtime."
