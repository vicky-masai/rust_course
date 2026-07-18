# Mutable Borrowing vs Immutable Borrowing in Rust

## Interview Question

Explain Mutable Borrowing vs Immutable Borrowing in Rust.

## Interview Answer

> "Rust supports two kinds of borrowing: immutable borrowing and mutable borrowing. Immutable borrowing allows multiple parts of the program to read the same data simultaneously because reading doesn't change the data. Mutable borrowing allows modifying the data, but Rust permits only one mutable reference at a time.
>
> This rule guarantees exclusive access during modification, preventing data races and inconsistent state. In backend applications, immutable borrowing is commonly used for request validation and configuration data, while mutable borrowing is used when updating objects before saving them to the database."

---

## Follow-up Questions & Answers

### 1. Why does Rust allow multiple immutable borrows?

**Interview Answer:**

> "Because immutable borrows only read data. Multiple readers cannot create inconsistent state."

---

### 2. Why is only one mutable borrow allowed?

**Interview Answer:**

> "A mutable borrow has exclusive access to the data. This prevents concurrent modifications and eliminates data races."

---

### 3. Can mutable and immutable borrows exist together?

**Interview Answer:**

> "No, not at the same time. Rust enforces exclusive access for mutable borrows."

---

### 4. What is exclusive access?

**Interview Answer:**

> "Exclusive access means only one mutable reference can modify a value while no other references can read or write it."

---

### 5. What problem does this solve?

**Interview Answer:**

> "It prevents race conditions, dangling references, and inconsistent state before the program runs."

---

### 6. What is Non-Lexical Lifetime (NLL)?

**Interview Answer:**

> "NLL allows a borrow to end after its last use instead of waiting until the end of the scope, making borrowing more flexible."

---

### 7. How does this improve backend performance?

**Interview Answer:**

> "It allows safe sharing of large objects without cloning while maintaining memory safety and eliminating synchronization bugs."

---

### 8. Is this checked at runtime?

**Interview Answer:**

> "No. The borrow rules are enforced entirely at compile time, so there is no runtime overhead."

---

### 9. How does this differ from Java or Go?

**Interview Answer:**

> "Java and Go rely on runtime synchronization for shared mutable data. Rust prevents many concurrency issues at compile time through its ownership and borrowing rules."

---

### 10. Where do you use mutable borrowing in real projects?

**Interview Answer:**

> "I use mutable borrowing when updating domain models, modifying request objects during processing, or changing application state before persisting it to the database."
