# Arc

## Interview Question

Explain Arc.

## Interview Answer

"Arc provides thread-safe shared ownership using atomic reference counting."

---

## Follow-up Questions & Answers

### Q1. What makes `Arc` different from `Rc` at the implementation level?

**Interview Answer**

`Arc` uses atomic operations (`AtomicUsize`) for reference count increments and decrements, which are safe across threads but carry a small CPU cost due to memory fences. `Rc` uses non-atomic `Cell<usize>` operations, which are faster but UB if used across threads. This is why `Arc` implements `Send` and `Sync` while `Rc` does not.

---

### Q2. Is `Arc<T>` itself mutable?

**Interview Answer**

No, `Arc<T>` provides only shared `&T` references. To get interior mutability, you combine it with `Mutex<T>`, `RwLock<T>`, or `AtomicXxx`. The common pattern is `Arc<Mutex<T>>` where the mutex protects mutable state while `Arc` provides shared ownership across threads.

---

### Q3. What is the overhead of cloning an `Arc`?

**Interview Answer**

Cloning an `Arc` performs a single atomic increment on the reference count, which on modern x86 hardware costs roughly 5-20 nanoseconds depending on contention. The actual data is never copied. When the last `Arc` is dropped, the atomic decrement triggers deallocation of both the `Arc` and the heap-allocated `T`.

---

### Q4. Can you use `Arc<str>` and why is it useful?

**Interview Answer**

`Arc<str>` is a thin pointer to a string slice stored on the heap, which avoids the extra allocation that `Arc<String>` would require. It's particularly useful in backends for sharing immutable string data like configuration values or route patterns across handler closures and threads without repeated cloning of the underlying bytes.

---

### Q5. What happens when you move an `Arc` out of a function's return?

**Interview Answer**

Moving an `Arc` transfers ownership without touching the reference count—it's just a pointer copy on the stack. The count only changes when you explicitly clone or drop. This means passing an `Arc` by value into a function is cheap and the compiler ensures the original binding is no longer usable afterward.

---

### Q6. How does `Arc` interact with `Weak<T>`?

**Interview Answer**

`Arc` provides `downgrade()` to create a `Weak<T>` reference that doesn't prevent deallocation. When all strong references drop, the data is freed even if weak references remain. Calling `Weak::upgrade()` returns `Some(Arc<T>)` if the data is still alive, otherwise `None`. This is useful for cache implementations to avoid reference cycles.

---

### Q7. Why can't `Arc<Mutex<T>>` be `Clone` by default?

**Interview Answer**

Rust's `Clone` derive on a struct clones all fields. For `Arc<Mutex<T>>`, this would just clone the pointer, not the inner data. While this is intentional for shared ownership, custom `Clone` implementations sometimes clone the inner data under the lock, which can be surprising and expensive. The standard approach is to share the `Arc` and clone the pointer.

---

### Q8. Is `Arc` suitable for single-threaded use cases?

**Interview Answer**

`Arc` works in single-threaded code but is overkill—`Rc` is faster because it skips atomic operations. Use `Rc` when you know data never crosses thread boundaries. If you later need thread safety, switching to `Arc` is straightforward since the API is identical, but the performance difference matters in hot loops.

---

### Q9. How do you convert `Arc<T>` to `T` if `T: Clone`?

**Interview Answer**

You can use `Arc::try_unwrap(arc)` which returns `Ok(T)` if the reference count is exactly one (meaning you have exclusive ownership), or `Err(Arc<T>)` otherwise. Alternatively, `(*arc).clone()` gives you a cloned `T` without consuming the `Arc`. `try_unwrap` is preferred when you want to reclaim the heap allocation.

---

### Q10. How is `Arc` used in Tokio for shared application state?

**Interview Answer**

In Axum or Actix, application state like database pools or config is wrapped in `Arc` and cloned into each handler's closure. For mutable state, `Arc<RwLock<T>>` or `Arc<AtomicXxx>` is used. The pattern `Extension(Arc<AppState>)` in Axum clones the `Arc` pointer cheaply per request, giving each handler shared access without data copying.

---
