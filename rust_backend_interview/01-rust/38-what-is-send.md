# What is Send?

## Interview Question

What is Send?

## Interview Answer

"Send means ownership of a type can safely move between threads."

---

## Follow-up Questions & Answers

### Q1. Which types are automatically `Send`?

**Interview Answer**

Most primitive types (`i32`, `bool`, `String`, `Vec<T>`) are `Send`. Types composed entirely of `Send` fields are auto-`Send`. Non-`Send` types include `Rc<T>`, raw pointers (`*const T`), and `Cell<T>`. The compiler auto-derives `Send` via the orphan rules, and you can check with `fn assert_send<T: Send>() {}`.

---

### Q2. Is `Send` related to `Copy`?

**Interview Answer**

No, `Send` is about thread safety while `Copy` is about bitwise copying. `i32` is both `Send` and `Copy`, but `String` is `Send` and not `Copy`. `Rc<T>` is neither. `Send` requires that moving the type across threads doesn't create data races, while `Copy` means the value can be duplicated without a move.

---

### Q3. Can you implement `Send` for a custom type?

**Interview Answer**

You can use `unsafe impl Send for MyType {}` but only if you guarantee the type contains no non-thread-safe data. If your struct wraps raw pointers, `Rc`, or `Cell`, implementing `Send` unsafely can cause undefined behavior. Always verify by ensuring all fields are either `Send` or properly synchronized.

---

### Q4. How does `Arc<Mutex<T>>` relate to `Send`?

**Interview Answer**

`Arc<T>` is `Send` when `T: Send + Sync`, and `Mutex<T>` is both `Send` and `Sync` when `T: Send`. So `Arc<Mutex<T>>` is `Send` for any `T: Send`, allowing you to move the shared mutable state into another thread safely. This is the fundamental pattern for cross-thread shared state in Rust.

---

### Q5. What error do you get if a type is not `Send`?

**Interview Answer**

You get a compile error like "future cannot be sent between threads safely: the type `Rc<RefCell<Vec<u8>>>` is not `Send`." This typically occurs when using `tokio::spawn` which requires the future to be `Send`. The error message names the exact non-`Send` type so you can fix it, often by switching to `Arc` or `Arc<Mutex<T>>`.

---

### Q6. Is `&mut T` `Send`?

**Interview Answer**

Yes, `&mut T` is `Send` when `T: Send`. This is safe because sending a mutable reference transfers exclusive access to the receiver, so no data race can occur. However, `&T` requires `T: Sync` to be `Send`, since multiple threads could hold shared references simultaneously.

---

### Q7. How does `Send` interact with async runtimes like Tokio?

**Interview Answer**

`tokio::spawn` requires the future to be `Send` because Tokio may move the task between worker threads. If your future holds a non-`Send` type like `Rc`, the compiler rejects it. You can use `tokio::task::spawn_local` for non-`Send` futures, but they must run on a single-threaded `LocalSet`.

---

### Q8. What is `!Send` and when would you want a non-`Send` type?

**Interview Answer**

`!Send` means a type cannot be safely moved to another thread. Types like `Rc<T>` or `Cell<T>` are `!Send` by design because they're optimized for single-threaded use. You might intentionally use `!Send` types in performance-critical single-threaded code where atomic operations would be wasteful.

---

### Q9. Can a `Send` type contain a `!Send` type?

**Interview Answer**

No, a struct containing a `!Send` field is automatically `!Send` unless you use `unsafe impl Send` to override this. For example, `struct Foo { data: Rc<i32> }` is not `Send`. To make it `Send`, you'd need to wrap the `Rc` in a `Mutex` or replace it with `Arc`.

---

### Q10. How does `Send` affect `tokio::sync::mpsc` channel usage?

**Interview Answer**

Both the `Sender` and `Receiver` of an `mpsc` channel are `Send` when the message type `T: Send`. This allows you to clone the `Sender` and distribute it across async tasks. If `T` is `!Send`, you can't cross thread boundaries with it, so you'd need to use `spawn_local` or restructure to avoid sending non-thread-safe types through channels.

---
