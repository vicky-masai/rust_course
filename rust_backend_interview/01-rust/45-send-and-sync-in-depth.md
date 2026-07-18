# Send and Sync in depth

## Interview Question

Explain Send and Sync in depth.

## Interview Answer

"`Send` allows ownership transfer across threads. `Sync` allows shared references to be safely accessed by multiple threads."

---

## Follow-up Questions & Answers

### Q1. How do you create a type that is `Send` but not `Sync`?

**Interview Answer**

Use `Cell<T>` or `RefCell<T>` inside a struct—these provide interior mutability without synchronization. `Cell<u32>` is `Send` but not `Sync` because concurrent reads/writes would race. This pattern is useful for thread-local counters or accumulators that are moved between threads but never shared.

---

### Q2. What is the relationship between `Sync` and `&T`?

**Interview Answer**

`T: Sync` is equivalent to `&T: Send`. This means if you can safely send a shared reference to another thread, the type is `Sync`. This is why `Rc<T>` is `!Sync`—you can't safely send `&Rc<T>` to another thread because the non-atomic reference count could race.

---

### Q3. How does `Pin<P>` interact with `Send` and `Sync`?

**Interview Answer**

`Pin<P>` is `Send` when `P: Send`, and `Sync` when `P: Sync`. However, pinned data may have self-referential pointers, so moving it after pinning is unsafe regardless of `Send`. The `Unpin` trait is auto-implemented for most types, allowing them to be safely unpinned and moved.

---

### Q4. Can `fn()` pointers be `Send` and `Sync`?

**Interview Answer**

Yes, `fn()` function pointers are both `Send` and `Sync` because they don't capture any state—they're just code addresses. However, closures that capture non-`Send` or non-`Sync` types inherit those bounds. A closure capturing `Rc<T>` is neither `Send` nor `Sync`.

---

### Q5. How do you handle types that are conditionally `Send`/`Sync`?

**Interview Answer**

Use `PhantomData<*const ()>` to opt out of auto traits. For conditional implementations, use trait bounds: `struct Wrapper<T> { inner: T }` is `Send` when `T: Send`. For explicit control, use `unsafe impl Send for MyType {}` after verifying thread safety manually.

---

### Q6. What are the common mistakes with `Send` and `Sync`?

**Interview Answer**

The most common mistake is using `Rc` or `Cell` in shared state, causing compile errors with `tokio::spawn`. Another is assuming `Arc<T>` is `Send` without `T: Send + Sync`. Also, `MutexGuard` is `Send` but not `Sync`, which surprises developers trying to share guards across threads.

---

### Q7. How do `Send` and `Sync` affect FFI?

**Interview Answer**

Types crossing FFI boundaries must be `Send` if sent to another thread. Raw pointers (`*const T`, `*mut T`) are `!Send` and `!Sync` by default because the compiler can't verify their thread safety. Use `unsafe impl Send/Sync` after ensuring the foreign code handles threading correctly.

---

### Q8. What is auto-trait behavior for `Send` and `Sync`?

**Interview Answer**

`Send` and `Sync` are auto-traits, meaning the compiler automatically implements them for types whose fields are all `Send`/`Sync`. Enums, structs, and tuples derive these traits automatically. You can opt out with `PhantomData<*const ()>` for `!Send` or `PhantomData<*mut ()>` for `!Send` and `!Sync`.

---

### Q9. How do you test if a type implements `Send`/`Sync`?

**Interview Answer**

Use compile-time assertions: `fn assert_send<T: Send>() {}` and call it with your type. For runtime checks, `std::any::TypeId::of::<T>()` can verify types but not traits directly. The `static_assertions` crate provides `assert_send!` and `assert_sync!` macros for cleaner compile-time checks.

---

### Q10. How do `Send`/`Sync` interact with async traits?

**Interview Answer**

Async trait methods (via `async-trait` or native `-> impl Future`) must return futures that are `Send` if the trait object is used across threads. This means the future's state machine can only contain `Send` types. Non-`Send` futures must use `spawn_local` or run on a `LocalSet`.

---
