# Interior Mutability (`Cell<T>` and `RefCell<T>`) in Rust

## Interview Question

Explain Interior Mutability (`Cell<T>` and `RefCell<T>`) in Rust.

## Interview Answer

> "Interior mutability is a Rust pattern that allows data to be mutated even when accessed through an immutable reference. This is achieved using types like `Cell<T>` and `RefCell<T>`.
>
> `Cell<T>` is designed for simple `Copy` types and provides mutation by replacing values rather than borrowing them.
>
> `RefCell<T>` performs borrow checking at runtime instead of compile time. It allows either multiple immutable borrows or one mutable borrow, and if the borrowing rules are violated, it panics at runtime.
>
> `Cell` and `RefCell` are intended for single-threaded scenarios. For multithreaded applications, I use synchronization primitives like `Mutex` or `RwLock`."

---

## Follow-up Questions & Answers

### Q1. What is Interior Mutability?

**Interview Answer:**

> "Interior mutability allows data inside an immutable object to be modified safely using types like `Cell` and `RefCell`."

---

### Q2. What is `Cell<T>`?

**Interview Answer:**

> "`Cell<T>` provides interior mutability for `Copy` types by replacing values instead of returning references."

---

### Q3. What is `RefCell<T>`?

**Interview Answer:**

> "`RefCell<T>` provides interior mutability with runtime borrow checking, allowing mutable and immutable borrows while enforcing Rust's borrowing rules during execution."

---

### Q4. What's the difference between `Cell` and `RefCell`?

**Interview Answer:**

> "`Cell` works by copying values and is suitable for `Copy` types. `RefCell` supports borrowing non-`Copy` values and performs borrow checking at runtime."

---

### Q5. What's the difference between `RefCell` and `Mutex`?

**Interview Answer:**

> "`RefCell` is designed for single-threaded code and enforces borrowing rules at runtime. `Mutex` provides thread-safe synchronization for shared mutable data across multiple threads."

---

### Q6. Why can `RefCell` panic?

**Interview Answer:**

> "Because borrow rules are checked at runtime. Attempting multiple mutable borrows or mixing mutable and immutable borrows incorrectly causes a panic."

---

### Q7. Is `RefCell` thread-safe?

**Interview Answer:**

> "No. `RefCell` is not `Send` or `Sync` and should only be used in single-threaded contexts."

---

### Q8. Where do you use `RefCell`?

**Interview Answer:**

> "I primarily use it in unit tests, mock implementations, lazy caches, and scenarios requiring interior mutability in single-threaded code."

---

### Q9. Should you use `RefCell` in Axum handlers?

**Interview Answer:**

> "Generally no. Axum applications are typically multithreaded, so shared mutable state should use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` instead."

---

### Q10. When would you choose `Cell`, `RefCell`, or `Mutex`?

**Interview Answer:**

> "I use `Cell` for simple `Copy` values, `RefCell` for interior mutability in single-threaded code, and `Mutex` or `RwLock` for thread-safe shared mutable state in backend applications."
