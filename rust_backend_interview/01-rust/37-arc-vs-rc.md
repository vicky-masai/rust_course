# Arc vs Rc

## Interview Question

Arc vs Rc.

## Interview Answer

"Rc is for single-threaded programs. Arc is thread-safe and used across multiple threads."

---

## Follow-up Questions & Answers

### Q1. Why can't `Rc` be used across threads?

**Interview Answer**

`Rc` uses non-atomic reference count operations (`Cell<usize>`) which are not safe under concurrent access. If two threads simultaneously increment the count, a data race occurs causing undefined behavior. `Rc` is deliberately not `Send` or `Sync` to prevent this at compile time.

---

### Q2. Is there a runtime cost to using `Arc` over `Rc`?

**Interview Answer**

Yes, atomic operations on `Arc` cost roughly 5-20 nanoseconds per clone or drop due to memory fence instructions. On a single-threaded program, `Rc` avoids this overhead entirely. In hot paths with millions of reference count changes, this difference can be measurable, though in most backend code it's negligible.

---

### Q3. Can you use `Weak<T>` with both `Rc` and `Arc`?

**Interview Answer**

Yes, both `Rc` and `Arc` have corresponding `Weak` variants (`Rc::downgrade` and `Arc::downgrade`). The semantics are identical—weak references don't prevent deallocation, and `upgrade()` returns `None` if all strong references are gone. This is useful for breaking reference cycles in graph structures or caches.

---

### Q4. What happens if you try to send an `Rc` across threads?

**Interview Answer**

The compiler rejects it because `Rc` does not implement `Send`. You'll get a compile error like "Rc cannot be sent between threads safely." The fix is to switch to `Arc`, which implements both `Send` and `Sync`. This compile-time check prevents data races on the reference count.

---

### Q5. How do you choose between `Arc<Mutex<T>>` and `Arc<RwLock<T>>`?

**Interview Answer**

Use `Mutex` when writes are frequent or contention is low, as it has lower overhead per acquisition. Use `RwLock` when reads vastly outnumber writes, since multiple readers can hold the lock simultaneously. In Rust, `RwLock` is not re-entrant, so be careful about calling read methods while already holding a read guard.

---

### Q6. Can `Rc<T>` be upgraded to `Arc<T>` later?

**Interview Answer**

Not directly—you'd need to refactor the codebase to use `Arc` everywhere. The APIs are nearly identical, so the migration is mostly mechanical, but it increases atomic overhead. A common strategy is to start with `Arc` from the beginning if there's any chance the code will need thread safety in the future.

---

### Q7. Are there non-atomic reference-counted types besides `Rc`?

**Interview Answer**

Yes, `triomphe::ThinArc` and `servo_arc::Arc` provide alternative implementations. In the standard library, `Rc` is the only non-atomic `Rc`. Custom implementations can use `Cell<usize>` for performance in single-threaded scenarios where `Rc`'s overhead matters, such as parser ASTs or tree structures.

---

### Q8. How does `Rc::clone` differ from `T::clone`?

**Interview Answer**

`Rc::clone` only increments the reference count—it does not clone the inner data. `(*rc).clone()` or `Rc::unwrap_or_clone` actually clones the underlying `T`. This distinction is critical because people often assume `let b = a.clone()` on an `Rc` gives them a deep copy, but it only gives another pointer to the same data.

---

### Q9. What is `Rc::make_mut` and when is it useful?

**Interview Answer**

`Rc::make_mut` provides copy-on-write semantics: if the reference count is one, it returns a mutable reference directly; otherwise it clones the data, decrements the old count, and returns a mutable reference to the new copy. This is useful for efficiently building up data structures that are mostly shared but occasionally modified.

---

### Q10. In what scenarios would you use both `Rc` and `Arc` in the same project?

**Interview Answer**

In a backend service, `Arc` is used for data shared across async tasks (like DB pools), while `Rc` might be used internally within a single-threaded component like a parser or AST builder. The key rule is: `Rc` for internal data structures within one thread, `Arc` for anything crossing task or thread boundaries.

---
